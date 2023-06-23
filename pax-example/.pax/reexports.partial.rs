pub mod pax_reexports {
    pub use crate::Example;
    pub mod camera {
        pub use crate::camera::Camera;
        pub use crate::camera::TypeExample;
    }
    pub mod fireworks {
        pub use crate::fireworks::Fireworks;
    }
    pub mod grids {
        pub use crate::grids::Grids;
        pub use crate::grids::RectDef;
    }
    pub mod hello_rgb {
        pub use crate::hello_rgb::HelloRGB;
    }
    pub use f64;
    pub mod pax{
        pub mod api{
            pub use pax::api::Numeric;
            pub use pax::api::Size;
            pub use pax::api::SizePixels;
        }
    }
    pub mod pax_std{
        pub mod primitives{
            pub use pax_std::primitives::Ellipse;
            pub use pax_std::primitives::Frame;
            pub use pax_std::primitives::Group;
            pub use pax_std::primitives::Rectangle;
        }
        pub mod stacker{
            pub use pax_std::stacker::Stacker;
        }
        pub mod types{
            pub use pax_std::types::Color;
            pub use pax_std::types::ColorVariant;
            pub use pax_std::types::StackerCell;
            pub use pax_std::types::StackerDirection;
            pub use pax_std::types::Stroke;
        }
    }
    pub mod std{
        pub mod option{
            pub use std::option::Option;
        }
        pub mod vec{
            pub use std::vec::Vec;
        }
    }
    pub use usize;

}