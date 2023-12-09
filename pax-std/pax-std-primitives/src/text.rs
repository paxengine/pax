use std::cell::RefCell;

use core::any::Any;
use pax_core::{
    handle_vtable_update, with_properties_unwrapped, ExpandedNode, HandlerRegistry, InstanceNode,
    InstanceNodePtr, InstanceNodePtrList, InstantiationArgs, PaxEngine, PropertiesComputable,
    PropertiesTreeContext, RenderTreeContext,
};
use pax_message::{AnyCreatePatch, TextPatch};
use pax_runtime_api::{CommonProperties, Layer, SizePixels, StringBox};
use pax_std::primitives::Text;
use piet::RenderContext;
use std::collections::HashMap;
use std::rc::Rc;

use pax_std::types::text::{Font, TextAlignHorizontal, TextAlignVertical, TextStyle};
use pax_std::types::Color;

pub struct TextInstance {
    pub handler_registry: Option<Rc<RefCell<HandlerRegistry>>>,
    pub instance_id: u32,
    //Used as a cache of last-sent values, for crude dirty-checking.
    //Hopefully, this will by obviated by the built-in expression dirty-checking mechanism.
    //Note: must build in awareness of id_chain, since each virtual instance if this single `Text` instance
    //      shares this last_patches cache
    last_patches: HashMap<Vec<u32>, pax_message::TextPatch>,
    instance_prototypical_properties_factory: Box<dyn FnMut() -> Rc<RefCell<dyn Any>>>,
    instance_prototypical_common_properties_factory:
        Box<dyn FnMut() -> Rc<RefCell<CommonProperties>>>,
}

