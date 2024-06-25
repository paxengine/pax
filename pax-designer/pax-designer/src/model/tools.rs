use std::any::Any;
use std::ops::ControlFlow;
use std::rc::Rc;

use super::action::orm::{CreateComponent, SetNodeTransformProperties};
use super::action::pointer::Pointer;
use super::action::{Action, ActionContext, CanUndo, RaycastMode};
use super::input::InputEvent;
use super::{RuntimeNodeInfo, SelectionStateSnapshot, StageInfo};
use crate::glass::RectTool;
use crate::math::coordinate_spaces::{Glass, World};
use crate::math::{
    AxisAlignedBox, GetUnit, IntoInversionConfiguration, InversionConfiguration, SizeUnit,
};
use crate::model::action::orm::group_ungroup::MoveNodeKeepScreenPos;
use crate::model::Tool;
use crate::model::{AppState, ToolBehaviour};
use crate::{SetStage, ROOT_PROJECT_ID};
use anyhow::{anyhow, Result};
use pax_designtime::DesigntimeManager;
use pax_engine::api::Color;
use pax_engine::api::Size;
use pax_engine::layout::{LayoutProperties, TransformAndBounds};
use pax_engine::math::Point2;
use pax_engine::math::Vector2;
use pax_engine::{log, NodeLocal};
use pax_manifest::{PaxType, TemplateNodeId, TypeId, UniqueTemplateNodeIdentifier};
use pax_runtime_api::math::Transform2;
use pax_runtime_api::{Axis, Window};

pub struct CreateComponentTool {
    type_id: TypeId,
    origin: Point2<Glass>,
    bounds: AxisAlignedBox,
}

impl CreateComponentTool {
    pub fn new(_ctx: &mut ActionContext, point: Point2<Glass>, type_id: &TypeId) -> Self {
        Self {
            type_id: type_id.clone(),
            origin: point,
            bounds: AxisAlignedBox::new(Point2::default(), Point2::default()),
        }
    }
}

impl ToolBehaviour for CreateComponentTool {
    fn pointer_down(&mut self, _point: Point2<Glass>, _ctx: &mut ActionContext) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn pointer_move(
        &mut self,
        point: Point2<Glass>,
        ctx: &mut ActionContext,
    ) -> std::ops::ControlFlow<()> {
        let is_shift_key_down = ctx
            .app_state
            .keys_pressed
            .get()
            .contains(&InputEvent::Shift);
        let is_alt_key_down = ctx.app_state.keys_pressed.get().contains(&InputEvent::Alt);
        self.bounds = AxisAlignedBox::new(self.origin, self.origin + Vector2::new(1.0, 1.0))
            .morph_constrained(point, self.origin, is_alt_key_down, is_shift_key_down);
        ControlFlow::Continue(())
    }

    fn pointer_up(&mut self, point: Point2<Glass>, ctx: &mut ActionContext) -> ControlFlow<()> {
        self.pointer_move(point, ctx);
        let box_transform = ctx.world_transform() * self.bounds.as_transform();
        let (o, u, v) = box_transform.decompose();
        // TODO make CreateComponent take transform?
        let world_box = AxisAlignedBox::new(o, o + u + v);
        ctx.execute(CreateComponent {
            bounds: world_box,
            type_id: self.type_id.clone(),
            custom_props: vec![],
        })
        .unwrap();
        ControlFlow::Break(())
    }

    fn keyboard(
        &mut self,
        _event: crate::model::input::InputEvent,
        _dir: crate::model::input::Dir,
        _ctx: &mut ActionContext,
    ) -> std::ops::ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visualize(&self, glass: &mut crate::glass::Glass) {
        glass.is_rect_tool_active.set(true);
        glass.rect_tool.set(RectTool {
            x: Size::Pixels(self.bounds.top_left().x.into()),
            y: Size::Pixels(self.bounds.top_left().y.into()),
            width: Size::Pixels(self.bounds.width().into()),
            height: Size::Pixels(self.bounds.height().into()),
            stroke: Color::rgba(0.into(), 0.into(), 255.into(), 200.into()),
            fill: Color::rgba(0.into(), 0.into(), 255.into(), 30.into()),
        });
    }
}

