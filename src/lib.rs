pub mod spike_generators;
pub mod synapses;

#[cfg(test)]
mod tests {

    extern crate dimensioned as dim;
    use dim::si;
    use std::vec;

    use super::spike_generators::{discrete::SpikeAtTimes, SpikeGenerator, InputSpikeGenerator};

    #[test]
    fn spike_generator_at_times() {
        let times = vec![1. * si::S, 2. * si::S];
        let mut spiker = SpikeAtTimes::new(times.clone(), 0.1 * si::S, 0.3 * si::A);

        for i in 0..=30 {
            assert_eq!(spiker.did_spike(), i == 5 || i == 10);
            spiker.advance(0.2 * si::S);
        }
    }
}
