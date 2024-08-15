extern crate burn_core;

use burn_core::prelude::*;

#[derive(Clone, Default, PartialEq, PartialOrd, Debug)]
pub struct EdgedDevice;

#[derive(Clone, Default, Debug)]
pub struct EdgedBackend;
impl Backend for EdgedBackend {
    type Device = EdgedDevice;
    fn name() -> String {
        String::from("edged")
    }
    fn sync(_device: &Self::Device) {}
}
