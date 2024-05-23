#![allow(unused, unused_imports, non_snake_case, unused_parens)]
use pax_manifest::*;
use pax_runtime::api::*;
use pax_runtime::*;
use pax_manifest::deserializer::{from_pax_try_coerce};
use std::cell::Ref;
use pax_runtime::api::properties::UntypedProperty;
use pax_manifest::ControlFlowRepeatPredicateDefinition::ElemIdIndexId;
use pax_manifest::ControlFlowRepeatPredicateDefinition::ElemId;
use pax_runtime_api::pax_value::PaxValue;
use pax_runtime_api::pax_value::PaxAny;
use pax_runtime_api::pax_value::ToFromPaxAny;
use pax_runtime_api::{borrow, borrow_mut};
use pax_runtime::api::pax_value::ToFromPaxValue;

const INITAL_MANIFEST: &str = include_str!("../initial-manifest.json");

// generate imports, pointing to userland cartridge `pub mod pax_reexports`

use playground::pax_reexports::pax_std::types::text::TextStyle;

use playground::pax_reexports::pax_engine::api::Transform2D;

use playground::pax_reexports::pax_std::types::text::SystemFont;

use playground::pax_reexports::pax_std::types::text::TextAlignHorizontal;

use playground::pax_reexports::Example;

use playground::pax_reexports::pax_engine::api::Fill;

use playground::pax_reexports::pax_std::primitives::Text;

use playground::pax_reexports::pax_std::primitives::Group;

use playground::pax_reexports::std::string::String;

use playground::pax_reexports::pax_std::types::RectangleCornerRadii;

use playground::pax_reexports::pax_std::primitives::Rectangle;

use playground::pax_reexports::pax_std::types::text::Font;

use playground::pax_reexports::pax_std::types::text::FontWeight;

use playground::pax_reexports::pax_engine::api::Numeric;

use playground::pax_reexports::pax_engine::api::Rotation;

use bool;

use usize;

use playground::pax_reexports::pax_engine::api::Size;

use playground::pax_reexports::pax_std::types::text::WebFont;

use playground::pax_reexports::pax_std::primitives::BlankComponent;

use playground::pax_reexports::pax_std::types::text::LocalFont;

use playground::pax_reexports::pax_std::types::text::TextAlignVertical;

use playground::pax_reexports::pax_engine::api::Stroke;

use playground::pax_reexports::pax_engine::api::Color;

use playground::pax_reexports::pax_engine::api::ColorChannel;

use playground::pax_reexports::pax_std::types::text::FontStyle;

use std::any::Any;

use pax_runtime::api::{use_RefCell};

use std::collections::HashMap;

use std::collections::VecDeque;

use std::ops::Deref;

use std::rc::Rc;

use pax_runtime::RepeatItem;

use pax_runtime::RepeatProperties;

use pax_runtime::ConditionalProperties;

use pax_runtime::SlotProperties;

use pax_runtime::api::Property;

use pax_runtime::api::CommonProperties;

use pax_runtime::api::Color::*;

use pax_runtime::ComponentInstance;

use pax_runtime::InstanceNodePtr;

use pax_runtime::InstanceNodePtrList;

use pax_runtime::ExpressionContext;

use pax_runtime::PaxEngine;

use pax_runtime::InstanceNode;

use pax_runtime::HandlerRegistry;

use pax_runtime::InstantiationArgs;

use pax_runtime::ConditionalInstance;

use pax_runtime::SlotInstance;

use pax_runtime::properties::RuntimePropertiesStackFrame;

use pax_runtime::repeat::RepeatInstance;

use piet_common::RenderContext;


use_RefCell!();

pub fn instantiate_expression_table() -> HashMap<usize, Box<dyn Fn(ExpressionContext) -> PaxAny>> {
    let mut vtable: HashMap<usize, Box<dyn Fn(ExpressionContext) -> PaxAny>> = HashMap::new();

    
    // self.num_clicks+"clicks"
    
        
    
    vtable.insert(0, Box::new(|ec: ExpressionContext| -> PaxAny {
        
            let num_clicks =
            {
                let properties = if let Some(sf) = ec.stack_frame.resolve_symbol("num_clicks") {
                    Rc::clone(&sf)
                } else {
                    panic!("num_clicks didn't have an 0th stackframe");
                };
                let mut borrowed = &mut *borrow_mut!(*properties);
                

                    if let Ok(p) = <playground::pax_reexports::Example>::ref_from_pax_any(&*borrowed) {

                        
                            //binding simple numeric property
                            Numeric::from(p.num_clicks.get())
                        
                    } else {unreachable!()}
                
            };
            
                let num_clicks = Numeric::from( num_clicks );
            

        

        

        let ___ret = ((num_clicks).to_pax_any()+(" clicks").to_string().to_pax_any());

        ___ret.to_pax_any()
    }));
    
        
    
    
    // Font::system("TimesNewRoman",FontStyle::Normal,FontWeight::Bold)
    
        
    
    vtable.insert(1, Box::new(|ec: ExpressionContext| -> PaxAny {
        

        

        let ___ret = (Font::system((("Times New Roman").to_string().to_pax_any()).try_coerce().unwrap(),((FontStyle::Normal).to_pax_any()).try_coerce().unwrap(),((FontWeight::Bold).to_pax_any()).try_coerce().unwrap(),)).to_pax_any();

        ___ret.to_pax_any()
    }));
    
        
    
    
    // rgb(ticks,75,150)
    
        
    
    vtable.insert(2, Box::new(|ec: ExpressionContext| -> PaxAny {
        
            let ticks =
            {
                let properties = if let Some(sf) = ec.stack_frame.resolve_symbol("ticks") {
                    Rc::clone(&sf)
                } else {
                    panic!("ticks didn't have an 0th stackframe");
                };
                let mut borrowed = &mut *borrow_mut!(*properties);
                

                    if let Ok(p) = <playground::pax_reexports::Example>::ref_from_pax_any(&*borrowed) {

                        
                            //binding simple numeric property
                            Numeric::from(p.ticks.get())
                        
                    } else {unreachable!()}
                
            };
            
                let ticks = Numeric::from( ticks );
            

        

        

        let ___ret = Color::rgb((ticks).to_pax_any().try_coerce().unwrap(),(75).to_pax_any().try_coerce().unwrap(),(150).to_pax_any().try_coerce().unwrap(),).to_pax_any();

        ___ret.to_pax_any()
    }));
    
        
    
    
    // RectangleCornerRadii::radii(10.0,10.0,10.0,10.0)
    
        
    
    vtable.insert(3, Box::new(|ec: ExpressionContext| -> PaxAny {
        

        

        let ___ret = (RectangleCornerRadii::radii(((10.0).to_pax_any()).try_coerce().unwrap(),((10.0).to_pax_any()).try_coerce().unwrap(),((10.0).to_pax_any()).try_coerce().unwrap(),((10.0).to_pax_any()).try_coerce().unwrap(),)).to_pax_any();

        ___ret.to_pax_any()
    }));
    
        
    
    

    vtable
}

