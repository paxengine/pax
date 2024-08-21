use pax_message::{AnyCreatePatch, TextboxPatch};
use pax_runtime::api as pax_runtime_api;
use pax_runtime::api::{use_RefCell, Layer, Property};
use pax_runtime::{
    BaseInstance, ExpandedNode, ExpandedNodeIdentifier, InstanceFlags, InstanceNode,
    InstantiationArgs, RuntimeContext,
};
use_RefCell!();
use crate::*;
use pax_engine::pax;
use pax_runtime_api::*;
use std::collections::HashMap;
use std::rc::Rc;

use crate::common::patch_if_needed;

/// A platform-native text input field
#[pax]
#[engine_import_path("pax_engine")]
#[primitive("pax_std::forms::textbox::TextboxInstance")]
#[custom(Default)]
pub struct Textbox {
    pub text: Property<String>,
    pub background: Property<Color>,
    pub stroke: Property<Stroke>,
    pub border_radius: Property<Numeric>,
    pub style: Property<TextStyle>,
    pub focus_on_mount: Property<bool>,
}

impl Default for Textbox {
    fn default() -> Self {
        Self {
            text: Default::default(),
            background: Property::new(Color::rgb(249.into(), 250.into(), 251.into())),
            stroke: Property::new(Stroke {
                color: Property::new(Color::rgb(209.into(), 213.into(), 219.into())),
                width: Property::new(Size::Pixels(1.into())),
            }),
            border_radius: Property::new(8.0.into()),
            style: Property::new(TextStyle {
                font: Property::new(Font::default()),
                font_size: Property::new(Size::Pixels(Numeric::F64(14.0))),
                fill: Property::new(Color::BLACK),
                underline: Property::new(false),
                align_horizontal: Property::new(TextAlignHorizontal::Left),
                align_multiline: Property::new(TextAlignHorizontal::Left),
                align_vertical: Property::new(TextAlignVertical::Center),
            }),
            focus_on_mount: Property::new(false),
        }
    }
}
pub struct TextboxInstance {
    base: BaseInstance,
    native_message_props: RefCell<HashMap<ExpandedNodeIdentifier, Property<()>>>,
}

impl InstanceNode for TextboxInstance {
    fn instantiate(args: InstantiationArgs) -> Rc<Self>
    where
        Self: Sized,
    {
        Rc::new(Self {
            base: BaseInstance::new(
                args,
                InstanceFlags {
                    invisible_to_slot: false,
                    invisible_to_raycasting: false,
                    layer: Layer::Native,
                    is_component: false,
                },
            ),
            native_message_props: Default::default(),
        })
    }

    fn update(self: Rc<Self>, expanded_node: &Rc<ExpandedNode>, _context: &Rc<RuntimeContext>) {
        //trigger computation of property that computes + sends native message update
        borrow!(self.native_message_props)
            .get(&expanded_node.id)
            .unwrap()
            .get();
    }

    fn handle_mount(
        self: Rc<Self>,
        expanded_node: &Rc<ExpandedNode>,
        context: &Rc<RuntimeContext>,
    ) {
        // Send creation message
        let id = expanded_node.id.clone();
        context.enqueue_native_message(pax_message::NativeMessage::TextboxCreate(AnyCreatePatch {
            id: id.to_u32(),
            parent_frame: expanded_node.parent_frame.get().map(|v| v.to_u32()),
            occlusion_layer_id: 0,
        }));

        // send update message when relevant properties change
        let weak_self_ref = Rc::downgrade(&expanded_node);
        let context = Rc::clone(context);
        let last_patch = Rc::new(RefCell::new(TextboxPatch {
            id: id.to_u32(),
            ..Default::default()
        }));

        let deps: Vec<_> = borrow_mut!(expanded_node.properties_scope)
            .values()
            .cloned()
            .map(|v| v.get_untyped_property().clone())
            .chain([expanded_node.transform_and_bounds.untyped()])
            .collect();
        borrow_mut!(self.native_message_props).insert(
            id,
            Property::computed(
                move || {
                    let Some(expanded_node) = weak_self_ref.upgrade() else {
                        unreachable!()
                    };
                    let id = expanded_node.id.clone();
                    let mut old_state = borrow_mut!(last_patch);

                    let mut patch = TextboxPatch {
                        id: id.to_u32(),
                        ..Default::default()
                    };
                    expanded_node.with_properties_unwrapped(|properties: &mut Textbox| {
                        let computed_tab = expanded_node.transform_and_bounds.get();
                        let (width, height) = computed_tab.bounds;
                        let updates = [
                            patch_if_needed(
                                &mut old_state.text,
                                &mut patch.text,
                                properties.text.get(),
                            ),
                            patch_if_needed(&mut old_state.size_x, &mut patch.size_x, width),
                            patch_if_needed(&mut old_state.size_y, &mut patch.size_y, height),
                            patch_if_needed(
                                &mut old_state.transform,
                                &mut patch.transform,
                                computed_tab.transform.coeffs().to_vec(),
                            ),
                            patch_if_needed(
                                &mut old_state.style,
                                &mut patch.style,
                                (&properties.style.get()).into(),
                            ),
                            patch_if_needed(
                                &mut old_state.stroke_color,
                                &mut patch.stroke_color,
                                (&properties.stroke.get().color.get()).into(),
                            ),
                            patch_if_needed(
                                &mut old_state.stroke_width,
                                &mut patch.stroke_width,
                                properties.stroke.get().width.get().get_pixels(width),
                            ),
                            patch_if_needed(
                                &mut old_state.background,
                                &mut patch.background,
                                (&properties.background.get()).into(),
                            ),
                            patch_if_needed(
                                &mut old_state.border_radius,
                                &mut patch.border_radius,
                                properties.border_radius.get().to_float(),
                            ),
                            patch_if_needed(
                                &mut old_state.focus_on_mount,
                                &mut patch.focus_on_mount,
                                properties.focus_on_mount.get(),
                            ),
                        ];
                        if updates.into_iter().any(|v| v == true) {
                            context.enqueue_native_message(
                                pax_message::NativeMessage::TextboxUpdate(patch),
                            );
                        }
                    });
                    ()
                },
                &deps,
            ),
        );
    }

    fn handle_unmount(&self, expanded_node: &Rc<ExpandedNode>, context: &Rc<RuntimeContext>) {
        let id = expanded_node.id.clone();
        context.enqueue_native_message(pax_message::NativeMessage::TextboxDelete(id.to_u32()));
        // Reset so that native_message sending updates while unmounted
        borrow_mut!(self.native_message_props).remove(&id);
    }

    fn base(&self) -> &BaseInstance {
        &self.base
    }

    fn handle_native_interrupt(
        &self,
        expanded_node: &Rc<ExpandedNode>,
        interrupt: &pax_message::NativeInterrupt,
    ) {
        if let pax_message::NativeInterrupt::FormTextboxInput(args) = interrupt {
            expanded_node.with_properties_unwrapped(|textbox: &mut Textbox| {
                textbox.text.set(args.text.clone());
            })
        } else {
            log::warn!("textbox element was handed interrupt it doesn't use");
        }
    }

    fn resolve_debug(
        &self,
        f: &mut std::fmt::Formatter,
        _expanded_node: Option<&ExpandedNode>,
    ) -> std::fmt::Result {
        f.debug_struct("Textbox").finish_non_exhaustive()
    }
}
