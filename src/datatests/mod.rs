//! Perform tests on real world data
//!
//! The data was accumulated listening to communication between
//! `rostopic pub` and `rostopic echo` for various standard messages.

mod string;
mod pose;
mod pose_with_covariance;
mod pose_array;
