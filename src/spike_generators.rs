extern crate dimensioned as dim;

use dim::dimensions;
use dim::si;
use std::vec;

pub mod spike_generator{
    pub trait SpikeGenerator{
        pub fn did_spike(&self) -> bool;
        pub fn try_advance(&mut self, dt: impl dimensions::Time) -> bool;

        pub fn advance_and_check(&mut self, dt: impl dimensions::Time) -> Option<bool> {
            if(!self.try_advance(dt)){
                Option::None
            }else{
                Option::Some(self.did_spike())
            }
        }
    }

    pub trait SpikeGeneratorWithInput<Input: dimensions::Current>: SpikeGenerator{
        pub fn handle_input(&mut self, input: Input, dt: impl dimensions::Time);
    }

    pub struct SpikeAtTimes<T>{
        times: Vec<T>,
        time: T,
        idx: u32
    }

    impl<T: dimensions::Time + From<si::Second<f64>>> SpikeAtTimes{
        pub fn new(Vec<T> times) -> SpikeAtTimes{
            SpikeAtTimes{
                times: times,
                time: (0 * si::S).into(),
                idx: 0
            }
        }
    }

    impl<T: dimensions::Time + From<si::Second<f64>>> SpikeGenerator for SpikeAtTimes<T>{
        pub fn did_spike(&self){
            return self.idx < self.times.len() && self.times[self.idx] == self.time;
        }

        pub fn try_advance(&mut self, dt: impl dimensions::Time) -> bool {
            self.time += dt.into();
            while (self.idx < self.times.len() && self.times[self.idx] < self.time) self.idx++;
            true
        }
    }
}