pub trait ComponentFactory {

    /// Returns the default CommonProperties factory
    fn build_default_common_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<CommonProperties>>>{
        Box::new(|_,_| Rc::new(RefCell::new(CommonProperties::default())))    
    }

    /// Returns the default properties factory for this component
    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>>;
    
    /// Returns the CommonProperties factory based on the defined properties 
    fn build_inline_common_properties(&self, defined_properties: HashMap<String,ValueDefinition>) ->Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<CommonProperties>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new({
            let mut cp = CommonProperties::default();
            for (key, value) in &defined_properties {
                match key.as_str() {
                    
                    "id" => {
                        let resolved_property: Property<Option<String>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<String>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<String>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for id")
                        };
                        cp.id = resolved_property;
                    },
                    
                    "x" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for x")
                        };
                        cp.x = resolved_property;
                    },
                    
                    "y" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for y")
                        };
                        cp.y = resolved_property;
                    },
                    
                    "scale_x" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for scale_x")
                        };
                        cp.scale_x = resolved_property;
                    },
                    
                    "scale_y" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for scale_y")
                        };
                        cp.scale_y = resolved_property;
                    },
                    
                    "skew_x" => {
                        let resolved_property: Property<Option<f64>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<f64>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<f64>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for skew_x")
                        };
                        cp.skew_x = resolved_property;
                    },
                    
                    "skew_y" => {
                        let resolved_property: Property<Option<f64>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<f64>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<f64>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for skew_y")
                        };
                        cp.skew_y = resolved_property;
                    },
                    
                    "anchor_x" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for anchor_x")
                        };
                        cp.anchor_x = resolved_property;
                    },
                    
                    "anchor_y" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for anchor_y")
                        };
                        cp.anchor_y = resolved_property;
                    },
                    
                    "rotate" => {
                        let resolved_property: Property<Option<pax_engine::api::Rotation>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Rotation>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Rotation>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for rotate")
                        };
                        cp.rotate = resolved_property;
                    },
                    
                    "transform" => {
                        let resolved_property: Property<Option<pax_engine::api::Transform2D>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Transform2D>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Transform2D>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for transform")
                        };
                        cp.transform = resolved_property;
                    },
                    
                    "width" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for width")
                        };
                        cp.width = resolved_property;
                    },
                    
                    "height" => {
                        let resolved_property: Property<Option<pax_engine::api::Size>> = match value.clone() {
                            ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<pax_engine::api::Size>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                                Property::new_with_name(Some(val), &lv.raw_value)
                            },
                            ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                            {
                                if let Some(info) = info {
                                    let mut dependents = vec![];
                                    for dependency in &info.dependencies {
                                        if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                            dependents.push(p);
                                        } else {
                                            panic!("Failed to resolve symbol {}", dependency);
                                        }
                                    }
                                    let cloned_stack = stack_frame.clone();
                                    let cloned_table = table.clone();
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                        let coerced = new_value_wrapped.try_coerce::<pax_engine::api::Size>().unwrap();
                                        Some(coerced)
                                    }, &dependents, &token.raw_value)
                                } else {
                                    unreachable!("No info for expression")
                                }
                            },
                            _ => unreachable!("Invalid value definition for height")
                        };
                        cp.height = resolved_property;
                    },
                    
                    _ => {}
                }
            }

            cp.clone()
        })))
    }

    /// Returns the properties factory based on the defined properties
    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>>;
    
    /// Returns the requested closure for the handler registry based on the defined handlers for this component
    /// The argument type is extrapolated based on how the handler was used in the initial compiled template
    fn build_handler(&self, fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>);
    
    /// Returns the handler registry based on the defined handlers for this component
    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>>;

    // Takes a hander registry and adds the given inline handlers to it
    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>>;
   
    // Calls the instantion function for the component
    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode>;

    // Returns the property scope for the component
    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>> {
        Box::new(|_| HashMap::new())
    }
}


struct ExampleFactory{}

impl ComponentFactory for ExampleFactory {

    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(|_,_| Rc::new(RefCell::new(Example::default().to_pax_any())))
    }

    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new(
            {
        let mut properties = Example::default();
        
            if let Some(vd) = defined_properties.get("ticks") {
                properties.ticks.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<usize>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<usize>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(usizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for ticks")
                    });
            }
        
            if let Some(vd) = defined_properties.get("num_clicks") {
                properties.num_clicks.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<usize>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<usize>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(usizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for num_clicks")
                    });
            }
        
        properties.to_pax_any()
        })))
    }

    fn build_handler(&self,fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>) {
        match fn_name {
            
            "handle_pre_render" => {
                |properties, ctx, args|{
                    let properties = &mut *borrow_mut!(properties.as_ref());
                    if let Ok(mut properties) = <Example>::mut_from_pax_any(properties) {
                        // downcast args to handler.type
                        
                            if let None = args {
                                Example::handle_pre_render(properties,ctx);
                            } else {
                                panic!("Unexpected args present");
                            }
                        
                        
                    } else {panic!("Failed to downcast properties to Example")};
                }
            },
            
            "increment" => {
                |properties, ctx, args|{
                    let properties = &mut *borrow_mut!(properties.as_ref());
                    if let Ok(mut properties) = <Example>::mut_from_pax_any(properties) {
                        // downcast args to handler.type
                        
                            if let Some(args) = args {
                                if let Ok(args) = <Event<Click>>::ref_from_pax_any(&args) {
                                    Example::increment(properties,ctx, args.clone());
                                } else {panic!("Failed to downcast args to Event<Click>")};
                            } else {
                                panic!("No Event<Click> present");
                            }
                        
                        
                    } else {panic!("Failed to downcast properties to Example")};
                }
            },
            
            _ => panic!("Unknown handler name {}", fn_name)
        }
    }

    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>> {
        let mut handler_registry = HandlerRegistry::default();
        for (event, functions) in &handlers {
            handler_registry.handlers.insert(event.clone(), functions.iter().map(|fn_name| {
                Handler::new_component_handler(self.build_handler(&fn_name))
            }).collect());
        } 
        Rc::new(RefCell::new(handler_registry))
    }

    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, handler_registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>> {
        {
            let mut handler_registry_mut = borrow_mut!(handler_registry);
            for (event, fn_name) in &handlers {
                let handler_vec = handler_registry_mut.handlers.entry(event.clone()).or_insert(Vec::new());
                handler_vec.push(Handler::new_inline_handler(self.build_handler(&fn_name)));
            } 
        }   
        handler_registry
    }

    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode> {
        
        ComponentInstance::instantiate(args)
            
    }

    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>>  {
        Box::new(|props| {
            let properties = &mut *borrow_mut!(props.as_ref());
            if let Ok(properties) = <Example>::mut_from_pax_any(properties) {
                let mut scope = HashMap::new();
                
                    scope.insert("ticks".to_string(), properties.ticks.untyped());
                
                    scope.insert("num_clicks".to_string(), properties.num_clicks.untyped());
                
                scope
            } else {
                panic!("Failed to downcast properties to Example");
            }
        })
    }

}
struct BlankComponentFactory{}

