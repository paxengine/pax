
use piet_web::{WebRenderContext};
use piet::{Color, StrokeStyle, RenderContext};
use kurbo::{Affine, BezPath};

use crate::{Variable, Property, PropertyTreeContext, SceneGraphContext, StackFrame};
use std::rc::Rc;
use std::cell::{RefCell, Ref};



pub type RenderNodePtr = Rc<RefCell<dyn RenderNode>>;
pub type RenderNodePtrList = Rc<RefCell<Vec<RenderNodePtr>>>;

pub fn wrap_render_node_ptr_into_list(rnp: RenderNodePtr) -> RenderNodePtrList {
    Rc::new(RefCell::new(vec![Rc::clone(&rnp)]))
}

pub struct SceneGraph {
    pub root: RenderNodePtr
    // pub call_stack: Vec<StackFrame<'a>>
}

impl SceneGraph {

}

pub struct Runtime {
    stack: Vec<Rc<RefCell<StackFrame>>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: Vec::new(),
        }
    }
    // pub fn peek_stack_frame(&self) -> StackFrame {
    //     // &self.stack[0]
    // }

    pub fn peek_stack_frame(&mut self) -> Option<Rc<RefCell<StackFrame>>> {
        if self.stack.len() > 0 {
            Some(Rc::clone(&self.stack[0]))
        }else{
            None
        }
    }
    pub fn pop_stack_frame(&mut self) {
        self.stack.pop();
        // self.adoptee_index
    }
    pub fn push_stack_frame(&mut self, adoptees: RenderNodePtrList) {

        //TODO:  manage iterator:
        //  - either an internal int counter, or
        //  - figure out a with-the-grain way to store an iter internally & expose via push_stack_frame/pop_stack_frame/peek_stack_frame

        // let x = adoptees.borrow();
        // let x_iter = x.iter();
        // x_iter.ne
        //
        // self.stack.push(StackFrame::new() {
        //     adoptee_iter: Box::new(x_iter)
        // });
    }
    // pub fn get_next_adoptee_for_current_stack_frame(&self) {
    //     self.peek_stack_frame().adoptees
    // }
}

pub trait RenderNode
{
    fn eval_properties_in_place(&mut self, ctx: &PropertyTreeContext);
    fn get_align(&self) -> (f64, f64);
    fn get_children(&self) -> RenderNodePtrList;
    fn get_size(&self) -> Option<(Size<f64>, Size<f64>)>;

    /// Returns the size of this node in pixels, requiring
    /// parent bounds for calculation of `Percent` values
    fn get_size_calc(&self, bounds: (f64, f64)) -> (f64, f64);

    fn get_id(&self) -> &str;
    fn get_origin(&self) -> (Size<f64>, Size<f64>);
    fn get_transform(&self) -> &Affine;
    fn pre_render(&mut self, sc: &mut SceneGraphContext);
    fn render(&self, sc: &mut SceneGraphContext, rc: &mut WebRenderContext);
    fn post_render(&self, sc: &mut SceneGraphContext);
}

pub struct Group {
    pub children: Rc<RefCell<Vec<RenderNodePtr>>>,
    pub id: String,
    pub align: (f64, f64),
    pub origin: (Size<f64>, Size<f64>),
    pub transform: Affine,
    pub variables: Vec<Variable>,
}

impl RenderNode for Group {
    fn eval_properties_in_place(&mut self, _: &PropertyTreeContext) {
        //TODO: handle each of Group's `Expressable` properties
    }

    fn get_align(&self) -> (f64, f64) { self.align }
    fn get_children(&self) -> RenderNodePtrList {
        Rc::clone(&self.children)
    }
    fn get_size(&self) -> Option<(Size<f64>, Size<f64>)> { None }
    fn get_size_calc(&self, bounds: (f64, f64)) -> (f64, f64) { bounds }
    fn get_id(&self) -> &str {
        &self.id.as_str()
    }
    fn get_origin(&self) -> (Size<f64>, Size<f64>) { self.origin }
    fn get_transform(&self) -> &Affine {
        &self.transform
    }
    fn pre_render(&mut self, _sc: &mut SceneGraphContext) {}
    fn render(&self, _sc: &mut SceneGraphContext, _rc: &mut WebRenderContext) {}
    fn post_render(&self, _sc: &mut SceneGraphContext) {}
}

pub struct Stroke {
    pub color: Color,
    pub width: f64,
    pub style: StrokeStyle,
}


#[derive(Copy, Clone)]
pub enum Size<T> {
    Pixel(T),
    Percent(T),
}

