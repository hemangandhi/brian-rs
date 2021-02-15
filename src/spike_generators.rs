pub trait SpikeGenerator<I> {
    fn did_spike(&self) -> bool;
    fn get_current(&self) -> I;
}

pub trait InnerSpikeGenerator<I, T>: SpikeGenerator<I> {
    fn handle_input(&mut self, input: I, dt: T);
}

pub trait InputSpikeGenerator<I, T>: SpikeGenerator<I> {
    fn advance(&mut self, dt: T);
}

pub mod discrete {
    extern crate dimensioned as dim;

    use dim::si;
    use std::cmp::Ordering;

    use super::{InputSpikeGenerator, SpikeGenerator};

    #[derive(Debug)]
    pub struct SpikeAtTimes<T, I> {
        times: Vec<T>,
        time: T,
        error_tolerance: T,
        idx: usize,
        spike_current: I,
    }

    impl<T: From<si::Second<f64>>, I> SpikeAtTimes<T, I> {
        pub fn new(times: Vec<T>, tolerance: T, spike_current: I) -> SpikeAtTimes<T, I> {
            SpikeAtTimes {
                times: times,
                time: (0.0 * si::S).into(),
                error_tolerance: tolerance,
                idx: 0,
                spike_current: spike_current,
            }
        }
    }

    impl<T, I> SpikeGenerator<I> for SpikeAtTimes<T, I>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        I: From<si::Volt<f64>> + Copy,
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

        fn get_current(&self) -> I {
            if self.did_spike() {
                self.spike_current.into()
            } else {
                (0.0 * si::V).into()
            }
        }
    }

    impl<T, I> InputSpikeGenerator<I, T> for SpikeAtTimes<T, I>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        I: From<si::Volt<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {
            self.time += dt.into();
            while self.idx < self.times.len() && self.times[self.idx] < self.time {
                self.idx += 1;
            }
        }
    }

    /// A neuron that will spike a given number of times between certain time
    /// slots. (So it only means "rate" if the slot is one unit long.) This is
    /// implemented by taking slots from "rate_at_time" and spiking that many
    /// times in that slot.
    ///
    /// "tolerance" is an implementation detail, but an important one: since
    /// slots are subdivided to ensure the correct number of spikes in the slot
    /// the tolerance is "how far from the starting of a sub-slot should the
    /// spike be within." Hence, for a tolerance t, you want to advance in a
    /// step t < dt < 2t to be sure that you hit every spike exactly once.
    pub struct SpikeAtRate<T, I> {
        rate_at_time: Box<dyn Fn(T) -> Option<(i32, T)>>,
        time: T,
        slot_start_time: T,
        slot_end_time: T,
        spike_current: I,
        current_rate: i32,
        num_spiked: i32,
        tolerance: T,
    }

    impl<T, I> SpikeAtRate<T, I>
    where
        T: From<si::Second<f64>> + PartialOrd + Copy,
    {
        pub fn new(
            rate_fn: Box<dyn Fn(T) -> Option<(i32, T)>>,
            slot_end_time: T,
            spike_current: I,
            starting_rate: i32,
            tolerance: T,
        ) -> Self {
            SpikeAtRate {
                rate_at_time: rate_fn,
                time: (0.0 * si::S).into(),
                slot_start_time: (0.0 * si::S).into(),
                slot_end_time: slot_end_time,
                spike_current: spike_current,
                current_rate: starting_rate,
                num_spiked: 0,
                tolerance: tolerance,
            }
        }

        pub fn rate_fn_of_times<'a>(
            slot_starts_to_rate: &'a mut Vec<(T, i32)>,
        ) -> Box<dyn Fn(T) -> Option<(i32, T)> + 'a> {
            slot_starts_to_rate.sort_unstable_by(|a, b| {
                let (t1, r1) = a;
                let (t2, r2) = b;
                match t1.partial_cmp(t2) {
                    Option::None | Option::Some(Ordering::Equal) => r1.cmp(r2),
                    Option::Some(x) => x,
                }
            });
            Box::new(move |time: T| {
                let slot: Vec<&(T, i32)> = (*slot_starts_to_rate)
                    .iter()
                    .filter(|slt| time > slt.0)
                    .take(1)
                    .collect();
                if slot.len() == 0 {
                    return Option::None;
                }
                let (new_slot_end, new_rate) = slot[0];
                return Option::Some((*new_rate, *new_slot_end));
            })
        }
    }

    impl<T, I> SpikeGenerator<I> for SpikeAtRate<T, I>
    where
        T: Into<si::Second<f64>> + Copy + std::ops::Sub<Output = T>,
        I: From<si::Volt<f64>> + Copy,
    {
        fn did_spike(&self) -> bool {
            if self.current_rate <= 0 {
                return false;
            }
            let spike_interval_len: si::Second<f64> =
                ((self.slot_end_time - self.slot_start_time).into()) / (self.current_rate as f64);
            let adjusted_time = self.time.into()
                - spike_interval_len * (self.num_spiked as f64)
                - self.slot_start_time.into();
            0.0 * si::S < adjusted_time && adjusted_time <= self.tolerance.into()
        }

        fn get_current(&self) -> I {
            if self.did_spike() {
                self.spike_current
            } else {
                (0.0 * si::V).into()
            }
        }
    }

    impl<T, I> InputSpikeGenerator<I, T> for SpikeAtRate<T, I>
    where
        T: Into<si::Second<f64>>
            + Copy
            + std::ops::Sub<Output = T>
            + std::ops::AddAssign
            + PartialOrd<T>,
        I: From<si::Volt<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {
            // We move the "spiked" counter first since the more usual usage
	    // pattern would need to read whether the neuron spiked after
	    // advancing and doing this state change after the ones below
	    // would actually mean that checking "did_spike" in a loop would
	    // actually miss every spike since this check would incorrectly
	    // increment self.num_spiked.
            if self.did_spike() {
                self.num_spiked += 1;
            }
            self.time += dt;
            if self.time > self.slot_end_time && self.current_rate > -1 {
                self.slot_start_time = self.slot_end_time;
                if let Option::Some((new_rate, new_end)) = (*self.rate_at_time)(self.time) {
                    self.current_rate = new_rate;
                    self.slot_end_time = new_end;
                    self.num_spiked = 0;
                } else {
                    self.current_rate = -1;
                }
            }
        }
    }
}