impl ComponentFactory for BlankComponentFactory {

    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(|_,_| Rc::new(RefCell::new(BlankComponent::default().to_pax_any())))
    }

    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new(
            {
        let mut properties = BlankComponent::default();
        
        properties.to_pax_any()
        })))
    }

    fn build_handler(&self,fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>) {
        match fn_name {
            
            _ => panic!("Unknown handler name {}", fn_name)
        }
    }

    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>> {
        let mut handler_registry = HandlerRegistry::default();
        for (event, functions) in &handlers {
            handler_registry.handlers.insert(event.clone(), functions.iter().map(|fn_name| {
                Handler::new_component_handler(self.build_handler(&fn_name))
            }).collect());
        } 
        Rc::new(RefCell::new(handler_registry))
    }

    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, handler_registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>> {
        {
            let mut handler_registry_mut = borrow_mut!(handler_registry);
            for (event, fn_name) in &handlers {
                let handler_vec = handler_registry_mut.handlers.entry(event.clone()).or_insert(Vec::new());
                handler_vec.push(Handler::new_inline_handler(self.build_handler(&fn_name)));
            } 
        }   
        handler_registry
    }

    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode> {
        
        ComponentInstance::instantiate(args)
            
    }

    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>>  {
        Box::new(|props| {
            let properties = &mut *borrow_mut!(props.as_ref());
            if let Ok(properties) = <BlankComponent>::mut_from_pax_any(properties) {
                let mut scope = HashMap::new();
                
                scope
            } else {
                panic!("Failed to downcast properties to BlankComponent");
            }
        })
    }

}
struct GroupFactory{}

impl ComponentFactory for GroupFactory {

    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(|_,_| Rc::new(RefCell::new(Group::default().to_pax_any())))
    }

    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new(
            {
        let mut properties = Group::default();
        
        properties.to_pax_any()
        })))
    }

    fn build_handler(&self,fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>) {
        match fn_name {
            
            _ => panic!("Unknown handler name {}", fn_name)
        }
    }

    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>> {
        let mut handler_registry = HandlerRegistry::default();
        for (event, functions) in &handlers {
            handler_registry.handlers.insert(event.clone(), functions.iter().map(|fn_name| {
                Handler::new_component_handler(self.build_handler(&fn_name))
            }).collect());
        } 
        Rc::new(RefCell::new(handler_registry))
    }

    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, handler_registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>> {
        {
            let mut handler_registry_mut = borrow_mut!(handler_registry);
            for (event, fn_name) in &handlers {
                let handler_vec = handler_registry_mut.handlers.entry(event.clone()).or_insert(Vec::new());
                handler_vec.push(Handler::new_inline_handler(self.build_handler(&fn_name)));
            } 
        }   
        handler_registry
    }

    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode> {
        
        pax_std_primitives::group::GroupInstance::instantiate(args)
            
    }

    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>>  {
        Box::new(|props| {
            let properties = &mut *borrow_mut!(props.as_ref());
            if let Ok(properties) = <Group>::mut_from_pax_any(properties) {
                let mut scope = HashMap::new();
                
                scope
            } else {
                panic!("Failed to downcast properties to Group");
            }
        })
    }

}
struct RectangleFactory{}

impl ComponentFactory for RectangleFactory {

    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(|_,_| Rc::new(RefCell::new(Rectangle::default().to_pax_any())))
    }

    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new(
            {
        let mut properties = Rectangle::default();
        
            if let Some(vd) = defined_properties.get("stroke") {
                properties.stroke.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Stroke>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Stroke>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOStrokeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for stroke")
                    });
            }
        
            if let Some(vd) = defined_properties.get("fill") {
                properties.fill.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Fill>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Fill>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOFillTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for fill")
                    });
            }
        
            if let Some(vd) = defined_properties.get("corner_radii") {
                properties.corner_radii.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::RectangleCornerRadii>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::RectangleCornerRadii>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCORectangleCornerRadiiTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for corner_radii")
                    });
            }
        
        properties.to_pax_any()
        })))
    }

    fn build_handler(&self,fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>) {
        match fn_name {
            
            _ => panic!("Unknown handler name {}", fn_name)
        }
    }

    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>> {
        let mut handler_registry = HandlerRegistry::default();
        for (event, functions) in &handlers {
            handler_registry.handlers.insert(event.clone(), functions.iter().map(|fn_name| {
                Handler::new_component_handler(self.build_handler(&fn_name))
            }).collect());
        } 
        Rc::new(RefCell::new(handler_registry))
    }

    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, handler_registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>> {
        {
            let mut handler_registry_mut = borrow_mut!(handler_registry);
            for (event, fn_name) in &handlers {
                let handler_vec = handler_registry_mut.handlers.entry(event.clone()).or_insert(Vec::new());
                handler_vec.push(Handler::new_inline_handler(self.build_handler(&fn_name)));
            } 
        }   
        handler_registry
    }

    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode> {
        
        pax_std_primitives::rectangle::RectangleInstance::instantiate(args)
            
    }

    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>>  {
        Box::new(|props| {
            let properties = &mut *borrow_mut!(props.as_ref());
            if let Ok(properties) = <Rectangle>::mut_from_pax_any(properties) {
                let mut scope = HashMap::new();
                
                    scope.insert("stroke".to_string(), properties.stroke.untyped());
                
                    scope.insert("fill".to_string(), properties.fill.untyped());
                
                    scope.insert("corner_radii".to_string(), properties.corner_radii.untyped());
                
                scope
            } else {
                panic!("Failed to downcast properties to Rectangle");
            }
        })
    }

}
struct TextFactory{}

