//! Camera trait with some common implementations.

pub use self::arc_ball::{ArcBall, CoordSystem};
pub use self::camera::Camera;
pub use self::first_person::FirstPerson;
pub use self::first_person_stereo::FirstPersonStereo;
pub use self::fixed_view::FixedView;

mod arc_ball;
#[doc(hidden)]
pub mod camera;
mod first_person;
mod first_person_stereo;
mod fixed_view;