pub struct SelectNodes<'a> {
    pub ids: &'a [TemplateNodeId],
    //if true, deselects all other objects first
    pub overwrite: bool,
}

impl Action for SelectNodes<'_> {
    fn perform(self: Box<Self>, ctx: &mut ActionContext) -> Result<CanUndo> {
        let mut ids = ctx.app_state.selected_template_node_ids.get();
        // TODO this is not it, should instead not trigger selectnodes if
        // clicking on group of nodes that is already selected and was moved
        if self.overwrite
            || !ctx
                .app_state
                .keys_pressed
                .get()
                .contains(&InputEvent::Shift)
        {
            ids.clear();
        }
        ids.extend_from_slice(self.ids);
        // Only set if changed, otherwise re-triggers when same object gets re-selected
        if ids != ctx.app_state.selected_template_node_ids.get() {
            ctx.app_state.selected_template_node_ids.set(ids);
        }
        Ok(CanUndo::No)
    }
}

pub struct PointerTool {
    action: PointerToolAction,
}

pub enum PointerToolAction {
    Moving {
        has_moved: bool,
        hit: UniqueTemplateNodeIdentifier,
        pickup_point: Point2<Glass>,
        initial_selection: SelectionStateSnapshot,
    },
    Selecting {
        p1: Point2<Glass>,
        p2: Point2<Glass>,
    },
    ResizingStage(ResizeStageDim),
}

pub enum ResizeStageDim {
    Height,
    Width,
}

impl PointerTool {
    pub fn new(ctx: &mut ActionContext, point: Point2<Glass>) -> Self {
        if let Some(hit) = ctx.raycast_glass(point, RaycastMode::Top) {
            let node_id = hit.global_id().unwrap();
            let selected = ctx.selection_state();
            if !selected.items.iter().any(|s| s.id == node_id) {
                let _ = ctx.execute(SelectNodes {
                    ids: &[node_id.get_template_node_id()],
                    overwrite: false,
                });
            }
            let selection = ctx.selection_state();
            Self {
                action: PointerToolAction::Moving {
                    hit: node_id.clone(),
                    has_moved: false,
                    pickup_point: point,
                    initial_selection: (&selection).into(),
                },
            }
        } else {
            // resize stage if we are at edge
            let stage = ctx.derived_state.stage.get();
            let world_point = ctx.world_transform() * point;
            if (world_point.y - stage.height as f64).abs() < 10.0 {
                Self {
                    action: PointerToolAction::ResizingStage(ResizeStageDim::Height),
                }
            } else if (world_point.x - stage.width as f64).abs() < 10.0 {
                Self {
                    action: PointerToolAction::ResizingStage(ResizeStageDim::Width),
                }
            } else {
                Self {
                    action: PointerToolAction::Selecting {
                        p1: point,
                        p2: point,
                    },
                }
            }
        }
    }
}

