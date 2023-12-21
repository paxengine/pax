use pax_lang::*;
use pax_lang::api::*;

// Given a string component ID, this component is intended
// to coordinate with the designtime to render a specific component, selected by string ID
// This affords us the ability to select the active component to render in the design tool

#[derive(Pax)]
pub struct DesigntimeComponentView {
    pub active_component_id: Property<String>
}
