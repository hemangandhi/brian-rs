//! This module represents neurons, generalized as "spike generators." To
//! support a wider range of abstractions, the input neurons are divided into
//! discrete and continuous implementations.

/// The general trait encapsulating a spike generator that has an output voltage
/// V.
pub trait SpikeGenerator<V> {
    /// Get whether the neuron/generator has spiked at the update.
    fn did_spike(&self) -> bool;
    /// Gets the voltage of the neuron/generator at the current time.
    fn get_voltage(&self) -> V;
}

/// An extension of a neuron that is in a hidden layer. Such a neuron will have
/// a voltage as well as a time-step as input.
pub trait InnerSpikeGenerator<V, T>: SpikeGenerator<V> {
    fn handle_input(&mut self, input: V, dt: T);
}

/// An extension of a neuron for input neurons. These neurons can be advanced
/// with no inputs except the time-step.
pub trait InputSpikeGenerator<V, T>: SpikeGenerator<V> {
    fn advance(&mut self, dt: T);
}

/// This module handles discrete neurons. Discrete neurons would be useful for
/// rate-encoding in SNNs and form a good basis for their continuous
/// counterparts.
pub mod discrete {
    extern crate dimensioned as dim;

    use dim::si;
    use std::cmp::Ordering;

    use super::{InputSpikeGenerator, SpikeGenerator};

    /// An input neuron that spikes at a given time.
    ///
    /// This can be used to represent simple visual inputs such as the neurons
    /// that detect whether a particular area is a given color.
    ///
    /// The timings of the spike would generally be based on the example being
    /// shown to the SNN, hence is a part of feature extraction.
    #[derive(Debug)]
    pub struct SpikeAtTimes<T, I> {
        times: Vec<T>,
        time: T,
        error_tolerance: T,
        idx: usize,
        spike_voltage: I,
    }

    impl<T: From<si::Second<f64>>, I> SpikeAtTimes<T, I> {
	/// Makes a new input neuron that shall spike at the given times,
	/// spiking at the given rate.
	///
	/// The tolerance is in case of floating-point imprecision or a
	/// time-step that doesn't exactly hit a spike time. This is an
	/// absolute error.
        pub fn new(times: Vec<T>, tolerance: T, spike_voltage: I) -> SpikeAtTimes<T, I> {
            SpikeAtTimes {
                times: times,
                time: (0.0 * si::S).into(),
                error_tolerance: tolerance,
                idx: 0,
                spike_voltage: spike_voltage,
            }
        }
    }

    impl<T, V> SpikeGenerator<V> for SpikeAtTimes<T, V>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        V: From<si::Volt<f64>> + Copy,
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

        fn get_voltage(&self) -> V {
            if self.did_spike() {
                self.spike_voltage.into()
            } else {
                (0.0 * si::V).into()
            }
        }
    }

    impl<T, V> InputSpikeGenerator<V, T> for SpikeAtTimes<T, V>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        V: From<si::Volt<f64>> + Copy,
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
    pub struct SpikeAtRate<T, V> {
        rate_at_time: Box<dyn Fn(T) -> Option<(i32, T)>>,
        time: T,
        slot_start_time: T,
        slot_end_time: T,
        spike_voltage: V,
        current_rate: i32,
        num_spiked: i32,
        tolerance: T,
    }

    impl<T, V> SpikeAtRate<T, V>
    where
        T: From<si::Second<f64>> + PartialOrd + Copy,
    {
        /// Makes a new neuron that will spike at the rate indicated by invoking
	/// the rate_fn at a time-step.
	///
	/// Args:
	/// * `rate_fn`: Returns the rate at which the neuron should spike at at a given
	///              time. It also returns a deadline for when all those spikes
	///              should occur. If the function returns None, it is assumed that
	///              the neuron is done spiking.
	/// * `slot_end_time`: When the first starting_rate spikes should occur by.
	/// * `spike_voltage`: The voltage to spike at when spiking.
	/// * `starting_rate`: The initial rate to spike at.
	/// * `tolerance`: "tolerance" is an implementation detail, but an important one: since
	///                slots are subdivided to ensure the correct number of spikes in the slot
	///                the tolerance is "how far from the starting of a sub-slot should the
	///                spike be within." Hence, for a tolerance t, you want to advance in a
	///                step t < dt < 2t to be sure that you hit every spike exactly once.
        pub fn new(
            rate_fn: Box<dyn Fn(T) -> Option<(i32, T)>>,
            slot_end_time: T,
            spike_voltage: V,
            starting_rate: i32,
            tolerance: T,
        ) -> Self {
            SpikeAtRate {
                rate_at_time: rate_fn,
                time: (0.0 * si::S).into(),
                slot_start_time: (0.0 * si::S).into(),
                slot_end_time: slot_end_time,
                spike_voltage: spike_voltage,
                current_rate: starting_rate,
                num_spiked: 0,
                tolerance: tolerance,
            }
        }

	/// Makes a function that, given the vector of slot start times and
	/// rates within that slot, returns a function that would serve as a
	/// `rate_fn` above.
	///
	/// As a side-effect, the input vector is lexicographically sorted based
	/// on the partial ordering on T. (So if T is a float, the incomparable
	/// values are all treated as equal, so use that at your own risk.)
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

    impl<T, V> SpikeGenerator<V> for SpikeAtRate<T, V>
    where
        T: Into<si::Second<f64>> + Copy + std::ops::Sub<Output = T>,
        V: From<si::Volt<f64>> + Copy,
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

        fn get_voltage(&self) -> V {
            if self.did_spike() {
                self.spike_voltage
            } else {
                (0.0 * si::V).into()
            }
        }
    }

    impl<T, V> InputSpikeGenerator<V, T> for SpikeAtRate<T, V>
    where
        T: Into<si::Second<f64>>
            + Copy
            + std::ops::Sub<Output = T>
            + std::ops::AddAssign
            + PartialOrd<T>,
        V: From<si::Volt<f64>> + Copy,
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