impl ToolBehaviour for PointerTool {
    fn pointer_down(&mut self, _point: Point2<Glass>, _ctx: &mut ActionContext) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn pointer_move(&mut self, point: Point2<Glass>, ctx: &mut ActionContext) -> ControlFlow<()> {
        match &mut self.action {
            &mut PointerToolAction::Moving {
                pickup_point,
                ref initial_selection,
                ref mut has_moved,
                ..
            } => {
                if (pickup_point - point).length_squared() < 3.0 {
                    // don't commit any movement for very small pixel changes,
                    // this creates designtime changes that
                    // make double click behavior for for example
                    // text editing not work
                    return ControlFlow::Continue(());
                }
                *has_moved = true;
                let translation = point - pickup_point;

                let move_translation = TransformAndBounds {
                    transform: Transform2::translate(translation),
                    bounds: (1.0, 1.0),
                };

                for item in &initial_selection.items {
                    if let Err(e) = ctx.execute(SetNodeTransformProperties {
                        id: item.id.clone(),
                        transform_and_bounds: move_translation * item.transform_and_bounds,
                        parent_transform_and_bounds: item.parent_transform_and_bounds,
                        inv_config: item.layout_properties.into_inv_config(),
                    }) {
                        pax_engine::log::error!("Error moving selected: {:?}", e);
                    }
                }
            }
            PointerToolAction::Selecting { ref mut p2, .. } => *p2 = point,
            PointerToolAction::ResizingStage(dir) => {
                let world_point = ctx.world_transform() * point;
                let size_before = ctx.derived_state.stage.get();
                let (new_width, new_height) = match dir {
                    ResizeStageDim::Height => (size_before.width, world_point.y as u32),
                    ResizeStageDim::Width => (world_point.x as u32, size_before.height),
                };
                ctx.execute(SetStage(StageInfo {
                    width: new_width,
                    height: new_height,
                    color: Color::BLUE,
                }))
                .unwrap();
            }
        }
        ControlFlow::Continue(())
    }

    fn pointer_up(&mut self, point: Point2<Glass>, ctx: &mut ActionContext) -> ControlFlow<()> {
        // move last little distance to pointer up position
        self.pointer_move(point, ctx);

        match &self.action {
            PointerToolAction::Selecting { .. } => {
                // TODO select multiple objects
                let _ = ctx.execute(SelectNodes {
                    ids: &[],
                    overwrite: false,
                });
            }
            PointerToolAction::Moving {
                has_moved, ref hit, ..
            } => {
                if *has_moved {
                    if let Some(mut drop_hit) = ctx.raycast_glass(point, RaycastMode::Nth(1)) {
                        log::debug!("original hit: {:?}", hit);
                        let drop_slot_object = loop {
                            if drop_hit
                                .parent()
                                .is_some_and(|n| n.is_of_type::<pax_engine::Slot>())
                            {
                                break Some(hit);
                            }
                            if let Some(par) = drop_hit.parent() {
                                drop_hit = par;
                            } else {
                                break None;
                            };
                        };
                        if let Some(drop_slot) = drop_slot_object {
                            if let Err(e) = ctx.execute(MoveNodeKeepScreenPos {
                                node: drop_slot,
                                new_parent: hit,
                                index: pax_manifest::TreeIndexPosition::Top,
                            }) {
                                log::warn!("failed to swap nodes: {}", e);
                            };
                        }
                        log::debug!("thing to replace: {:?}", drop_slot_object);
                    }
                } else {
                    let _ = ctx.execute(SelectNodes {
                        ids: &[hit.get_template_node_id()],
                        overwrite: false,
                    });
                }
            }
            PointerToolAction::ResizingStage(_dir) => {}
        }
        ControlFlow::Break(())
    }

    fn keyboard(
        &mut self,
        _event: crate::model::input::InputEvent,
        _dir: crate::model::input::Dir,
        _ctx: &mut ActionContext,
    ) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn visualize(&self, glass: &mut crate::glass::Glass) {
        if let PointerToolAction::Selecting { p1, p2 } = self.action {
            glass.is_rect_tool_active.set(true);
            glass.rect_tool.set(RectTool {
                x: Size::Pixels(p1.x.into()),
                y: Size::Pixels(p1.y.into()),
                width: Size::Pixels((p2.x - p1.x).into()),
                height: Size::Pixels((p2.y - p1.y).into()),
                stroke: Color::rgba(0.into(), 255.into(), 255.into(), 200.into()),
                fill: Color::rgba(0.into(), 255.into(), 255.into(), 30.into()),
            });
        }
    }
}
