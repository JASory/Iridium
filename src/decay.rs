//! Decay modes. Each struct represents a decay mode. 


pub(crate) mod internal;
pub(crate) mod types;
pub(crate) mod decayimpl;
pub(crate) mod dhelper;

pub use types::*;
pub use decayimpl::DecayMode;

pub(crate) use dhelper::is_mode_recorded;
