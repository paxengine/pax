#![allow(unused_imports)]
use pax_kit::*;

#[pax]
#[main]
#[inlined(
<Text text={message} class=centered class=small id=text/>
<Rectangle class=centered class=small @click=self.increment fill={rgb(ticks, 75, 150)} 
    corner_radii={RectangleCornerRadii::radii(10.00, 10.00, 10.00, 10.00)}/>

@settings {
    @mount: handle_mount
    @pre_render: handle_pre_render
    .centered {
        x: 50%
        y: 50%
        anchor_x: 50%
        anchor_y: 50%
    }
    .small {
        width: 120px
        height: 120px
    }
    #text {
        style: {
            font: {Font::Web("Times New Roman", "", FontStyle::Normal, FontWeight::Bold)}
            font_size: 32px
            fill: {rgba(1.00, 1.00, 1.00, 1.00)}
            align_vertical: TextAlignVertical::Center
            align_horizontal: TextAlignHorizontal::Center
            align_multiline: TextAlignHorizontal::Center
        }
    }
}
)]
pub struct Example {
    pub ticks: Property<usize>,
    pub num_clicks: Property<usize>,
    pub message: Property<String>,
}

impl Example {
    pub fn handle_mount(&mut self, ctx: &NodeContext) {
        self.message.set("Click me".to_string());
    }
    pub fn handle_pre_render(&mut self, ctx: &NodeContext) {
        let old_ticks = self.ticks.get();
        self.ticks.set(old_ticks + 1);
    }

    pub fn increment(&mut self, ctx: &NodeContext, args: Event<Click>){
        let old_num_clicks = self.num_clicks.get();
        self.num_clicks.set(old_num_clicks + 1);
        self.message.set(format!("{} clicks", self.num_clicks.get()));
    }
}
