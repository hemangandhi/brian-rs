pub mod spike_generators;
pub mod synapses;

#[cfg(test)]
mod tests {

    extern crate dimensioned as dim;
    use dim::si;
    use std::vec;

    use super::spike_generators::{SpikeAtTimes, SpikeGenerator};

    #[test]
    fn spike_generator_at_times() {
        let times = vec![1. * si::S, 2. * si::S];
        let mut spiker = SpikeAtTimes::new(times.clone(), 0.1 * si::S);

        for i in 0..=10 {
            println!("{}", i);
            assert_eq!(spiker.did_spike(), i == 5 || i == 10);
            spiker.try_advance(0.2 * si::S);
        }
    }
}
