//! Infrared protocols

#[cfg(feature = "denon")]
pub mod denon;
#[cfg(feature = "nec")]
pub mod nec;
#[cfg(feature = "rc5")]
pub mod rc5;
#[cfg(feature = "rc6")]
pub mod rc6;
#[cfg(feature = "sbp")]
pub mod sbp;

#[cfg(feature = "denon")]
pub use denon::Denon;
#[cfg(feature = "nec")]
#[doc(inline)]
pub use nec::{Nec, };
#[cfg(feature = "rc5")]
#[doc(inline)]
pub use rc5::Rc5;
#[cfg(feature = "rc6")]
#[doc(inline)]
pub use rc6::Rc6;
#[cfg(feature = "sbp")]
#[doc(inline)]
pub use sbp::Sbp;

pub mod capture;

pub(crate) mod utils;
