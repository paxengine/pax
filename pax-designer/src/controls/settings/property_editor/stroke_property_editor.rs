use std::rc::Rc;

use pax_engine::api::{pax_value::ToFromPaxAny, *};
use pax_engine::*;

use crate::controls::settings::AREAS_PROP;

use super::PropertyEditorData;
use crate::controls::settings::color_picker::ColorPicker;

use pax_engine::api::Stroke;
use pax_std::*;

#[pax]
#[file("controls/settings/property_editor/stroke_property_editor.pax")]
pub struct StrokePropertyEditor {
    pub data: Property<PropertyEditorData>,

    // All the below props should be private: never set by user, used for internal state
    pub stroke: Property<Stroke>,
    pub stroke_width_text: Property<String>,
    pub stroke_width: Property<f64>,
    pub color: Property<Color>,
    pub external: Property<bool>,
    pub property_listener: Property<bool>,
}

impl StrokePropertyEditor {
    pub fn on_mount(&mut self, ctx: &NodeContext) {
        let index = self.data.get().editor_index;
        if index != 0 {
            AREAS_PROP.with(|areas| {
                areas.update(|areas| {
                    while areas.len() <= index {
                        areas.push(0.0)
                    }
                    areas[index - 1] = 40.0;
                });
            });
        }
        let data = self.data.clone();
        let deps = [data.untyped()];
        let ctxc = ctx.clone();
        let external = self.external.clone();
        self.stroke.replace_with(Property::computed(
            move || {
                external.set(true);
                let value = pax_engine::pax_lang::from_pax(&data.get().get_value_as_str(&ctxc));
                if let Ok(value) = value {
                    let stroke: Stroke = Stroke::try_coerce(value)
                        .unwrap_or_default();
                    return stroke
                }
                Stroke::default()
            },
            &deps,
        ));

        let deps = [self.stroke.untyped()];
        let stroke = self.stroke.clone();
        self.color
            .replace_with(Property::computed(move || stroke.get().color.get(), &deps));

        let stroke = self.stroke.clone();
        self.stroke_width.replace_with(Property::computed(
            move || stroke.get().width.get().expect_pixels().to_float(),
            &deps,
        ));

        let stroke_width = self.stroke_width.clone();
        let deps = [stroke_width.untyped()];
        self.stroke_width_text.replace_with(Property::computed(
            move || format!("{:.1}", stroke_width.get()),
            &deps,
        ));

        let color = self.color.clone();
        let stroke_width = self.stroke_width.clone();
        let deps = [color.untyped(), stroke_width.untyped()];
        let external = self.external.clone();
        let data = self.data.clone();
        let ctxc = ctx.clone();
        self.property_listener.replace_with(Property::computed(
            move || {
                let color = color.get();
                let stroke_width = stroke_width.get();
                if !external.get() {
                    let rgba = color.to_rgba_0_1();
                    let col_str = format!(
                        "rgba({}, {}, {}, {})",
                        (rgba[0] * 255.0) as u8,
                        (rgba[1] * 255.0) as u8,
                        (rgba[2] * 255.0) as u8,
                        (rgba[3] * 255.0) as u8
                    );
                    let stroke_str =
                        format!("{{color: {} width: {:.1}px }}", col_str, stroke_width);
                    if let Err(e) = data.get().set_value(&ctxc, &stroke_str) {
                        log::warn!("failed to set fill color: {e}");
                    }
                }
                external.set(false);
                true
            },
            &deps,
        ));
    }

    pub fn width_change(&mut self, _ctx: &NodeContext, event: Event<TextboxChange>) {
        if let Ok(num) = event.text.parse() {
            self.stroke_width.set(num)
        } else {
            log::warn!("can't set stroke: {:?} is not a number", event.text);
        }
    }

    pub fn pre_render(&mut self, _ctx: &NodeContext) {
        self.property_listener.get();
    }
}