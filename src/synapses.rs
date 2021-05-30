extern crate dimensioned as dim;

use dim::dimensions;

pub trait Synaptic<V, T>{

    fn on_pre(&mut self, input: V);
    fn on_post(&mut self, input: V);
    fn current_weight(&self) -> f64;

    fn advance_once(&mut self, dt: T);
}