impl<R: 'static + RenderContext> InstanceNode<R> for TextInstance {
    fn get_instance_id(&self) -> u32 {
        self.instance_id
    }

    fn instantiate(args: InstantiationArgs<R>) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        let mut node_registry = (*args.node_registry).borrow_mut();
        let instance_id = node_registry.mint_instance_id();
        let ret = Rc::new(RefCell::new(TextInstance {
            instance_id,
            handler_registry: args.handler_registry,
            instance_prototypical_common_properties_factory: args
                .prototypical_common_properties_factory,
            instance_prototypical_properties_factory: args.prototypical_properties_factory,
            last_patches: HashMap::default(),
        }));

        node_registry.register(instance_id, Rc::clone(&ret) as InstanceNodePtr<R>);

        ret
    }

    fn get_instance_children(&self) -> InstanceNodePtrList<R> {
        Rc::new(RefCell::new(vec![]))
    }

    fn expand_node_and_compute_properties(
        &mut self,
        ptc: &mut PropertiesTreeContext<R>,
    ) -> Rc<RefCell<ExpandedNode<R>>> {
        let this_expanded_node = ExpandedNode::get_or_create_with_prototypical_properties(
            ptc,
            &(self.instance_prototypical_properties_factory)(),
            &(self.instance_prototypical_common_properties_factory)(),
        );
        let properties_wrapped = this_expanded_node.borrow().get_properties();

        with_properties_unwrapped!(&properties_wrapped, Text, |properties: &mut Text| {
            handle_vtable_update!(ptc, properties.text, pax_runtime_api::StringBox);
        });

        this_expanded_node
    }

    fn handle_native_patches(
        &mut self,
        ptc: &mut PropertiesTreeContext<R>,
        expanded_node: &ExpandedNode<R>,
    ) {
        let mut new_message: TextPatch = Default::default();
        new_message.id_chain = expanded_node.id_chain.clone();
        if !self.last_patches.contains_key(&new_message.id_chain) {
            let mut patch = TextPatch::default();
            patch.id_chain = new_message.id_chain.clone();
            self.last_patches
                .insert(new_message.id_chain.clone(), patch);
        }
        let last_patch = self.last_patches.get_mut(&new_message.id_chain).unwrap();
        let mut has_any_updates = false;

        let properties = expanded_node.get_properties();
        with_properties_unwrapped!(&properties, Text, |properties: &mut Text| {
            let val = properties.text.get().string.clone();
            let is_new_value = match &last_patch.content {
                Some(cached_value) => !val.eq(cached_value),
                None => true,
            };
            if is_new_value {
                new_message.content = Some(val.clone());
                last_patch.content = Some(val.clone());
                has_any_updates = true;
            }

            let val = properties.style.get();
            let _is_new_val = match &last_patch.style {
                Some(cached_value) => !val.eq(cached_value),
                None => true,
            };

            if is_new_value {
                new_message.style = Some(val.into());
                last_patch.style = Some(val.into());
                has_any_updates = true;
            }

            let val = properties.style_link.get();
            let _is_new_val = match &last_patch.style_link {
                Some(cached_value) => !val.eq(cached_value),
                None => true,
            };

            if is_new_value {
                new_message.style_link = Some(val.into());
                last_patch.style_link = Some(val.into());
                has_any_updates = true;
            }

            let val = expanded_node.computed_tab.as_ref().unwrap().bounds.0;
            let is_new_value = match &last_patch.size_x {
                Some(cached_value) => !val.eq(cached_value),
                None => true,
            };
            if is_new_value {
                new_message.size_x = Some(val);
                last_patch.size_x = Some(val);
                has_any_updates = true;
            }

            let val = expanded_node.computed_tab.as_ref().unwrap().bounds.1;
            let is_new_value = match &last_patch.size_y {
                Some(cached_value) => !val.eq(cached_value),
                None => true,
            };
            if is_new_value {
                new_message.size_y = Some(val);
                last_patch.size_y = Some(val);
                has_any_updates = true;
            }

            let latest_transform = expanded_node
                .computed_tab
                .as_ref()
                .unwrap()
                .transform
                .as_coeffs();
            let is_new_transform = match &last_patch.transform {
                Some(cached_transform) => latest_transform
                    .iter()
                    .enumerate()
                    .any(|(i, elem)| *elem != cached_transform[i]),
                None => true,
            };
            if is_new_transform {
                new_message.transform = Some(latest_transform.to_vec());
                last_patch.transform = Some(latest_transform.to_vec());
                has_any_updates = true;
            }

            if has_any_updates {
                ptc.enqueue_native_message(pax_message::NativeMessage::TextUpdate(new_message));
            }
        });
    }

    fn handle_render(&mut self, _rtc: &mut RenderTreeContext<R>, _rc: &mut R) {
        //no-op -- only native rendering for Text (unless/until we support rasterizing text, which Piet should be able to handle!)
    }

    fn handle_mount(&mut self, ptc: &mut PropertiesTreeContext<R>, node: &ExpandedNode<R>) {
        let id_chain = node.id_chain.clone();
        let canvas_index = node.computed_canvas_index.expect("no canvas index");

        //though macOS and iOS don't need this ancestry chain for clipping, Web does
        let clipping_ids = ptc.get_current_clipping_ids();

        let scroller_ids = ptc.get_current_scroller_ids();

        ptc.enqueue_native_message(pax_message::NativeMessage::TextCreate(AnyCreatePatch {
            id_chain,
            clipping_ids,
            scroller_ids,
            z_index: canvas_index,
        }));
    }

    fn get_layer_type(&mut self) -> Layer {
        Layer::Native
    }

    #[cfg(debug_assertions)]
    fn resolve_debug(
        &self,
        f: &mut std::fmt::Formatter,
        expanded_node: Option<&ExpandedNode<R>>,
    ) -> std::fmt::Result {
        match expanded_node {
            Some(expanded_node) => {
                with_properties_unwrapped!(&expanded_node.get_properties(), Text, |r: &mut Text| {
                    f.debug_struct("Text").field("text", r.text.get()).finish()
                })
            }
            None => f.debug_struct("Text").finish_non_exhaustive(),
        }
    }
}
/*fn expand_node_and_compute_properties(
    &mut self,
    ptc: &mut PropertiesTreeContext<R>,
) -> Rc<RefCell<ExpandedNode<R>>> {
    //
    // let properties = &mut *self.properties.as_ref().borrow_mut();
    //
    // if let Some(text) = rtc.compute_vtable_value(properties.text._get_vtable_id()) {
    //     let new_value = unsafe_unwrap!(text, TypesCoproduct, StringBox);
    //     properties.text.set(new_value);
    // }
    //
    // if let Some(style_font) =
    //     rtc.compute_vtable_value(properties.style.get().font._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_font, TypesCoproduct, Font);
    //     properties.style.get_mut().font.set(new_value);
    // }
    //
    // if let Some(style_font_size) =
    //     rtc.compute_vtable_value(properties.style.get().font_size._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_font_size, TypesCoproduct, SizePixels);
    //     properties.style.get_mut().font_size.set(new_value);
    // }
    //
    // if let Some(style_fill) =
    //     rtc.compute_vtable_value(properties.style.get().fill._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_fill, TypesCoproduct, Color);
    //     properties.style.get_mut().fill.set(new_value);
    // }
    //
    // if let Some(style_underline) =
    //     rtc.compute_vtable_value(properties.style.get().underline._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_underline, TypesCoproduct, bool);
    //     properties.style.get_mut().underline.set(new_value);
    // }
    //
    // if let Some(style_align_multiline) =
    //     rtc.compute_vtable_value(properties.style.get().align_multiline._get_vtable_id())
    // {
    //     let new_value =
    //         unsafe_unwrap!(style_align_multiline, TypesCoproduct, TextAlignHorizontal);
    //     properties.style.get_mut().align_multiline.set(new_value);
    // }
    //
    // if let Some(style_align_vertical) =
    //     rtc.compute_vtable_value(properties.style.get().align_vertical._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_align_vertical, TypesCoproduct, TextAlignVertical);
    //     properties.style.get_mut().align_vertical.set(new_value);
    // }
    //
    // if let Some(style_align_horizontal) =
    //     rtc.compute_vtable_value(properties.style.get().align_horizontal._get_vtable_id())
    // {
    //     let new_value =
    //         unsafe_unwrap!(style_align_horizontal, TypesCoproduct, TextAlignHorizontal);
    //     properties.style.get_mut().align_horizontal.set(new_value);
    // }
    //
    // if let Some(style_link) = rtc.compute_vtable_value(properties.style_link._get_vtable_id()) {
    //     let new_value = unsafe_unwrap!(style_link, TypesCoproduct, TextStyle);
    //     properties.style_link.set(new_value);
    // }
    //
    // let style_link = properties.style_link.get_mut();
    // if let Some(style_font) = rtc.compute_vtable_value(style_link.font._get_vtable_id()) {
    //     let new_value = unsafe_unwrap!(style_font, TypesCoproduct, Font);
    //     style_link.font.set(new_value);
    // }
    //
    // if let Some(style_font_size) =
    //     rtc.compute_vtable_value(style_link.font_size._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_font_size, TypesCoproduct, SizePixels);
    //     style_link.font_size.set(new_value);
    // }
    //
    // if let Some(style_fill) = rtc.compute_vtable_value(style_link.fill._get_vtable_id()) {
    //     let new_value = unsafe_unwrap!(style_fill, TypesCoproduct, Color);
    //     style_link.fill.set(new_value);
    // }
    //
    // if let Some(style_underline) =
    //     rtc.compute_vtable_value(style_link.underline._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_underline, TypesCoproduct, bool);
    //     style_link.underline.set(new_value);
    // }
    //
    // if let Some(style_align_multiline) =
    //     rtc.compute_vtable_value(style_link.align_multiline._get_vtable_id())
    // {
    //     let new_value =
    //         unsafe_unwrap!(style_align_multiline, TypesCoproduct, TextAlignHorizontal);
    //     style_link.align_multiline.set(new_value);
    // }
    //
    // if let Some(style_align_vertical) =
    //     rtc.compute_vtable_value(style_link.align_vertical._get_vtable_id())
    // {
    //     let new_value = unsafe_unwrap!(style_align_vertical, TypesCoproduct, TextAlignVertical);
    //     style_link.align_vertical.set(new_value);
    // }
    //
    // if let Some(style_align_horizontal) =
    //     rtc.compute_vtable_value(style_link.align_horizontal._get_vtable_id())
    // {
    //     let new_value =
    //         unsafe_unwrap!(style_align_horizontal, TypesCoproduct, TextAlignHorizontal);
    //     style_link.align_horizontal.set(new_value);
    // }

    // self.common_properties.compute_properties(rtc);
    todo!()
}*/
