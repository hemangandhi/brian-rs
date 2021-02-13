extern crate dimensioned as dim;

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

#[derive(Debug)]
pub struct SpikeAtTimes<T> {
    times: Vec<T>,
    time: T,
    error_tolerance: T,
    idx: usize,
}

impl<T: From<si::Second<f64>>> SpikeAtTimes<T> {
    pub fn new(times: Vec<T>, tolerance: T) -> SpikeAtTimes<T> {
        SpikeAtTimes {
            times: times,
            time: (0.0 * si::S).into(),
            error_tolerance: tolerance,
            idx: 0,
        }
    }
}

impl<T> SpikeGenerator for SpikeAtTimes<T>
where
    // TODO: alias this as a trait?
    T: From<si::Second<f64>>
        + Copy
        + PartialOrd<T>
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>,
{
    fn did_spike(&self) -> bool {
        let idx = if self.idx >= self.times.len() {
            self.times.len() - 1
        } else {
            self.idx
        };
        let time_diff = self.times[idx] - self.time;
        return -self.error_tolerance < time_diff && time_diff < self.error_tolerance;
    }

    fn try_advance(&mut self, dt: si::Second<f64>) -> bool {
        self.time += dt.into();
        while self.idx < self.times.len() && self.times[self.idx] < self.time {
            self.idx += 1;
        }
        true
    }
}