impl ComponentFactory for TextFactory {

    fn build_default_properties(&self) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(|_,_| Rc::new(RefCell::new(Text::default().to_pax_any())))
    }

    fn build_inline_properties(&self, defined_properties: HashMap<String,ValueDefinition>) -> Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> {
        Box::new(move |stack_frame , table | Rc::new(RefCell::new(
            {
        let mut properties = Text::default();
        
            if let Some(vd) = defined_properties.get("editable") {
                properties.editable.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<bool>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<bool>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(boolTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for editable")
                    });
            }
        
            if let Some(vd) = defined_properties.get("text") {
                properties.text.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::std::string::String>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for text")
                    });
            }
        
            if let Some(vd) = defined_properties.get("style") {
                properties.style.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for style")
                    });
            }
        
            if let Some(vd) = defined_properties.get("style_link") {
                properties.style_link.replace_with(
                    match vd.clone() {
                        ValueDefinition::LiteralValue(lv) => {
                                let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>(&lv.raw_value)
                                    .map_err(|e| format!("failed to read {}: {}", &lv.raw_value, e)).unwrap();
                            Property::new_with_name(val, &lv.raw_value)
                        },
                        ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token,info) =>
                        {
                            if let Some(info) = info {
                                let mut dependents = vec![];
                                for dependency in &info.dependencies {
                                    if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                        dependents.push(p);
                                    } else {
                                        panic!("Failed to resolve symbol {}", dependency);
                                    }
                                }
                                let cloned_stack = stack_frame.clone();
                                let cloned_table = table.clone();
                                Property::computed_with_name(move || {
                                    let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, info.vtable_id.clone());
                                    let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>().unwrap();
                                    coerced
                                }, &dependents, &token.raw_value)
                            } else {
                                unreachable!("No info for expression")
                            }
                        },
                        ValueDefinition::Block(block) => {
                            Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                        }
                        _ => unreachable!("Invalid value definition for style_link")
                    });
            }
        
        properties.to_pax_any()
        })))
    }

    fn build_handler(&self,fn_name: &str) -> fn(Rc<RefCell<PaxAny>>, &NodeContext, Option::<PaxAny>) {
        match fn_name {
            
            _ => panic!("Unknown handler name {}", fn_name)
        }
    }

    fn build_component_handlers(&self, handlers: Vec<(String, Vec<String>)>) -> Rc<RefCell<HandlerRegistry>> {
        let mut handler_registry = HandlerRegistry::default();
        for (event, functions) in &handlers {
            handler_registry.handlers.insert(event.clone(), functions.iter().map(|fn_name| {
                Handler::new_component_handler(self.build_handler(&fn_name))
            }).collect());
        } 
        Rc::new(RefCell::new(handler_registry))
    }

    fn add_inline_handlers(&self, handlers: Vec<(String, String)>, handler_registry: Rc<RefCell<HandlerRegistry>>) -> Rc<RefCell<HandlerRegistry>> {
        {
            let mut handler_registry_mut = borrow_mut!(handler_registry);
            for (event, fn_name) in &handlers {
                let handler_vec = handler_registry_mut.handlers.entry(event.clone()).or_insert(Vec::new());
                handler_vec.push(Handler::new_inline_handler(self.build_handler(&fn_name)));
            } 
        }   
        handler_registry
    }

    fn build_component(&self, args: InstantiationArgs) -> Rc<dyn InstanceNode> {
        
        pax_std_primitives::text::TextInstance::instantiate(args)
            
    }

    fn get_properties_scope_factory(&self) -> Box<dyn Fn(Rc<RefCell<PaxAny>>) -> HashMap<String, UntypedProperty>>  {
        Box::new(|props| {
            let properties = &mut *borrow_mut!(props.as_ref());
            if let Ok(properties) = <Text>::mut_from_pax_any(properties) {
                let mut scope = HashMap::new();
                
                    scope.insert("editable".to_string(), properties.editable.untyped());
                
                    scope.insert("text".to_string(), properties.text.untyped());
                
                    scope.insert("style".to_string(), properties.style.untyped());
                
                    scope.insert("style_link".to_string(), properties.style_link.untyped());
                
                scope
            } else {
                panic!("Failed to downcast properties to Text");
            }
        })
    }

}

trait TypeFactory {
    type Output: Default + Clone;
    
    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output;
}

#[allow(non_camel_case_types)]
struct StringTypeFactory{}

impl TypeFactory for StringTypeFactory {

    type Output=String;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: String = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct boolTypeFactory{}

impl TypeFactory for boolTypeFactory {

    type Output=bool;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: bool = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct f64TypeFactory{}

impl TypeFactory for f64TypeFactory {

    type Output=f64;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: f64 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct i128TypeFactory{}

impl TypeFactory for i128TypeFactory {

    type Output=i128;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: i128 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct i16TypeFactory{}

impl TypeFactory for i16TypeFactory {

    type Output=i16;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: i16 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct i32TypeFactory{}

impl TypeFactory for i32TypeFactory {

    type Output=i32;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: i32 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct i64TypeFactory{}

impl TypeFactory for i64TypeFactory {

    type Output=i64;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: i64 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct i8TypeFactory{}

impl TypeFactory for i8TypeFactory {

    type Output=i8;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: i8 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct isizeTypeFactory{}

impl TypeFactory for isizeTypeFactory {

    type Output=isize;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: isize = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct u128TypeFactory{}

impl TypeFactory for u128TypeFactory {

    type Output=u128;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: u128 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct u16TypeFactory{}

impl TypeFactory for u16TypeFactory {

    type Output=u16;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: u16 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct u32TypeFactory{}

impl TypeFactory for u32TypeFactory {

    type Output=u32;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: u32 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct u64TypeFactory{}

impl TypeFactory for u64TypeFactory {

    type Output=u64;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: u64 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct u8TypeFactory{}

impl TypeFactory for u8TypeFactory {