pub struct Rectangle {
    pub align: (f64, f64),
    pub size: (
        Box<dyn Property<Size<f64>>>,
        Box<dyn Property<Size<f64>>>,
    ),
    pub origin: (Size<f64>, Size<f64>),
    pub transform: Affine,
    pub stroke: Stroke,
    pub fill: Box<dyn Property<Color>>,
    pub id: String,
}


impl RenderNode for Rectangle {
    fn get_align(&self) -> (f64, f64) { self.align }
    fn get_children(&self) -> RenderNodePtrList {
        Rc::new(RefCell::new(vec![]))
    }
    fn eval_properties_in_place(&mut self, ctx: &PropertyTreeContext) {
        self.size.0.eval_in_place(ctx);
        self.size.1.eval_in_place(ctx);
        self.fill.eval_in_place(ctx);
    }
    fn get_origin(&self) -> (Size<f64>, Size<f64>) { self.origin }
    fn get_size(&self) -> Option<(Size<f64>, Size<f64>)> { Some((*self.size.0.read(), *self.size.1.read())) }
    fn get_size_calc(&self, bounds: (f64, f64)) -> (f64, f64) {
        let size_raw = self.get_size().unwrap();
        return (
            match size_raw.0 {
                Size::Pixel(width) => {
                    width
                },
                Size::Percent(width) => {
                    bounds.0 * (width / 100.0)
                }
            },
            match size_raw.1 {
                Size::Pixel(height) => {
                    height
                },
                Size::Percent(height) => {
                    bounds.1 * (height / 100.0)
                }
            }
        )
    }
    fn get_transform(&self) -> &Affine {
        &self.transform
    }
    fn get_id(&self) -> &str {
        &self.id.as_str()
    }
    fn pre_render(&mut self, _sc: &mut SceneGraphContext) {}
    fn render(&self, sc: &mut SceneGraphContext, rc: &mut WebRenderContext) {

        let transform = sc.transform;
        let bounding_dimens = sc.bounding_dimens;
        let width: f64 =  bounding_dimens.0;
        let height: f64 =  bounding_dimens.1;

        let fill: &Color = &self.fill.read();

        let mut bez_path = BezPath::new();
        bez_path.move_to((0.0, 0.0));
        bez_path.line_to((width , 0.0));
        bez_path.line_to((width , height ));
        bez_path.line_to((0.0, height));
        bez_path.line_to((0.0,0.0));
        bez_path.close_path();

        let transformed_bez_path = *transform * bez_path;
        let duplicate_transformed_bez_path = transformed_bez_path.clone();

        rc.fill(transformed_bez_path, fill);
        rc.stroke(duplicate_transformed_bez_path, &self.stroke.color, self.stroke.width);
    }
    fn post_render(&self, _sc: &mut SceneGraphContext) {}
}

pub struct Yield {
    pub id: String,
    pub transform: Affine,
    children: RenderNodePtrList,
}

impl RenderNode for Yield {
    fn eval_properties_in_place(&mut self, _: &PropertyTreeContext) {
        //TODO: handle each of Group's `Expressable` properties
    }

    fn get_align(&self) -> (f64, f64) { (0.0,0.0) }
    fn get_children(&self) -> RenderNodePtrList {
        //TODO: return adoptee via iterator from stack frame
        // Some(&self.children)
        Rc::clone(&self.children)
    }
    fn get_size(&self) -> Option<(Size<f64>, Size<f64>)> { None }
    fn get_size_calc(&self, bounds: (f64, f64)) -> (f64, f64) { bounds }
    fn get_id(&self) -> &str {
        &self.id.as_str()
    }
    fn get_origin(&self) -> (Size<f64>, Size<f64>) { (Size::Pixel(0.0), Size::Pixel(0.0)) }
    fn get_transform(&self) -> &Affine {
        &self.transform
    }
    fn pre_render(&mut self, sc: &mut SceneGraphContext) {
        // grab the first adoptee from the current stack frame
        // and make it Yield's own child.
        //
        // this might be more elegant as a dynamic lookup inside the get_children
        // method, but at the time of authoring that would require refactoring
        // get_children to accept the SceneGraphContext.  This felt better at the time.
        self.children = match sc.runtime.borrow_mut().peek_stack_frame() {
            Some(stack_frame) => {
                match stack_frame.borrow_mut().next_adoptee() {
                    Some(adoptee) => {
                        wrap_render_node_ptr_into_list(adoptee)
                    },
                    None => {Rc::new(RefCell::new(vec![]))}
                }
            },
            None => {Rc::new(RefCell::new(vec![]))}
        }
    }
    fn render(&self, _sc: &mut SceneGraphContext, _rc: &mut WebRenderContext) {}
    fn post_render(&self, _sc: &mut SceneGraphContext) {}
}

pub struct Repeat {

}

pub struct If {

}

