extern crate dimensioned as dim;

use dim::dimensions;

pub trait Synaptic{
    type PreSynaptic;
    type PostSynaptic;

    fn on_pre(&mut self);
    fn on_post(&mut self);
    fn current_weight(&self) -> f64;

    fn advance_once(&mut self, dt: impl dimensions::Time);
}