    type Output=u8;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: u8 = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct usizeTypeFactory{}

impl TypeFactory for usizeTypeFactory {

    type Output=usize;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: usize = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOBlankComponentTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOBlankComponentTypeFactory {

    type Output=playground::pax_reexports::pax_std::primitives::BlankComponent;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::primitives::BlankComponent = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOColorTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOColorTypeFactory {

    type Output=playground::pax_reexports::pax_engine::api::Color;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_engine::api::Color = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOExampleTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOExampleTypeFactory {

    type Output=playground::pax_reexports::Example;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::Example = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "ticks" => {
                        
                            properties.ticks = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<usize>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<usize>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(usizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for ticks")
                                };
                            
                        
                    },
                    
                    "num_clicks" => {
                        
                            properties.num_clicks = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<usize>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<usize>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(usizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for num_clicks")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOFillTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOFillTypeFactory {

    type Output=playground::pax_reexports::pax_engine::api::Fill;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_engine::api::Fill = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::Font;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::Font = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "System" => {
                        
                    },
                    
                    "Web" => {
                        
                    },
                    
                    "Local" => {
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontStyleTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontStyleTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::FontStyle;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::FontStyle = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontWeightTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontWeightTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::FontWeight;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::FontWeight = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOGroupTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOGroupTypeFactory {

    type Output=playground::pax_reexports::pax_std::primitives::Group;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::primitives::Group = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOLocalFontTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOLocalFontTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::LocalFont;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::LocalFont = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "family" => {
                        
                            properties.family = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for family")
                                };
                            
                        
                    },
                    
                    "path" => {
                        
                            properties.path = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for path")
                                };
                            
                        
                    },
                    
                    "style" => {
                        
                            properties.style = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontStyle>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontStyleTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for style")
                                };
                            
                        
                    },
                    
                    "weight" => {
                        
                            properties.weight = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontWeight>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontWeightTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for weight")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory {

    type Output=playground::pax_reexports::pax_engine::api::Numeric;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_engine::api::Numeric = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCORectangleTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCORectangleTypeFactory {

    type Output=playground::pax_reexports::pax_std::primitives::Rectangle;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::primitives::Rectangle = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "stroke" => {
                        
                            properties.stroke = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Stroke>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Stroke>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOStrokeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for stroke")
                                };
                            
                        
                    },
                    
                    "fill" => {
                        
                            properties.fill = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Fill>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Fill>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOFillTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for fill")
                                };
                            
                        
                    },
                    
                    "corner_radii" => {
                        
                            properties.corner_radii = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::RectangleCornerRadii>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::RectangleCornerRadii>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCORectangleCornerRadiiTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for corner_radii")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCORectangleCornerRadiiTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCORectangleCornerRadiiTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::RectangleCornerRadii;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::RectangleCornerRadii = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "top_left" => {
                        
                            properties.top_left = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for top_left")
                                };
                            
                        
                    },
                    
                    "top_right" => {
                        
                            properties.top_right = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for top_right")
                                };
                            
                        
                    },
                    
                    "bottom_right" => {
                        
                            properties.bottom_right = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for bottom_right")
                                };
                            
                        
                    },
                    
                    "bottom_left" => {
                        
                            properties.bottom_left = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Numeric>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCONumericTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for bottom_left")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOSizeTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOSizeTypeFactory {

    type Output=playground::pax_reexports::pax_engine::api::Size;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_engine::api::Size = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory {

    type Output=playground::pax_reexports::std::string::String;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::std::string::String = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOStrokeTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOStrokeTypeFactory {

    type Output=playground::pax_reexports::pax_engine::api::Stroke;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_engine::api::Stroke = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "color" => {
                        
                            properties.color = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Color>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Color>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOColorTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for color")
                                };
                            
                        
                    },
                    
                    "width" => {
                        
                            properties.width = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Size>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Size>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOSizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for width")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOSystemFontTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOSystemFontTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::SystemFont;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::SystemFont = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "family" => {
                        
                            properties.family = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for family")
                                };
                            
                        
                    },
                    
                    "style" => {
                        
                            properties.style = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontStyle>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontStyleTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for style")
                                };
                            
                        
                    },
                    
                    "weight" => {
                        
                            properties.weight = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontWeight>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontWeightTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for weight")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOTextTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOprimitivesCOCOTextTypeFactory {

    type Output=playground::pax_reexports::pax_std::primitives::Text;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::primitives::Text = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "editable" => {
                        
                            properties.editable = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<bool>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<bool>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(boolTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for editable")
                                };
                            
                        
                    },
                    
                    "text" => {
                        
                            properties.text = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::std::string::String>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for text")
                                };
                            
                        
                    },
                    
                    "style" => {
                        
                            properties.style = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for style")
                                };
                            
                        
                    },
                    
                    "style_link" => {
                        
                            properties.style_link = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextStyle>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for style_link")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignHorizontalTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignHorizontalTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::TextAlignHorizontal;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::TextAlignHorizontal = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignVerticalTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignVerticalTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::TextAlignVertical;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::TextAlignVertical = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextStyleTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::TextStyle;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::TextStyle = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "font" => {
                        
                            properties.font = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::Font>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::Font>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for font")
                                };
                            
                        
                    },
                    
                    "font_size" => {
                        
                            properties.font_size = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Size>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Size>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOSizeTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for font_size")
                                };
                            
                        
                    },
                    
                    "fill" => {
                        
                            properties.fill = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_engine::api::Color>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_engine::api::Color>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_engineCOCOapiCOCOColorTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for fill")
                                };
                            
                        
                    },
                    
                    "underline" => {
                        
                            properties.underline = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<bool>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<bool>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(boolTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for underline")
                                };
                            
                        
                    },
                    
                    "align_multiline" => {
                        
                            properties.align_multiline = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignHorizontal>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignHorizontal>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignHorizontalTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for align_multiline")
                                };
                            
                        
                    },
                    
                    "align_vertical" => {
                        
                            properties.align_vertical = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignVertical>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignVertical>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignVerticalTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for align_vertical")
                                };
                            
                        
                    },
                    
