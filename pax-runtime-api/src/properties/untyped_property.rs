use super::{
    private::PropertyId,
    properties_table::{PropertyType, PROPERTY_TABLE},
    PropertyValue,
};

/// Reactive untyped property. Shallow clones can be cheaply made. Manages
/// refcounting and deletion of underlying data when all instances with a
/// specific PropertyId has been dropped.
#[derive(Debug)]
pub struct UntypedProperty {
    pub(crate) id: PropertyId,
}

impl Clone for UntypedProperty {
    fn clone(&self) -> Self {
        PROPERTY_TABLE.with(|t| {
            t.increase_ref_count(self.id);
        });
        UntypedProperty { id: self.id }
    }
}

impl Drop for UntypedProperty {
    fn drop(&mut self) {
        PROPERTY_TABLE.with(|t| {
            let ref_count = t.decrease_ref_count(self.id);
            if ref_count == 0 {
                t.remove_entry(self.id);
            }
        });
    }
}

impl UntypedProperty {
    pub(crate) fn new<T: PropertyValue>(
        val: T,
        inbound: Vec<PropertyId>,
        data: PropertyType<T>,
        debug_name: Option<&str>,
    ) -> Self {
        UntypedProperty {
            id: PROPERTY_TABLE.with(|t| t.add_entry(val, inbound, data, debug_name)),
        }
    }

    pub fn get_id(&self) -> PropertyId {
        self.id
    }
}