/// Ways of adding continuity to neuron implementations.
pub mod continuous {
    extern crate dimensioned as dim;

    use dim::si;

    /// Adds a time-based voltage decay to the discrete neuron type D.
    /// The neuron is at 0 voltage until it spikes. Then the voltage is left
    /// to the spike_decay_fn. Since the spking is detected by querying the
    /// wrapped discrete neuron, the precise timing of the spike may have an
    /// error as large as the time step used to `advance` this neuron.
    pub struct WithSpikeDecay<D, T, V> {
        time_since_spike: T,
        discrete_neuron: D,
        spike_voltage: V,
        spiked_yet: bool,
        spike_decay_fn: Box<dyn Fn(T, V) -> V>,
    }

    impl<T, D, V> WithSpikeDecay<D, T, V>
    where
        T: From<si::Second<f64>> + Into<si::Second<f64>> + Copy,
        V: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
	/// Args:
	/// * `discrete_neuron`: The discrete neuron to add a decay to.
	/// * `spike_decay_fn`: The function to decay along. The first argument is the time of
	///                     the previous spike and the second is the voltage at the spike.
        pub fn new(
	    discrete_neuron: D,
	    spike_decay_fn: Box<dyn Fn(T, V) -> V>) -> Self {
            WithSpikeDecay {
                time_since_spike: (0.0 * si::S).into(),
                discrete_neuron: discrete_neuron,
                spike_voltage: (0.0 * si::V).into(),
                spiked_yet: false,
                spike_decay_fn: spike_decay_fn,
            }
        }

	/// Wraps a discrete neuron into one that exponentially decays after
	/// spiking. The decay function outputted is V * a * e ^ (b * T) where V
	/// is the previous spike voltage, T is the time since the previous spike,
	/// * `spike_decay_scalar` is the scalar "a",
	/// * and `spike_timing_scalar` is the scalar "b" (in the exponent)
        pub fn exp_decay(
            discrete_neuron: D,
            spike_decay_scalar: f64,
            spike_timing_scalar: f64,
        ) -> Self {
            Self::new(
                discrete_neuron,
                Box::new(move |time: T, spike: V| {
                    ((-(time.into() / si::S) * spike_timing_scalar).exp()
                        * (spike.into() / si::V)
                        * spike_decay_scalar
                        * si::V)
                        .into()
                }),
            )
        }
    }

    impl<D, T, V> super::SpikeGenerator<V> for WithSpikeDecay<D, T, V>
    where
        D: super::SpikeGenerator<V>,
        T: Into<si::Second<f64>> + Copy,
        V: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
        fn did_spike(&self) -> bool {
            self.discrete_neuron.did_spike()
        }
        fn get_voltage(&self) -> V {
            if self.did_spike() {
                self.discrete_neuron.get_voltage()
            } else if !self.spiked_yet {
                (0.0 * si::V).into()
            } else {
		// Haha function pointer go brr.
                (*self.spike_decay_fn)(self.time_since_spike, self.spike_voltage)
            }
        }
    }

    impl<D, T, V> super::InputSpikeGenerator<V, T> for WithSpikeDecay<D, T, V>
    where
        D: super::InputSpikeGenerator<V, T>,
        T: From<si::Second<f64>> + Into<si::Second<f64>> + Copy + std::ops::AddAssign,
        V: From<si::Volt<f64>> + Into<si::Volt<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {
            self.discrete_neuron.advance(dt);
            if self.discrete_neuron.did_spike() {
                self.spiked_yet = true;
                self.time_since_spike = (0.0 * si::S).into();
                self.spike_voltage = self.discrete_neuron.get_voltage();
                return;
            }
            self.time_since_spike += dt;
        }
    }
}