                    "align_horizontal" => {
                        
                            properties.align_horizontal = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        let val = from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignHorizontal>(&lv.raw_value).unwrap();
                                        Property::new_with_name(val, &lv.raw_value)
                                    },
                                    ValueDefinition::Expression(token, info) | ValueDefinition::Identifier(token, info ) =>
                                    {
                                        if let Some(info) = info {
                                            let mut dependents = vec![];
                                            for dependency in &info.dependencies {
                                                if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                                    dependents.push(p);
                                                } else {
                                                    panic!("Failed to resolve symbol {}", dependency);
                                                }
                                            }
                                            let cloned_stack = stack_frame.clone();
                                            let cloned_table = table.clone();
                                            let cloned_info = info.clone();
                                            Property::computed_with_name(move || {
                                                let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, cloned_info.vtable_id);
                                                let coerced = new_value_wrapped.try_coerce::<playground::pax_reexports::pax_std::types::text::TextAlignHorizontal>().unwrap();
                                                coerced
                                            }, &dependents, &token.raw_value)
                                        } else {
                                            unreachable!("No info for expression")
                                        }
                                    },
                                    ValueDefinition::Block(block) => {
                                        Property::new_with_name(playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOTextAlignHorizontalTypeFactory{}.build_type(&block, stack_frame.clone(), table.clone()), "block")
                                    }
                                    _ => unreachable!("Invalid value definition for align_horizontal")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        
#[allow(non_camel_case_types)]
struct playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOWebFontTypeFactory{}

impl TypeFactory for playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOWebFontTypeFactory {

    type Output=playground::pax_reexports::pax_std::types::text::WebFont;

    fn build_type(&self, args: &LiteralBlockDefinition, stack_frame: Rc<RuntimePropertiesStackFrame>, table: Rc<ExpressionTable>) -> Self::Output {
        let mut properties: playground::pax_reexports::pax_std::types::text::WebFont = Default::default();
        for setting in &args.elements {
            if let SettingElement::Setting(k, vd) = setting {
                match k.raw_value.as_str() {
                    
                    "family" => {
                        
                            properties.family = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for family")
                                };
                            
                        
                    },
                    
                    "url" => {
                        
                            properties.url = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::std::string::String>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOstdCOCOstringCOCOStringTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for url")
                                };
                            
                        
                    },
                    
                    "style" => {
                        
                            properties.style = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontStyle>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontStyleTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for style")
                                };
                            
                        
                    },
                    
                    "weight" => {
                        
                            properties.weight = 
                            
                                match vd {
                                    ValueDefinition::LiteralValue(lv) => {
                                        from_pax_try_coerce::<playground::pax_reexports::pax_std::types::text::FontWeight>(&lv.raw_value).unwrap()
                                    },
                                    ValueDefinition::Block(block) => {
                                        playgroundCOCOpax_reexportsCOCOpax_stdCOCOtypesCOCOtextCOCOFontWeightTypeFactory{}.build_type(args, stack_frame.clone(), table.clone())
                                    }
                                    _ => unreachable!("Invalid value definition for weight")
                                };
                            
                        
                    },
                    
                    _ => panic!("Unknown property name {}", k.raw_value)
                }
            
            }
        }
        properties
    }
}
        
        

pub struct DefinitionToInstanceTraverser {
    #[cfg(not(feature = "designtime"))]
    manifest: PaxManifest,
    #[cfg(feature = "designtime")]
    designtime_manager: Rc<RefCell<pax_designtime::DesigntimeManager>>,
}

impl DefinitionToInstanceTraverser {

    #[cfg(not(feature = "designtime"))]
    pub fn new() -> Self {
        let manifest: PaxManifest = serde_json::from_str(INITAL_MANIFEST).expect("Failed to deserialize initial manifest");
        Self {
            manifest,
        }
    }

    #[cfg(not(feature = "designtime"))]
    pub fn get_manifest(&self) ->  &PaxManifest {
        &self.manifest
    }

    #[cfg(feature = "designtime")]
    pub fn new() -> Self {
        let manifest: PaxManifest = serde_json::from_str(INITAL_MANIFEST).expect("Failed to deserialize initial manifest");
        let designtime_manager = Rc::new(RefCell::new(pax_designtime::DesigntimeManager::new(manifest)));
        Self {
            designtime_manager,
        }
    }

    #[cfg(feature = "designtime")]
    pub fn get_designtime_manager(&self) -> Rc<RefCell<pax_designtime::DesigntimeManager>> {
        self.designtime_manager.clone()
    }

    #[cfg(feature = "designtime")]
    pub fn get_manifest(&self) ->  Ref<PaxManifest> {
        Ref::map(borrow!(self.designtime_manager), |manager| {
            manager.get_manifest()
        })
    }

    pub fn get_main_component(&mut self) -> Rc<ComponentInstance> {
        let main_component_type_id = {
            let manifest = self.get_manifest();
            manifest.main_component_type_id.clone()
        };
        let args = self.build_component_args(&main_component_type_id);
        let main_component = ComponentInstance::instantiate(args);
        main_component
    }

    pub fn get_component(&mut self, type_id: &TypeId) -> Rc<dyn InstanceNode> {
        let factory = Self::get_component_factory(type_id).expect("Failed to get component factory");
        let args = self.build_component_args(type_id);
        factory.build_component(args)
    }

    pub fn get_component_factory(type_id: &TypeId) -> Option<Box<dyn ComponentFactory>> {
        if type_id.is_blank_component() {
            return Some(Box::new(BlankComponentFactory{}) as Box<dyn ComponentFactory>);
        }

        match type_id.get_unique_identifier().as_str() {
            
            "playground::pax_reexports::Example" => {
                        Some(Box::new(ExampleFactory{}) as Box<dyn ComponentFactory>)
                },
            
            "playground::pax_reexports::pax_std::primitives::BlankComponent" => {
                        Some(Box::new(BlankComponentFactory{}) as Box<dyn ComponentFactory>)
                },
            
            "playground::pax_reexports::pax_std::primitives::Group" => {
                        Some(Box::new(GroupFactory{}) as Box<dyn ComponentFactory>)
                },
            
            "playground::pax_reexports::pax_std::primitives::Rectangle" => {
                        Some(Box::new(RectangleFactory{}) as Box<dyn ComponentFactory>)
                },
            
            "playground::pax_reexports::pax_std::primitives::Text" => {
                        Some(Box::new(TextFactory{}) as Box<dyn ComponentFactory>)
                },
            
            _ => None
        }
    }

