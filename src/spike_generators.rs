extern crate dimensioned as dim;

use std::ops::AddAssign;

use dim::dimensions;
use dim::si;

pub trait SpikeGenerator {
    fn did_spike(&self) -> bool;
    fn try_advance(&mut self, dt: si::Second<f64>) -> bool;

    fn advance_and_check(&mut self, dt: si::Second<f64>) -> Option<bool> {
        if !self.try_advance(dt) {
            Option::None
        } else {
            Option::Some(self.did_spike())
        }
    }
}

pub trait SpikeGeneratorWithInput<Input: dimensions::Current>: SpikeGenerator {
    fn handle_input(&mut self, input: Input, dt: impl dimensions::Time);
}

pub struct SpikeAtTimes<T> {
    times: Vec<T>,
    time: T,
    idx: usize,
}

impl<T: dimensions::Time + From<si::Second<f64>>> SpikeAtTimes<T> {
    pub fn new(times: Vec<T>) -> SpikeAtTimes<T> {
        SpikeAtTimes {
            times: times,
            time: (0.0 * si::S).into(),
            idx: 0,
        }
    }
}

impl<T: dimensions::Time + From<si::Second<f64>> + AddAssign + PartialOrd> SpikeGenerator for SpikeAtTimes<T> {
    fn did_spike(&self) -> bool {
        return self.idx < self.times.len() && self.times[self.idx] == self.time;
    }

    fn try_advance(&mut self, dt: si::Second<f64>) -> bool {
        self.time += dt.into();
        while self.idx < self.times.len() && self.times[self.idx] < self.time {
            self.idx += 1;
        }
        true
    }
}
