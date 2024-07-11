use std::{any::Any, cell::RefCell, ops::ControlFlow, rc::Rc};

use pax_engine::{
    api::NodeContext,
    layout::TransformAndBounds,
    log,
    math::{Point2, Transform2},
    NodeInterface, Property, Slot,
};
use pax_manifest::UniqueTemplateNodeIdentifier;
use pax_runtime_api::{borrow, borrow_mut, Color};
use pax_std::stacker::Stacker;

use crate::{
    glass::{
        control_point::{ControlPointBehaviourFactory, ControlPointStyling},
        outline::PathOutline,
        wireframe_editor::editor_generation::CPoint,
        ToolVisualizationState,
    },
    math::{coordinate_spaces::Glass, DecompositionConfiguration, IntoDecompositionConfiguration},
    model::{
        self,
        action::{
            orm::{MoveNode, NodeLayoutSettings, SetNodeLayout, SetNodePropertiesFromTransform},
            Action, ActionContext, RaycastMode,
        },
        input::InputEvent,
        GlassNode, GlassNodeSnapshot, ToolBehaviour,
    },
    ROOT_PROJECT_ID,
};

use super::ControlPointSet;

pub fn stacker_control_set(ctx: NodeContext, item: GlassNode) -> Vec<Property<ControlPointSet>> {
    struct StackerBehaviour {
        initial_node: GlassNodeSnapshot,
        pickup_point: Point2<Glass>,
        before_move_undo_id: usize,
        vis: Property<ToolVisualizationState>,
    }

    let to_glass_transform =
        model::read_app_state_with_derived(|_, derived| derived.to_glass_transform.get());

    // re-do this whenever slots change
    let stacker_node = ctx
        .get_nodes_by_global_id(item.id)
        .into_iter()
        .next()
        .unwrap();
    let slot_count = stacker_node.flattened_slot_children_count();

    impl ToolBehaviour for StackerBehaviour {
        fn pointer_down(
            &mut self,
            _point: Point2<Glass>,
            _ctx: &mut ActionContext,
        ) -> ControlFlow<()> {
            ControlFlow::Break(())
        }

        fn pointer_move(
            &mut self,
            point: Point2<Glass>,
            ctx: &mut ActionContext,
        ) -> ControlFlow<()> {
            let translation = point - self.pickup_point;

            let curr_node = ctx
                .engine_context
                .get_nodes_by_global_id(self.initial_node.id.clone())
                .into_iter()
                .next()
                .unwrap();
            let glass_curr_node = GlassNode::new(&curr_node, &ctx.glass_transform());

            let curr_container = curr_node.template_parent().unwrap();
            let glass_curr_container = GlassNode::new(&curr_container, &ctx.glass_transform());

            let move_translation = TransformAndBounds {
                transform: Transform2::translate(translation),
                bounds: (1.0, 1.0),
            };

            if let Err(e) = (SetNodePropertiesFromTransform {
                id: &self.initial_node.id,
                transform_and_bounds: &(move_translation * self.initial_node.transform_and_bounds),
                parent_transform_and_bounds: &glass_curr_node.parent_transform_and_bounds.get(),
                decomposition_config: &self
                    .initial_node
                    .layout_properties
                    .into_decomposition_config(),
            }
            .perform(ctx))
            {
                pax_engine::log::error!("Error moving stacker object: {:?}", e);
            }

            if let Some((container, slot_hit)) = raycast_slot(ctx, point, curr_node.clone(), false)
            {
                let curr_slot = curr_node.render_parent().unwrap();

                let new_index =
                    slot_hit.with_properties(|f: &mut Slot| f.index.get().to_int() as usize);
                let old_index =
                    curr_slot.with_properties(|f: &mut Slot| f.index.get().to_int() as usize);

                if curr_slot == slot_hit && new_index == old_index {
                    return ControlFlow::Continue(());
                }

                let glass_slot_hit = GlassNode::new(&slot_hit, &ctx.glass_transform());
                if let Err(e) = (MoveNode {
                    node_id: &self.initial_node.id,
                    new_parent_uid: &container.global_id().unwrap(),
                    index: pax_manifest::TreeIndexPosition::At(new_index.unwrap()),
                    // TODO try to make this the "future calculated position
                    // after ORM updates" instead. might be possible to
                    // subscribe to manifest changes, and update the bounds
                    // whenever that happens, but still takes 2 ticks (manifest
                    // update -> recalc bounds -> manifest update). Leave as
                    // this for now as the problem is only visible from first
                    // movement over a new slot to the next mouse-move op WHEN
                    // the slot bounds will change as a consequence of movement.
                    node_layout: NodeLayoutSettings::KeepScreenBounds {
                        node_transform_and_bounds: &(move_translation
                            * self.initial_node.transform_and_bounds),
                        new_parent_transform_and_bounds: &glass_slot_hit.transform_and_bounds.get(),
                        node_decompositon_config: self
                            .initial_node
                            .layout_properties
                            .into_decomposition_config(),
                    },
                }
                .perform(ctx))
                {
                    log::warn!("failed to swap nodes: {}", e);
                };
            } else if !glass_curr_container
                .transform_and_bounds
                .get()
                .contains_point(point)
                && ctx
                    .engine_context
                    .get_nodes_by_id(ROOT_PROJECT_ID)
                    .into_iter()
                    .next()
                    .unwrap()
                    != curr_container
            {
                let container_parent = curr_container.template_parent().unwrap();
                let container_parent = GlassNode::new(&container_parent, &ctx.glass_transform());
                log::debug!("move out!");
                if let Err(e) = (MoveNode {
                    node_id: &self.initial_node.id,
                    new_parent_uid: &container_parent.id,
                    index: pax_manifest::TreeIndexPosition::Top,
                    node_layout: NodeLayoutSettings::KeepScreenBounds {
                        node_transform_and_bounds: &(move_translation
                            * self.initial_node.transform_and_bounds),
                        new_parent_transform_and_bounds: &container_parent
                            .transform_and_bounds
                            .get(),
                        node_decompositon_config: self
                            .initial_node
                            .layout_properties
                            .into_decomposition_config(),
                    },
                }
                .perform(ctx))
                {
                    log::warn!("failed to swap nodes: {}", e);
                };
            }
            ControlFlow::Continue(())
        }

        fn pointer_up(
            &mut self,
            _point: Point2<Glass>,
            ctx: &mut ActionContext,
        ) -> ControlFlow<()> {
            let curr_node = ctx
                .engine_context
                .get_nodes_by_global_id(self.initial_node.id.clone())
                .into_iter()
                .next()
                .unwrap();
            if curr_node.render_parent().unwrap().is_of_type::<Slot>() {
                log::debug!("parent is slot!");
                if let Err(e) = (SetNodeLayout {
                    id: &self.initial_node.id,
                    node_layout: &NodeLayoutSettings::Fill::<Glass>,
                }
                .perform(ctx))
                {
                    log::warn!("failed: {e}")
                }
            } else {
            }
            ControlFlow::Break(())
        }

        fn keyboard(
            &mut self,
            _event: InputEvent,
            _dir: crate::model::input::Dir,
            _ctx: &mut ActionContext,
        ) -> ControlFlow<()> {
            ControlFlow::Break(())
        }

        fn get_visual(&self) -> Property<crate::glass::ToolVisualizationState> {
            let vis = self.vis.clone();
            let deps = [vis.untyped()];
            Property::computed(move || vis.get(), &deps)
        }
    }

    fn stacker_control_point_factory(slot_child: GlassNode) -> ControlPointBehaviourFactory {
        Rc::new(move |ctx, p| {
            let dt = borrow!(ctx.engine_context.designtime);
            let before_move_undo_id = dt.get_orm().get_last_undo_id().unwrap_or(0);

            // set visualization outline to always be the bounds of the parent of the moving node
            let manifest_ver = dt.get_orm().get_manifest_version();
            let glass_transform = ctx.glass_transform();
            let slot_child_index = slot_child.id.clone();
            let deps = [glass_transform.untyped(), manifest_ver.untyped()];
            let ctx = ctx.engine_context.clone();
            let vis = Property::computed(
                move || {
                    let slot_child_parent = ctx
                        .get_nodes_by_global_id(slot_child_index.clone())
                        .into_iter()
                        .next()
                        .unwrap()
                        .render_parent()
                        .unwrap();
                    let slot_child_parent = GlassNode::new(&slot_child_parent, &glass_transform);
                    let outline =
                        PathOutline::from_bounds(slot_child_parent.transform_and_bounds.get());
                    ToolVisualizationState {
                        rect_tool: Default::default(),
                        outline,
                    }
                },
                &deps,
            );

            Rc::new(RefCell::new(StackerBehaviour {
                initial_node: (&slot_child).into(),
                pickup_point: p,
                before_move_undo_id,
                vis,
            }))
        })
    }

    let control_point_styling = ControlPointStyling {
        round: true,
        stroke: Color::RED,
        fill: Color::rgba(255.into(), 255.into(), 255.into(), 150.into()),
        stroke_width_pixels: 2.0,
        size_pixels: 15.0,
    };

    let t_and_b = stacker_node.transform_and_bounds();
    let deps = [t_and_b.untyped(), slot_count.untyped()];
    vec![Property::computed(
        move || {
            let mut slots = vec![];
            let mut search_set: Vec<NodeInterface> = stacker_node.children();
            while let Some(node) = search_set.pop() {
                for n in node.children() {
                    if n.is_of_type::<Slot>() {
                        slots.push(n)
                    } else {
                        search_set.push(n)
                    }
                }
            }
            let to_glass = to_glass_transform.clone();
            let stacker_control_points = slots
                .into_iter()
                .map(|s| {
                    let slot_child = s.children().into_iter().next().unwrap();
                    let slot_child = GlassNode::new(&slot_child, &to_glass);
                    let t_and_b = slot_child.transform_and_bounds.get();
                    CPoint::new(t_and_b.center(), stacker_control_point_factory(slot_child))
                })
                .collect();
            ControlPointSet {
                points: stacker_control_points,
                styling: control_point_styling.clone(),
            }
        },
        &deps,
    )]
}

