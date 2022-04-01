use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Mul;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use mut_static::MutStatic;


pub struct TransitionQueueEntry<T> {
    pub global_frame_started: Option<usize>,
    pub duration_frames: usize,
    pub curve: EasingCurve,
    pub starting_value: T,
    pub ending_value: T,
}
/// An abstract Property that may be either: Literal,
/// a dynamic runtime Expression, or a Timeline-bound value
pub trait PropertyInstance<T: Default + Clone> {
    fn get(&self) -> &T;
    fn _get_vtable_id(&self) -> Option<&str>;

    fn set(&mut self, value: T);

    fn _get_transition_queue_mut(&mut self) -> Option<&mut VecDeque<TransitionQueueEntry<T>>>;

    //TODO: when trait fields land, DRY this implementation vs. other <T: PropertyInstance> implementations
    fn ease_to(&mut self, new_value: T, duration_frames: usize, curve: EasingCurve);

    fn ease_to_later(&mut self, new_value: T, duration_frames: usize, curve: EasingCurve);

}


impl<T: Default + Clone + 'static> Default for Box<dyn PropertyInstance<T>> {
    fn default() -> Box<dyn PropertyInstance<T>> {
        Box::new(PropertyLiteral::new(Default::default()))
    }
}

impl<T: Default + Clone + 'static> Clone for Box<dyn PropertyInstance<T>> {
    fn clone(&self) -> Self {
        Box::clone(self)
    }
}

//keep an eye on perf. here — might be more sensible to use something like
//a monotonically increasing counter of i32 instead of String UUIDs.  Might require coordinating between
//code-genned IDs in code-gen and dynamically generated IDs here to avoid dupes.
pub fn mint_unique_id() -> String {
    Uuid::new_v4().to_string()
}

pub enum ArgsCoproduct {
    Render(ArgsRender),
    Click(ArgsClick),
}

pub type Property<T> = Box<dyn PropertyInstance<T>>;

#[derive(Clone)]
pub struct ArgsRender {
    /// The current global engine tick count
    pub frames_elapsed: usize,
    /// The bounds of this element's container in px
    pub bounds: (f64, f64),
    // /// The number of adoptees passed to the current component (used by Spread for auto cell-count calc; might be extended/adjusted for other use-cases)
    // pub adoptee_count: usize,
}

#[derive(Clone)]
pub struct ArgsClick {
    x: f64,
    y: f64,
}


/// A Size value that can be either a concrete pixel value
/// or a percent of parent bounds.  Note that this may be more precisely
/// called a Dimension or a SizeDimension, but Size was initially chosen for brevity.
#[derive(Copy, Clone)]
pub enum Size {
    Pixel(f64),
    Percent(f64),
}

pub struct Logger(fn(&str));

lazy_static! {
    static ref LOGGER: MutStatic<Logger> = MutStatic::new();
}

pub fn register_logger(logger: fn(&str)) {
    LOGGER.borrow().set(Logger(logger)).unwrap();
}

pub fn log(msg: &str) {
    (LOGGER.borrow().read().expect("TODO: handle case where logger isn't registered").0)(msg);
}

impl Mul for Size {
    type Output = Size;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Size::Pixel(px0) => {
                match rhs {
                    //multiplying two pixel values adds them,
                    //in the sense of multiplying two affine translations.
                    //this might be wildly unexpected in some cases, so keep an eye on this and
                    //revisit whether to support Percent values in origin calcs (could rescind)
                    Size::Pixel(px1) => {
                        Size::Pixel(px0 + px1)
                    }
                    Size::Percent(pc1) => {
                        Size::Pixel(px0 * pc1)
                    }
                }
            }
            Size::Percent(pc0) => {
                match rhs {
                    Size::Pixel(px1) => {
                        Size::Pixel(pc0 * px1)
                    }
                    Size::Percent(pc1) => {
                        Size::Percent(pc0 * pc1)
                    }
                }
            }
        }
    }
}


/// TODO: revisit if 100% is the most ergonomic default size (remember Dreamweaver)
impl Default for Size {
    fn default() -> Self {
        Self::Percent(100.0)
    }
}

