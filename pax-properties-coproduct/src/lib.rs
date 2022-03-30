use std::cell::RefCell;
use std::rc::Rc;

/// ∐
/// This data structure represents all of the Component Properties that
/// exist in an application.
///
/// This enum/coproduct structure solves the problem of knowing
/// the amount of memory to allocate for `PropertiesCoproduct`s on stack frames.
/// Because our components can have different Property "schemas," and because stack frames are stored
/// in a central data structure (the runtime stack,) we run into an issue storingneed a means of
/// storing them together.  Generics + traits don't work because we
/// need concrete access to struct fields, vs. traits which give us methods only.
///
/// Keep in mind that each PropertiesCoproduct type will have the memory footprint
/// of the LARGEST type associated.  Even an instance of `Empty` will have the memory footprint
/// of `TheMostBloatedTypeEver`, so be judicious about what gets stored in PropertiesCoproduct
/// structs (e.g. be wary of binary assets like images/multimedia!)
///
pub enum PropertiesCoproduct {
    //core
    None,
    RepeatList(Vec<Rc<RefCell<PropertiesCoproduct>>>),
    RepeatItem(Rc<PropertiesCoproduct>, usize),
    //generated
    isize(isize), //used by range for repeat (0..10)
    SpreadCellProperties(pax_example::pax_types::pax_std::types::SpreadCellProperties),
    Rectangle(pax_example::pax_types::pax_std::primitives::Rectangle),
    Group(pax_example::pax_types::pax_std::primitives::Group),
    Spread(pax_example::pax_types::pax_std::components::Spread),
    Root(pax_example::Root),

}



//used namely for return types of expressions — may have other purposes
pub enum TypesCoproduct {
    //core (?)
    f64(f64),
    bool(bool),
    isize(isize),
    usize(usize), //used by Slot for index
    Vec_Rc_PropertiesCoproduct___(Vec<Rc<PropertiesCoproduct>>),
    Transform2D(pax_example::pax_types::Transform2D),
    //generated
    Stroke(pax_example::pax_types::pax_std::types::Stroke),
    Color(pax_example::pax_types::pax_std::types::Color),
    Size(pax_example::pax_types::pax_std::types::Size),
    SpreadDirection(pax_example::pax_types::pax_std::types::SpreadDirection),
    Vec_SpreadCellProperties_(Vec<pax_example::pax_types::pax_std::types::SpreadCellProperties>),
    Vec_LPAREN_usize_COMMA_Size_RPAREN(Vec<(usize, pax_example::pax_types::pax_std::types::Size)>),
}


//
// pub enum PatchCoproduct {
//
//     // Rectangle(pax_example::exports::pax_std::primitives::rectangle::Rectangle),
//     // Group(pax_example::exports::pax_std::primitives::group::Group),
//     RootPatch(pax_example::RootPatch),
// }
