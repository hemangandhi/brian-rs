pub mod spike_generators;
pub mod synapses;


// #[cfg(test)]
mod tests {

    extern crate brian_rs_macros;
    extern crate dimensioned as dim;

    use dim::si;
    use std::vec;

    use super::spike_generators::{
        continuous::WithSpikeDecay,
        discrete::{SpikeAtRate, SpikeAtTimes},
        InnerSpikeGenerator, InputSpikeGenerator, SpikeGenerator,
    };
    use super::synapses::Synaptic;

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
                assert_eq!(euler.get_voltage(), 0.5 * si::V);
            } else if i > 5 {
                assert_eq!(euler.get_voltage(), 1.0 * si::V);
            } else {
                assert_eq!(euler.get_voltage(), 0.0 * si::V);
            }
            euler.advance(0.2 * si::S);
        }
    }

    fn test_rate_fn(time: si::Second<f64>) -> Option<(i32, si::Second<f64>)> {
        assert!(time < (2.1 + 1e-7) * si::S, "Got invalid time {}", time);
        if time < 1. * si::S {
            Option::Some((0, 1.0 * si::S))
        } else if time < 2.0 * si::S {
            Option::Some((5, 2.0 * si::S))
        } else {
            Option::None
        }
    }

    #[test]
    fn rate_input_generator() {
        let mut rate_neuron = SpikeAtRate::new(
            Box::new(test_rate_fn),
            1. * si::S,
            0.5 * si::V,
            0,
            0.15 * si::S,
        );
        let mut spike_count = 0;
        for i in 0..30 {
            if rate_neuron.did_spike() {
                assert!(i >= 10 && i <= 20);
                spike_count += 1;
            }
            rate_neuron.advance(0.1 * si::S);
        }
        assert_eq!(spike_count, 5);
    }

    brian_rs_macros::define_neuron! {
    Izikhevich<si::Volt<f64>, si::Second<f64>>:
    params {
        a: f64, b: f64, c: f64, d: f64
    }
    initialize {
        v: si::Volt<f64> = c * si::V;
        u: si::Volt<f64> = 0.0 * si::V
    }
    time_step {
        v @ = (0.04 * self.v * self.v / si::V + 5.0 * self.v + 140.0 * si::V - self.u + input) / si::S;
        u @ = self.a * (self.b * self.v - self.u) / si::S
    }
    spike_when { self.v > 30.0 * si::V }
    get_voltage { self.v }
    reset { self.v = self.c * si::V; self.u += self.d * si::V }
    }

    brian_rs_macros::define_synapse! {
	StdpNeuron<si::Volt<f64>, si::Second<f64>>:
	params {
	    tau_pre: f64, tau_post: f64, w: si::Unitless<f64>, activation_bump: f64
	}
	initialize {
	    a_pre: si::Unitless<f64> = 0.0 * si::S / si::S;
	    a_post: si::Unitless<f64> = 0.0 * si::S / si::S
	}
	time_step {
	    a_pre @ = - (self.a_pre / self.tau_pre) / si::S;
	    a_post @ = - (self.a_post / self.tau_post) / si::S
	}
	weight_getter { *self.w }
	on_pre {
	    self.a_pre += self.activation_bump;
	    self.w += self.a_post
	}
	on_post {
	    self.a_post += self.activation_bump;
	    self.w += self.a_pre
	}
    }
}