    pub fn build_component_args(&self, type_id: &TypeId) -> InstantiationArgs {
        let manifest = self.get_manifest();
        let property_names = manifest.get_all_property_names(type_id);
        if let None = manifest.components.get(type_id) {
            panic!("Components with type_id {} not found in manifest", type_id);
        }
        let component = manifest.components.get(type_id).unwrap();
        let factory = Self::get_component_factory(&type_id).expect(&format!("No component factory for type: {}", type_id));
        let prototypical_common_properties_factory = factory.build_default_common_properties();
        let prototypical_properties_factory = factory.build_default_properties();

        // pull handlers for this component
        let handlers = manifest.get_component_handlers(type_id);
        let handler_registry = Some(factory.build_component_handlers(handlers));

        let mut component_template = None;
        if let Some(template) = &component.template {

            let root = template.get_root();
            let mut instances = Vec::new();
            for node_id in root {
                let node = template.get_node(&node_id).unwrap();
                match node.type_id.get_pax_type(){
                    PaxType::If | PaxType::Slot | PaxType::Repeat => {
                        instances.push(self.build_control_flow(type_id, &node_id));
                    },
                    PaxType::Comment => continue,
                    _ => {  
                        instances.push(self.build_template_node(type_id, &node_id));
                    }
                }
            }
            component_template = Some(RefCell::new(instances));
        }

        InstantiationArgs {
            prototypical_common_properties_factory,
            prototypical_properties_factory,
            handler_registry,
            component_template,
            children: None,
            template_node_identifier: None,
            properties_scope_factory: Some(factory.get_properties_scope_factory()),
        }
    }

