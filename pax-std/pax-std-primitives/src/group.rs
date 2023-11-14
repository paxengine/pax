use pax_core::{
    ExpandedNode, HandlerRegistry, InstanceNode, InstanceNodePtr, InstanceNodePtrList,
    InstantiationArgs, PropertiesTreeContext,
};
use piet_common::RenderContext;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use pax_runtime_api::{CommonProperties, Layer};

/// Gathers a set of children underneath a single render node:
/// useful for composing transforms and simplifying render trees.
pub struct GroupInstance<R: 'static + RenderContext> {
    pub instance_id: u32,
    pub primitive_children: InstanceNodePtrList<R>,
    pub handler_registry: Option<Rc<RefCell<HandlerRegistry<R>>>>,

    instance_prototypical_properties: Rc<RefCell<dyn Any>>,
    instance_prototypical_common_properties: Rc<RefCell<CommonProperties>>,
}

impl<R: 'static + RenderContext> InstanceNode<R> for GroupInstance<R> {
    fn get_instance_id(&self) -> u32 {
        self.instance_id
    }

    fn get_instance_children(&self) -> InstanceNodePtrList<R> {
        Rc::clone(&self.primitive_children)
    }

    fn instantiate(args: InstantiationArgs<R>) -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        let mut node_registry = args.node_registry.borrow_mut();
        let instance_id = node_registry.mint_instance_id();
        let ret = Rc::new(RefCell::new(Self {
            instance_id,
            primitive_children: match args.children {
                None => Rc::new(RefCell::new(vec![])),
                Some(children) => children,
            },
            handler_registry: args.handler_registry,

            instance_prototypical_common_properties: args.common_properties,
            instance_prototypical_properties: args.properties,
        }));

        node_registry.register(instance_id, Rc::clone(&ret) as InstanceNodePtr<R>);
        ret
    }

    fn get_handler_registry(&self) -> Option<Rc<RefCell<HandlerRegistry<R>>>> {
        match &self.handler_registry {
            Some(registry) => Some(Rc::clone(&registry)),
            _ => None,
        }
    }

    /// Can never hit a Group directly -- can only hit elements inside of it.
    /// Events can still be propagated to a group.
    // fn ray_cast_test(&self, _ray: &(f64, f64), _tab: &TransformAndBounds) -> bool {
    //     false
    // }

    fn get_layer_type(&mut self) -> Layer {
        Layer::DontCare
    }

    fn expand_node_and_compute_properties(
        &mut self,
        ptc: &mut PropertiesTreeContext<R>,
    ) -> Rc<RefCell<ExpandedNode<R>>> {
        let this_expanded_node = ExpandedNode::get_or_create_with_prototypical_properties(
            ptc,
            &self.instance_prototypical_properties,
            &self.instance_prototypical_common_properties,
        );
        this_expanded_node
    }
}