pub mod continuous {
    extern crate dimensioned as dim;

    use dim::si;

    pub struct WithSpikeDecay<D, T, I> {
        time_since_spike: T,
        discrete_neuron: D,
        spike_current: I,
        spiked_yet: bool,
        spike_decay_fn: Box<dyn Fn(T, I) -> I>,
    }

    impl<T, D, I> WithSpikeDecay<D, T, I>
    where
        T: From<si::Second<f64>> + Into<si::Second<f64>> + Copy,
        I: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
        pub fn new(discrete_neuron: D, spike_decay_fn: Box<dyn Fn(T, I) -> I>) -> Self {
            WithSpikeDecay {
                time_since_spike: (0.0 * si::S).into(),
                discrete_neuron: discrete_neuron,
                spike_current: (0.0 * si::V).into(),
                spiked_yet: false,
                spike_decay_fn: spike_decay_fn,
            }
        }

        pub fn exp_decay(
            discrete_neuron: D,
            spike_decay_scalar: f64,
            spike_timing_scalar: f64,
        ) -> Self {
            Self::new(
                discrete_neuron,
                Box::new(move |time: T, spike: I| {
                    ((-(time.into() / si::S) * spike_timing_scalar).exp()
                        * (spike.into() / si::V)
                        * spike_decay_scalar
                        * si::V)
                        .into()
                }),
            )
        }
    }

    impl<D, T, I> super::SpikeGenerator<I> for WithSpikeDecay<D, T, I>
    where
        D: super::SpikeGenerator<I>,
        T: Into<si::Second<f64>> + Copy,
        I: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
        fn did_spike(&self) -> bool {
            self.discrete_neuron.did_spike()
        }
        fn get_current(&self) -> I {
            if self.did_spike() {
                self.discrete_neuron.get_current()
            } else if !self.spiked_yet {
                (0.0 * si::V).into()
            } else {
                (*self.spike_decay_fn)(self.time_since_spike, self.spike_current)
            }
        }
    }

    impl<D, T, I> super::InputSpikeGenerator<I, T> for WithSpikeDecay<D, T, I>
    where
        D: super::InputSpikeGenerator<I, T>,
        T: From<si::Second<f64>> + Into<si::Second<f64>> + Copy + std::ops::AddAssign,
        I: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {
            self.discrete_neuron.advance(dt);
            if self.discrete_neuron.did_spike() {
                self.spiked_yet = true;
                self.time_since_spike = (0.0 * si::S).into();
                self.spike_current = self.discrete_neuron.get_current();
                return;
            }
            self.time_since_spike += dt;
        }
    }
}
