#![allow(unused_imports)]

pub mod path_animation;
use path_animation::PathAnimation;

use pax_kit::*;

use pax_kit::pax_engine::math::Generic;
use pax_kit::pax_engine::math::Point2;

#[pax]
#[main]
#[file("lib.pax")]
pub struct Example {
    pub scroll: Property<f64>,
}

impl Example {
    pub fn on_mount(&mut self, ctx: &NodeContext) {}
    pub fn on_mouse_move(&mut self, ctx: &NodeContext, event: Event<MouseMove>) {
        let (_, h) = ctx.bounds_self.get();
        let part = event.mouse.y / h;
        self.scroll.set(part);
    }
}
