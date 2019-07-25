extern crate dimensioned as dim;
extern crate num_traits as num;

use dim::dimensions;
use dim::si;
use num::{NumAssignOps, NumOps};

pub trait IntoSecond: dimensions::Time + From<si::Second<f64>> {}
pub trait OrdNumber<T>:
    NumOps<T, T> + std::ops::Neg<Output = T> + NumAssignOps<T> + PartialOrd
{
}

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
    error_tolerance: T,
    idx: usize,
}

impl<T: IntoSecond> SpikeAtTimes<T> {
    pub fn new(times: Vec<T>, tolerance: T) -> SpikeAtTimes<T> {
        SpikeAtTimes {
            times: times,
            time: (0.0 * si::S).into(),
            error_tolerance: tolerance,
            idx: 0,
        }
    }
}

impl<T: IntoSecond + OrdNumber<T> + Clone> SpikeGenerator for SpikeAtTimes<T> {
    fn did_spike(&self) -> bool {
        // TODO: don't clone?
        let time_diff = self.times[self.idx].clone() - self.time.clone();
        return self.idx < self.times.len() && -self.error_tolerance.clone() < time_diff
            || time_diff < self.error_tolerance;
    }

    fn try_advance(&mut self, dt: si::Second<f64>) -> bool {
        self.time += dt.into();
        while self.idx < self.times.len() && self.times[self.idx] < self.time {
            self.idx += 1;
        }
        true
    }
}