pub fn raycast_slot(
    ctx: &ActionContext,
    point: Point2<Glass>,
    original_hit: NodeInterface,
    prevent_move_self: bool,
) -> Option<(NodeInterface, NodeInterface)> {
    // If we drop on another object, check if it's an object in a slot.
    // If it is, add this object to the same parent
    let drop_hit = ctx.raycast_glass(point, RaycastMode::RawNth(0), &[original_hit.clone()])?;
    let mut drop_slot_container = drop_hit.clone();
    let drop_slot_topmost_container = loop {
        if drop_slot_container
            .containing_component()
            .is_some_and(|v| v.is_of_type::<Stacker>())
        {
            break Some(drop_slot_container);
        }
        if let Some(par) = drop_slot_container.render_parent() {
            drop_slot_container = par;
        } else {
            break None;
        };
    };
    let drop_container = drop_slot_topmost_container?;
    let mut slot = None;
    let mut curr = drop_hit.clone();
    let cc = drop_container.containing_component().unwrap();
    while curr.is_descendant_of(&cc) {
        if curr.is_of_type::<Slot>() {
            slot = Some(curr.clone());
        }
        curr = curr.render_parent().unwrap();
    }
    if prevent_move_self && original_hit.is_descendant_of(&cc) {
        return None;
    };
    Some((cc, slot.unwrap()))
}
