use core::option::Option;

use std::rc::Rc;

use crate::{
    declarative_macros::handle_vtable_update, BaseInstance, ExpandedNode, InstanceFlags,
    InstanceNode, InstantiationArgs, RuntimeContext,
};
use pax_runtime_api::Layer;

/// A special "control-flow" primitive (a la `yield` or perhaps `goto`) — represents a slot into which
/// an slot_child can be rendered.  Slot relies on `slot_children` being present
/// on the [`Runtime`] stack and will not render any content if there are no `slot_children` found.
///
/// Consider a Stacker:  the owner of a Stacker passes the Stacker some nodes to render
/// inside the cells of the Stacker.  To the owner of the Stacker, those nodes might seem like
/// "children," but to the Stacker they are "slot_children" — children provided from
/// the outside.  Inside Stacker's template, there are a number of Slots — this primitive —
/// that become the final rendered home of those slot_children.  This same technique
/// is portable and applicable elsewhere via Slot.
pub struct SlotInstance {
    base: BaseInstance,
}

///Contains the index value for slot, either a literal or an expression.
#[derive(Default)]
pub struct SlotProperties {
    pub index: Box<dyn pax_runtime_api::PropertyInstance<pax_runtime_api::Numeric>>,
    last_index: usize,
}

impl InstanceNode for SlotInstance {
    fn instantiate(args: InstantiationArgs) -> Rc<Self>
    where
        Self: Sized,
    {
        Rc::new(Self {
            base: BaseInstance::new(
                args,
                InstanceFlags {
                    invisible_to_slot: false,
                    invisible_to_raycasting: true,
                    layer: Layer::DontCare,
                    is_component: false,
                },
            ),
        })
    }

    fn recompute_children(
        self: Rc<Self>,
        expanded_node: &Rc<ExpandedNode>,
        context: &mut RuntimeContext,
    ) {
        let _index: usize =
            expanded_node.with_properties_unwrapped(|properties: &mut SlotProperties| {
                properties
                    .index
                    .get()
                    .get_as_int()
                    .try_into()
                    .expect("Slot index must be non-negative")
            });

        // The Stacker (or other slotted component) itself (that this slot is in)
        let current_component = expanded_node.containing_component.upgrade().unwrap();
        // The component the Stacker (or other slotted component) resides in (we need its environment)
        let parent_component = current_component.containing_component.upgrade().unwrap();

        // This is not the component hierarchy itself, but the children that
        // have been placed inside the component in this specific place
        let templates = current_component
            .instance_template
            .base()
            .get_template_children();

        let env = Rc::clone(&current_component.stack);
        // TODO
        // expanded_node.set_children(, )
    }

    fn update(self: Rc<Self>, expanded_node: &Rc<ExpandedNode>, context: &mut RuntimeContext) {
        let update_child =
            expanded_node.with_properties_unwrapped(|properties: &mut SlotProperties| {
                handle_vtable_update(
                    &context.expression_table(),
                    &expanded_node.stack,
                    &mut properties.index,
                );
                let curr_index: usize = properties.index.get().get_as_int().try_into().unwrap();
                let update_child = properties.last_index != curr_index;
                properties.last_index = curr_index;
                update_child
            });

        if update_child {
            self.recompute_children(expanded_node, context);
        }
    }

    #[cfg(debug_assertions)]
    fn resolve_debug(
        &self,
        f: &mut std::fmt::Formatter,
        _expanded_node: Option<&ExpandedNode>,
    ) -> std::fmt::Result {
        f.debug_struct("Slot").finish()
    }

    fn base(&self) -> &BaseInstance {
        &self.base
    }
}
