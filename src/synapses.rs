extern crate dimensioned as dim;

use dim::dimensions;
use dim::si;
use std::vec;

pub mod synapses{
    pub trait Synaptic{
        type PreSynaptic;
        type PostSynaptic;

        pub fn on_pre(&mut self);
        pub fn on_post(&mut self);
        pub fn current_weight(&self) -> f64;

        pub fn advance_once(&mut self, dt: impl dimensions::Time);
    }
}
