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

    use dim::dimensions;
    use dim::si;

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

    impl<T, I> super::SpikeGenerator<I> for SpikeAtTimes<T, I>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        I: From<si::Ampere<f64>> + Copy,
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
                (0.0 * si::A).into()
            }
        }
    }

    impl<T, I> super::InputSpikeGenerator<I, T> for SpikeAtTimes<T, I>
    where
        // TODO: alias this as a trait?
        T: From<si::Second<f64>>
            + Copy
            + PartialOrd<T>
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::ops::Neg<Output = T>,
        I: From<si::Ampere<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {
            self.time += dt.into();
            while self.idx < self.times.len() && self.times[self.idx] < self.time {
                self.idx += 1;
            }
        }
    }
}

pub mod continuous {
    extern crate dimensioned as dim;

    use dim::dimensions;
    use dim::si;

    #[derive(Debug)]
    pub struct ContinuousEquivalent<D, T, I> {
        last_spike: T,
        spike_time_scale: f64,
        spike_decay_scale: f64,
        discrete_neuron: D,
        spike_current: I,
    }

    impl<T: From<si::Second<f64>>, D, I: From<si::Ampere<f64>>> ContinuousEquivalent<D, T, I> {
        pub fn new(discrete_neuron: D, time_scale: f64, decay_scale: f64) -> Self {
            ContinuousEquivalent {
                last_spike: (0.0 * si::S).into(),
                spike_time_scale: time_scale,
                spike_decay_scale: decay_scale,
                discrete_neuron: discrete_neuron,
                spike_current: (0.0 * si::A).into(),
            }
        }
    }

    impl<D, T, I> super::SpikeGenerator<I> for ContinuousEquivalent<D, T, I>
    where
        D: super::SpikeGenerator<I>,
        T: Into<si::Second<f64>> + Copy,
        I: From<si::Ampere<f64>> + Into<si::Ampere<f64>> + Copy,
    {
        fn did_spike(&self) -> bool {
            self.discrete_neuron.did_spike()
        }
        fn get_current(&self) -> I {
            if self.did_spike() {
                self.discrete_neuron.get_current()
            } else {
                ((self.last_spike.into() / si::S * self.spike_time_scale).exp()
                    * self.spike_decay_scale
                    * self.spike_current.into())
                .into()
            }
        }
    }

    impl<D, T, I> super::InputSpikeGenerator<I, T> for ContinuousEquivalent<D, T, I>
    where
        D: super::InputSpikeGenerator<I, T>,
        T: From<si::Second<f64>> + Into<si::Second<f64>> + Copy,
        I: From<si::Ampere<f64>> + Into<si::Ampere<f64>> + Copy,
    {
        fn advance(&mut self, dt: T) {}
    }
}
