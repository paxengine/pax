use std::cell::RefCell;
use std::rc::Rc;

//Component types
#[repr(u32)]
pub enum PropertiesCoproduct {
    /* entries generated via properties-coproduct-lib.tera */
    None,
    RepeatList(Vec<Rc<RefCell<PropertiesCoproduct>>>),
    RepeatItem(Rc<PropertiesCoproduct>, usize),
    isize(isize),
    stdCOCOopsCOCORangeLABRisizeRABR(std::ops::Range<isize>),

    
    crateCOCOExample(pax_example::pax_reexports::Example),
    
    crateCOCOcameraCOCOCamera(pax_example::pax_reexports::camera::Camera),
    
    crateCOCOcameraCOCOTypeExample(pax_example::pax_reexports::camera::TypeExample),
    
    crateCOCOfireworksCOCOFireworks(pax_example::pax_reexports::fireworks::Fireworks),
    
    crateCOCOgridsCOCOGrids(pax_example::pax_reexports::grids::Grids),
    
    crateCOCOgridsCOCORectDef(pax_example::pax_reexports::grids::RectDef),
    
    crateCOCOhello_rgbCOCOHelloRGB(pax_example::pax_reexports::hello_rgb::HelloRGB),
    
    pax_stdCOCOprimitivesCOCOEllipse(pax_example::pax_reexports::pax_std::primitives::Ellipse),
    
    pax_stdCOCOprimitivesCOCOFrame(pax_example::pax_reexports::pax_std::primitives::Frame),
    
    pax_stdCOCOprimitivesCOCOGroup(pax_example::pax_reexports::pax_std::primitives::Group),
    
    pax_stdCOCOprimitivesCOCORectangle(pax_example::pax_reexports::pax_std::primitives::Rectangle),
    
    pax_stdCOCOstackerCOCOStacker(pax_example::pax_reexports::pax_std::stacker::Stacker),
    
    pax_stdCOCOtypesCOCOColor(pax_example::pax_reexports::pax_std::types::Color),
    
    pax_stdCOCOtypesCOCOColorVariant(pax_example::pax_reexports::pax_std::types::ColorVariant),
    
    pax_stdCOCOtypesCOCOStackerCell(pax_example::pax_reexports::pax_std::types::StackerCell),
    
    pax_stdCOCOtypesCOCOStackerDirection(pax_example::pax_reexports::pax_std::types::StackerDirection),
    
    pax_stdCOCOtypesCOCOStroke(pax_example::pax_reexports::pax_std::types::Stroke),
    
}

//Property types
#[repr(u32)]
pub enum TypesCoproduct {
    
    Numeric(pax_runtime_api::Numeric),
    
    Size(pax_runtime_api::Size),
    
    Size2D(pax_runtime_api::Size2D),
    
    SizePixels(pax_runtime_api::SizePixels),
    
    String(String),
    
    Transform2D(pax_runtime_api::Transform2D),
    
    bool(bool),
    
    crateCOCOcameraCOCOTypeExample(pax_example::pax_reexports::camera::TypeExample),
    
    f64(f64),
    
    isize(isize),
    
    paxCOCOapiCOCONumeric(pax_example::pax_reexports::pax::api::Numeric),
    
    paxCOCOapiCOCOSize(pax_example::pax_reexports::pax::api::Size),
    
    paxCOCOapiCOCOSizePixels(pax_example::pax_reexports::pax::api::SizePixels),
    
    pax_stdCOCOtypesCOCOColor(pax_example::pax_reexports::pax_std::types::Color),
    
    pax_stdCOCOtypesCOCOColorVariant(pax_example::pax_reexports::pax_std::types::ColorVariant),
    
    pax_stdCOCOtypesCOCOStackerDirection(pax_example::pax_reexports::pax_std::types::StackerDirection),
    
    pax_stdCOCOtypesCOCOStroke(pax_example::pax_reexports::pax_std::types::Stroke),
    
    stdCOCOopsCOCORangeLABRisizeRABR(std::ops::Range<isize>),
    
    stdCOCOvecCOCOVecLABRcrateCOCOgridsCOCORectDefRABR(pax_example::pax_reexports::std::vec::Vec<pax_example::pax_reexports::grids::RectDef>),
    
    stdCOCOvecCOCOVecLABRpax_stdCOCOtypesCOCOStackerCellRABR(pax_example::pax_reexports::std::vec::Vec<pax_example::pax_reexports::pax_std::types::StackerCell>),
    
    stdCOCOvecCOCOVecLABRstdCOCOoptionCOCOOptionLABRpaxCOCOapiCOCOSizeRABRRABR(pax_example::pax_reexports::std::vec::Vec<pax_example::pax_reexports::std::option::Option<pax_example::pax_reexports::pax::api::Size>>),
    
    stdCOCOvecCOCOVecLABRstdCOCOrcCOCORcLABRPropertiesCoproductRABRRABR(std::vec::Vec<std::rc::Rc<PropertiesCoproduct>>),
    
    usize(pax_example::pax_reexports::usize),
    
}
