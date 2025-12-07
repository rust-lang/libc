//! QuRT (Qualcomm Real-Time OS) bindings
//!
//! QuRT is Qualcomm's real-time operating system for Hexagon DSP architectures.
//! Headers available via the
//! Hexagon SDK: https://softwarecenter.qualcomm.com/catalog/item/Hexagon_SDK

pub(crate) mod errno;
pub(crate) mod fcntl;
pub(crate) mod limits;
pub(crate) mod pthread;
pub(crate) mod semaphore;
pub(crate) mod signal;
pub(crate) mod stdio;
pub(crate) mod stdlib;
pub(crate) mod sys;
pub(crate) mod time;
pub(crate) mod unistd;