    pub fn build_control_flow(&self, containing_component_type_id: &TypeId, node_id: &TemplateNodeId) -> Rc<dyn InstanceNode> {

        let manifest = self.get_manifest();
        let prototypical_common_properties_factory = Box::new(|_,_| Rc::new(RefCell::new(CommonProperties::default())));

        let containing_component = manifest.components.get(containing_component_type_id).unwrap();
        let containing_template = containing_component.template.as_ref().unwrap();
        let tnd = containing_template.get_node(node_id).unwrap();
        let unique_identifier = UniqueTemplateNodeIdentifier::build(containing_component_type_id.clone(), node_id.clone());

        let children = self.build_children(containing_component_type_id, &node_id);
        match tnd.type_id.get_pax_type(){
            PaxType::If => {
                let expression_info = tnd
                    .control_flow_settings
                    .as_ref()
                    .unwrap()
                    .condition_expression_info
                    .as_ref()
                    .unwrap();
                let vtable_id = expression_info.vtable_id;
                let dep_symbols = expression_info.dependencies.clone();
                let prototypical_properties_factory : Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> = Box::new(move |stack_frame, table| Rc::new(RefCell::new( {
                        let mut properties = ConditionalProperties::default();
                        let cloned_table = table.clone();
                        let cloned_stack = stack_frame.clone();

                        let mut dependencies = Vec::new();
                        for dependency in &dep_symbols {
                            if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                dependencies.push(p);
                            } else {
                                panic!("Failed to resolve symbol {}", dependency);
                            }
                        }

                        properties.boolean_expression =  Property::computed_with_name(move || {
                            let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, vtable_id);
                            let coerced = new_value_wrapped.try_coerce::<bool>().map_err(|e| format!("expr with vtable_id {} failed: {}", vtable_id, e)).unwrap();
                            coerced
                        }, &dependencies, "conditional (if) expr");
                        properties.to_pax_any()
                    })));
                ConditionalInstance::instantiate(InstantiationArgs {
                    prototypical_common_properties_factory,
                    prototypical_properties_factory,
                    handler_registry: None,
                    component_template: None,
                    children: Some(RefCell::new(children)),
                    template_node_identifier: Some(unique_identifier),
                    properties_scope_factory: None,
                })
            },
            PaxType::Slot => {
                let expression_info = tnd
                    .control_flow_settings
                    .as_ref()
                    .unwrap()
                    .slot_index_expression_info
                    .as_ref()
                    .unwrap();
                
                let vtable_id = expression_info.vtable_id;
                let dep_symbols = expression_info.dependencies.clone();

                let prototypical_properties_factory : Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>>  = Box::new(move |stack_frame, table| Rc::new(RefCell::new( {
                        let mut properties = SlotProperties::default();
                        let cloned_table = table.clone();
                        let cloned_stack = stack_frame.clone();

                        let mut dependencies = Vec::new();
                        for dependency in &dep_symbols {
                            if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                dependencies.push(p);
                            } else {
                                panic!("Failed to resolve symbol {}", dependency);
                            }
                        }
                        properties.index = Property::computed_with_name(move || {
                            let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, vtable_id);
                            let coerced = new_value_wrapped.try_coerce::<Numeric>().unwrap();
                            coerced
                        }, &dependencies, "slot index");
                        properties.to_pax_any()
                    })));
                SlotInstance::instantiate(InstantiationArgs {
                    prototypical_common_properties_factory,
                    prototypical_properties_factory,
                    handler_registry: None,
                    component_template: None,
                    children: Some(RefCell::new(children)),
                    template_node_identifier: Some(unique_identifier),
                    properties_scope_factory: None,
                })
            },
            PaxType::Repeat => {
                let rsd = tnd
                    .control_flow_settings
                    .as_ref()
                    .unwrap()
                    .repeat_source_definition
                    .clone()
                    .unwrap();
                let rpd = tnd
                    .control_flow_settings
                    .as_ref()
                    .unwrap()
                    .repeat_predicate_definition
                    .clone()
                    .unwrap();
                let expression_info = rsd.expression_info.as_ref().unwrap();
                let vtable_id = expression_info.vtable_id.clone();
                let dep_symbols = expression_info.dependencies.clone();
                let prototypical_properties_factory : Box<dyn Fn(Rc<RuntimePropertiesStackFrame>, Rc<ExpressionTable>) -> Rc<RefCell<PaxAny>>> = Box::new(move |stack_frame,table| Rc::new(RefCell::new( {
                        let mut properties = RepeatProperties::default();

                        let mut dependencies = Vec::new();
                        for dependency in &dep_symbols {
                            if let Some(p) = stack_frame.resolve_symbol_as_erased_property(dependency) {
                                dependencies.push(p);
                            } else {
                                panic!("Failed to resolve symbol {}", dependency);
                            }
                        }

                        properties.source_expression_vec = 
                            if let Some(t) = &rsd.symbolic_binding {
                                let cloned_table = table.clone();
                                let cloned_stack = stack_frame.clone();
                                Some(
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, vtable_id);
                                        let coerced = new_value_wrapped.try_coerce::<Vec<Rc<RefCell<PaxAny>>>>().unwrap();
                                        coerced
                                    }, &dependencies, "repeat source vec")
                                    ) 
                            } else {
                                None
                            };
                            
                        properties.source_expression_range =
                            if let Some(t) = &rsd.range_expression_paxel {
                                let cloned_table = table.clone();
                                let cloned_stack = stack_frame.clone();
                                Some(
                                    Property::computed_with_name(move || {
                                        let new_value_wrapped: PaxAny = cloned_table.compute_vtable_value(&cloned_stack, vtable_id);
                                        let coerced = new_value_wrapped.try_coerce::<std::ops::Range::<isize>>().unwrap();
                                        coerced
                                    }, &dependencies, "repeat source range")
                                )
                            } else {
                                None
                            };

                        let (elem, index) = match &rpd {
                            ElemId(token) => {
                                (Some(token.raw_value.clone()), None)
                            },
                            ElemIdIndexId(t1,t2) => {
                                (Some(t1.raw_value.clone()), Some(t2.raw_value.clone()))
                            }
                        };
                        properties.iterator_i_symbol = index;
                        properties.iterator_elem_symbol = elem;
                        properties.to_pax_any()
                    })));
                RepeatInstance::instantiate(InstantiationArgs {
                    prototypical_common_properties_factory,
                    prototypical_properties_factory,
                    handler_registry: None,
                    component_template: None,
                    children: Some(RefCell::new(children)),
                    template_node_identifier: Some(unique_identifier),
                    properties_scope_factory: None
                })
            },
            _ => {  
                unreachable!("Unexpected control flow type {}", tnd.type_id)
            }
        
        }

    }

    fn build_children(&self, containing_component_type_id: &TypeId, node_id: &TemplateNodeId) -> Vec<Rc<dyn InstanceNode>> {
        let manifest = self.get_manifest();
        let containing_component = manifest.components.get(containing_component_type_id).unwrap();
        let containing_template = containing_component.template.as_ref().unwrap();
        let children = containing_template.get_children(node_id);

        let mut children_instances = Vec::new();
        for child_id in &children.unwrap_or_default() {
            let child = containing_template.get_node(&child_id).unwrap();
            match child.type_id.get_pax_type() {
                PaxType::If | PaxType::Slot | PaxType::Repeat  => {
                    children_instances.push(self.build_control_flow(containing_component_type_id, &child_id));
                },
                PaxType::Comment => continue,
                _ => {  
                    children_instances.push(self.build_template_node(containing_component_type_id, child_id));
                }
            }
        }
        children_instances
    }

    pub fn build_template_node(&self, containing_component_type_id: &TypeId, node_id: &TemplateNodeId) -> Rc<dyn InstanceNode> {
        let manifest = self.get_manifest();

        let containing_component = manifest.components.get(containing_component_type_id).unwrap();
        let containing_template = containing_component.template.as_ref().unwrap();
        let node = containing_template.get_node(node_id).unwrap();
        let containing_component_factory = Self::get_component_factory(containing_component_type_id).unwrap();

        let mut args = self.build_component_args(&node.type_id);
        let node_component_factory = Self::get_component_factory(&node.type_id).unwrap();

        // update handlers from tnd
        let handlers_from_tnd = manifest.get_inline_event_handlers(node);
        let updated_registry = if let Some(registry) = args.handler_registry {
            containing_component_factory.add_inline_handlers(handlers_from_tnd, registry)    
        } else {
            containing_component_factory.add_inline_handlers(handlers_from_tnd, Rc::new(RefCell::new(HandlerRegistry::default())))       
        };

        args.handler_registry = Some(updated_registry);

        // update properties from tnd 
        let inline_properties = manifest.get_inline_properties(containing_component_type_id, node);
        let updated_properties = node_component_factory.build_inline_properties(inline_properties.clone());
        args.prototypical_properties_factory = updated_properties;

        // update common properties from tnd
        let updated_common_properties = node_component_factory.build_inline_common_properties(inline_properties);
        args.prototypical_common_properties_factory = updated_common_properties;

       
        args.children = Some(RefCell::new(self.build_children(containing_component_type_id, node_id)));
        args.template_node_identifier = Some(UniqueTemplateNodeIdentifier::build(containing_component_type_id.clone(), node_id.clone()));

        node_component_factory.build_component(args)
    }


    pub fn get_template_node_by_id(&self, id: &str) -> Option<Rc<dyn InstanceNode>> {
        let manifest = self.get_manifest();
        let main_component_type_id = manifest.main_component_type_id.clone();
        let main_component = manifest.components.get(&main_component_type_id).unwrap();
        let template = main_component.template.as_ref().unwrap();
        for node_id in template.get_ids() {
            if let Some(found) = self.recurse_get_template_node_by_id(id, &main_component_type_id, node_id) {
                return Some(self.build_template_node(&found.0, &found.1))
            }
        }
        None
    }

    fn check_for_id_in_template_node(&self, id: &str, tnd: &TemplateNodeDefinition) -> bool {
        if let Some(settings) = &tnd.settings {
            for setting in settings {
                if let SettingElement::Setting(token, value) = setting {
                    if &token.raw_value == "id" {
                        if let ValueDefinition::LiteralValue(lv) = value {
                            if lv.raw_value == id {
                                return true;
                            }
                        }
                    
                    }
                }
            }
        }
        false
    }

    fn recurse_get_template_node_by_id<'a>(&'a self, id: &str, containing_component_type_id: &'a TypeId, node_id: &'a TemplateNodeId) -> Option<(TypeId, TemplateNodeId)>{
        let manifest = self.get_manifest();
        let containing_component = manifest.components.get(containing_component_type_id).unwrap();
        let containing_template = containing_component.template.as_ref().unwrap();
        let tnd = containing_template.get_node(node_id).unwrap();

        if self.check_for_id_in_template_node(id, tnd) {
            return Some((containing_component_type_id.clone(), node_id.clone()));
        }

        if let Some(component) = &manifest.components.get(&tnd.type_id){
            if let Some(template) = &component.template {
                for node_id in template.get_ids() {
                    if let Some(found) = self.recurse_get_template_node_by_id(id, &tnd.type_id, node_id) {
                        return Some(found.clone());
                    }
                }
            }
        }
        None
    }
}