pub struct TransformInstance {
    rotate: Option<Box<dyn PropertyInstance<f64>>>
}

// More than just a tuble of (Size, Size),
// Size2D wraps up Properties as well to make it easy
// to declare expressable Size properties
pub type Size2D = Rc<RefCell<[Box<dyn PropertyInstance<Size>>; 2]>>;




/// A sugary representation of an Affine transform+, including
/// `origin` and `align` as layout-computed properties.
///
/// `translate` represents an (x,y) affine translation
/// `scale`     represents an (x,y) non-uniform affine scale
/// `rotate`    represents a (z) affine rotation (intuitive 2D rotation)
/// `origin`    represents the "(0,0)" point of the render node as it relates to its own bounding box.
///             By default that's the top-left of the element, but `origin` allows that
///             to be offset either by a pixel or percentage-of-element-size
///             for each of (x,y)
/// `align`     the offset of this element's `origin` as it relates to the element's parent.
///             By default this is the top-left corner of the parent container,
///             but can be set to be any value [0,1] for each of (x,y), representing
///             the percentage (between 0.0 and 1.0) multiplied by the parent container size.
///             For example, an align of (0.5, 0.5) will center an element's `origin` point both vertically
///             and horizontally within the parent container.  Combined with an origin of (Size::Percent(50.0), Size::Percent(50.0)),
///             an element will appear fully centered within its parent.
#[derive(Default, Clone)]
pub struct Transform2D { //Literal
    pub previous: Option<Box<Transform2D>>,
    pub rotate: Option<f64>, ///over z axis
    pub translate: Option<[f64; 2]>,
    pub origin: Option<[Size; 2]>,
    pub align: Option<[Size; 2]>,
    pub scale: Option<[f64; 2]>,
}

impl Mul for Transform2D {
    type Output = Transform2D;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret = rhs.clone();
        ret.previous = Some(Box::new(self));
        ret
    }
}

impl Transform2D {
    ///Scale coefficients (1.0 == 100%) over x-y plane
    pub fn scale(x: f64, y: f64) -> Self {
        let mut ret  = Transform2D::default();
        ret.scale = Some([x, y]);
        ret
    }
    ///Rotation over z axis
    pub fn rotate(z: f64) -> Self {
        let mut ret  = Transform2D::default();
        ret.rotate = Some(z);
        ret
    }
    ///Translation across x-y plane, pixels
    pub fn translate(x: f64, y: f64) -> Self {
        let mut ret  = Transform2D::default();
        ret.translate = Some([x, y]);
        ret
    }
    ///Describe alignment within parent bounding box, as a starting point before
    /// affine transformations are applied
    pub fn align(x: Size, y: Size) -> Self {
        let mut ret  = Transform2D::default();
        ret.align = Some([x, y]);
        ret
    }
    ///Describe alignment of the (0,0) position of this element as it relates to its own bounding box
    pub fn origin(x: Size, y: Size) -> Self {
        let mut ret  = Transform2D::default();
        ret.origin = Some([x, y]);
        ret
    }

    pub fn default_wrapped() -> Rc<RefCell<dyn PropertyInstance<Self>>> {
        Rc::new(RefCell::new(PropertyLiteral::new(Transform2D::default())))
    }
}



pub struct TransitionManager<T> {
    pub queue: VecDeque<TransitionQueueEntry<T>>,
    value: Option<T>,
}

impl<T> TransitionManager<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            value: None,
        }
    }
}

/// The Literal form of a Property: a bare literal value
pub struct PropertyLiteral<T> {
    value: T,
    transition_manager: TransitionManager<T>,
}


impl<T> Into<Box<dyn PropertyInstance<T>>> for PropertyLiteral<T>
where T: Default + Clone + 'static {
    fn into(self) -> Box<dyn PropertyInstance<T>> {
        Box::new(self)
    }
}


impl<T: Clone> PropertyLiteral<T> {
    pub fn new(value: T) -> Self {
        PropertyLiteral {
            value,
            transition_manager: TransitionManager::new(),
        }
    }


}
impl<T: Default + Clone> PropertyInstance<T> for PropertyLiteral<T> {
    fn get(&self) -> &T {
        &self.value
    }

