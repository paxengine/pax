use std::cell::RefCell;
use std::rc::Rc;

use crate::{DevAppRootProperties, RenderNodePtrList, SpreadCellProperties, SpreadProperties};
use crate::primitives::repeat::RepeatItem;
use crate::timeline::Timeline;

/// `Runtime` is a container for data and logic needed by the `Engine`,
/// explicitly aside from rendering.  For example, this is a home
/// for logic that manages scopes, stack frames, and properties.
pub struct Runtime {
    stack: Vec<Rc<RefCell<StackFrame>>>,
    logger: fn(&str),
}

impl Runtime {
    pub fn new(logger: fn(&str)) -> Self {
        Runtime {
            stack: Vec::new(),
            logger,
        }
    }

    pub fn log(&self, message: &str) {
        (&self.logger)(message);
    }

    /// Return a pointer to the top StackFrame on the stack,
    /// without mutating the stack or consuming the value
    pub fn peek_stack_frame(&mut self) -> Option<Rc<RefCell<StackFrame>>> {
        if self.stack.len() > 0 {
            Some(Rc::clone(&self.stack[&self.stack.len() - 1]))
        }else{
            None
        }
    }

    /// Remove the top element from the stack.  Currently does
    /// nothing with the value of the popped StackFrame.
    pub fn pop_stack_frame(&mut self){
        self.stack.pop(); //TODO: handle value here if needed
    }

    /// Add a new frame to the stack, passing a list of adoptees
    /// that may be handled by `Placeholder` and a scope that includes
    pub fn push_stack_frame(&mut self, adoptees: RenderNodePtrList, scope: Box<Scope>, timeline: Option<Rc<RefCell<Timeline>>>) {

        let parent = self.peek_stack_frame();

        self.stack.push(
            Rc::new(RefCell::new(
                StackFrame::new(adoptees, Rc::new(RefCell::new(*scope)), parent, timeline)
            ))
        );
    }

}


/// `Scope` attaches to stack frames to provide an evaluation context + relevant data access
/// for features like Expressions.
///
/// The stored values that are DI'ed into expressions are held in these scopes,
/// e.g. `index` and `datum` for `Repeat`.
pub struct Scope {
    pub properties: Rc<RefCell<PropertiesCoproduct>>,
    // TODO: children, parent, etc.
}

/// ∐
/// This data structure represents all of the Component Properties that
/// exist in an application.  It's likely this will need to be removed
/// from this file and a.) maintained manually as a "manifest", or b.) generated by macro
///
/// This enum/coproduct structure is used to solve the problem of knowing
/// the amount of memory to allocate for `PropertiesCoproduct`s on stack frames.
/// Because our components are polymoprhic (i.e. each component can have
/// a different 'shape' of Properties,) and because stack frames are stored
/// in a central data structure (the runtime stack,) we need a means of
/// storing them together.  Generics + traits don't work because we
/// need concrete access to struct fields, vs. traits which give us methods only.
///
/// Keep in mind that each PropertiesCoproduct type will have the memory footprint
/// of the LARGEST type associated.  Even an instance of `Empty` will have the memory footprint
/// of `TheMostBloatedTypeEver`, so be judicious about what gets stored in PropertiesCoproduct
/// structs (e.g. avoid binary assets like images/multimedia!)
pub enum PropertiesCoproduct {
    DevAppRoot(Rc<RefCell<DevAppRootProperties>>),
    RepeatItem(Rc<RefCell<RepeatItem>>),
    Spread(Rc<RefCell<SpreadProperties>>),
    SpreadCell(Rc<SpreadCellProperties>),
    Empty,
}

/// Data structure for a single frame of our runtime stack, including
/// a reference to its parent frame, a list of `adoptees` for
/// prospective [`Placeholder`] consumption, and a `Scope` for
/// runtime evaluation, e.g. of Expressions.  StackFrames also track
/// timeline playhead position.
pub struct StackFrame
{
    adoptees: RenderNodePtrList,
    scope: Rc<RefCell<Scope>>,
    parent: Option<Rc<RefCell<StackFrame>>>,
    timeline: Option<Rc<RefCell<Timeline>>>,
}

impl StackFrame {
    pub fn new(adoptees: RenderNodePtrList, scope: Rc<RefCell<Scope>>, parent: Option<Rc<RefCell<StackFrame>>>, timeline: Option<Rc<RefCell<Timeline>>>) -> Self {
        StackFrame {
            adoptees: Rc::clone(&adoptees),
            scope,
            parent,
            timeline,
        }
    }

    pub fn get_timeline_playhead_position(&self) -> usize {
        match &self.timeline {
            None => {
                //if this stackframe doesn't carry a timeline, then refer
                //to the parent stackframe's timeline (and recurse)
                match &self.parent {
                    Some(parent_frame) => {
                        parent_frame.borrow().get_timeline_playhead_position()
                    },
                    None => 0
                }
            },
            Some(timeline) => {
                timeline.borrow().playhead_position
            }
        }
    }

    pub fn has_adoptees(&self) -> bool {
        self.adoptees.borrow().len() > 0
    }

    /// Returns the adoptees attached to this stack frame, if present.
    /// Otherwise, recurses up the stack return ancestors' adoptees if found
    /// TODO:  if this logic is problematic, e.g. descendants are grabbing ancestors' adoptees
    ///        inappropriately, then we could adjust this logic to:
    ///        grab direct parent's adoptees instead of current node's,
    ///        but only if current node is a `should_flatten` node like `Repeat`
    pub fn get_adoptees(&self) -> RenderNodePtrList {
        if self.has_adoptees() {
            Rc::clone(&self.adoptees)
        }else {
            match &self.parent {
                Some(parent_frame) => {
                    parent_frame.borrow().get_adoptees()
                },
                None => Rc::new(RefCell::new(vec![]))
            }
        }

    }

    pub fn get_scope(&self) -> Rc<RefCell<Scope>> {
        Rc::clone(&self.scope)
    }
}
