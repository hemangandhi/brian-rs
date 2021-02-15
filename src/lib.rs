pub mod spike_generators;
pub mod synapses;

#[cfg(test)]
mod tests {

    extern crate dimensioned as dim;
    use dim::si;
    use std::vec;

    use super::spike_generators::{
        continuous::WithSpikeDecay, discrete::SpikeAtTimes, InputSpikeGenerator, SpikeGenerator,
    };

    #[test]
    fn spike_generator_at_times() {
        let times = vec![1. * si::S, 2. * si::S];
        let mut spiker = SpikeAtTimes::new(times.clone(), 0.1 * si::S, 0.3 * si::V);

        for i in 0..=30 {
            assert_eq!(spiker.did_spike(), i == 5 || i == 10);
            spiker.advance(0.2 * si::S);
        }
    }

    #[test]
    fn exp_decay_on_generator() {
        let times = vec![1. * si::S, 2. * si::S];
        let discrete_spiker = SpikeAtTimes::new(times.clone(), 0.1 * si::S, 0.5 * si::V);
	// Note: this sucks but I'm not sure there's a way to clue rustc into the type
	// in the function, so this is perhaps the best I can do since I can't cleverly
	// express a higher-kinded type without more pain. Furthermore, "euler" is
	// configured in a strange way so that it supplies a 0 current until it spikes,
	// supplying a 0.5 voltage then and then a 1.0 voltage thereafter (until the
	// next spike).
        let mut euler: WithSpikeDecay<
            SpikeAtTimes<si::Second<f64>, si::Volt<f64>>,
            si::Second<f64>,
            si::Volt<f64>,
        > = WithSpikeDecay::exp_decay(discrete_spiker, 2.0, 0.0);
	for i in 0..30 {
	    assert_eq!(euler.did_spike(), i == 5 || i == 10);
	    if euler.did_spike() {
		assert_eq!(euler.get_current(), 0.5 * si::V);
	    } else if i > 5 {
		assert_eq!(euler.get_current(), 1.0 * si::V);
	    } else {
		assert_eq!(euler.get_current(), 0.0 * si::V);
	    }
	    euler.advance(0.2 * si::S);
	}
    }  
}