    fn _get_vtable_id(&self) -> Option<&str> {
        None
    }

    // fn is_fresh(&self) -> bool {
    //     //TODO: should probably return true for the first frame that this Property exists.
    //     //Perhaps turn PropertyLiteral into a two-tuple (v,true)
    // }
    //
    // fn _mark_not_fresh(&mut self) {
    //     //no-op
    // }

    fn set(&mut self, value: T) {
        self.value = value;
    }



    //TODO: when trait fields land, DRY this implementation vs. other <T: PropertyInstance> implementations
    fn ease_to(&mut self, new_value: T, duration_frames: usize, curve: EasingCurve) {
        &self.transition_manager.queue.clear();
        &self.transition_manager.queue.push_back(TransitionQueueEntry {
            global_frame_started: None,
            duration_frames,
            curve,
            starting_value: self.value.clone(),
            ending_value: new_value
        });
    }

    fn ease_to_later(&mut self, new_value: T, duration_frames: usize, curve: EasingCurve) {
        &self.transition_manager.queue.push_back(TransitionQueueEntry {
            global_frame_started: None,
            duration_frames,
            curve,
            starting_value: self.value.clone(),
            ending_value: new_value
        });
    }

    fn _get_transition_queue_mut(&mut self) -> Option<&mut VecDeque<TransitionQueueEntry<T>>> {
        Some(&mut self.transition_manager.queue)
    }
}

pub enum EasingCurve {
    Linear,
    InQuad,
    OutQuad,
    InBack,
    OutBack,
    InOutBack,
    Custom(Box<dyn Fn(f64) -> f64>),
}

struct EasingEvaluators {}
impl EasingEvaluators {
    fn linear(t: f64) -> f64 {
        t
    }
    fn none(t: f64) -> f64 {
        if t == 1.0 { 1.0 } else { 0.0 }
    }
    fn in_quad(t: f64) -> f64 {
        t * t
    }
    fn out_quad(t: f64) -> f64 {
        1.0 - (1.0 - t) * (1.0 - t)
    }
    fn in_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.00;
        C3 * t * t * t - C1 * t * t
    }
    fn out_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C3: f64 = C1 + 1.00;
        1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
    }

    fn in_out_back(t: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C2 : f64 = C1 * 1.525;
        if t < 0.5 {
            ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
        }
    }
}

impl EasingCurve {
    //for a time on the unit interval `t ∈ [0,1]`, given a value `t`,
    // find the interpolated value `vt` between `v0` and `v1` given the self-contained easing curve
    pub fn interpolate<T: Interpolatable>(&self, v0: T, v1: T, t: f64) -> T /*vt*/ {
        let multiplier = match self {
            EasingCurve::Linear => {
                EasingEvaluators::linear(t)
            }
            EasingCurve::InQuad => {
                EasingEvaluators::in_quad(t)
            }
            EasingCurve::OutQuad => {
                EasingEvaluators::out_quad(t)
            }
            EasingCurve::InBack => {
                EasingEvaluators::in_back(t)
            }
            EasingCurve::OutBack => {
                EasingEvaluators::out_back(t)
            }
            EasingCurve::InOutBack => {
                EasingEvaluators::in_out_back(t)
            }
            EasingCurve::Custom(evaluator) => {
                (*evaluator)(t)
            }
        };

        v0.interpolate( v1, multiplier)
    }
}

pub trait Interpolatable
where Self : Sized + Clone //Clone used for default implementation of `interpolate`
{
    //default implementation acts like a `None` ease — that is,
    //the first value is simply returned.
    fn interpolate(&self, other: Self, t: f64) -> Self {
        self.clone()
    }
}

impl Interpolatable for f64 {
    fn interpolate(&self, other: f64, t: f64) -> f64 {
        self + (other - self) * t
    }
}


pub struct Timeline {
    pub playhead_position: usize,
    pub frame_count: usize,
    pub is_playing: bool,
}
