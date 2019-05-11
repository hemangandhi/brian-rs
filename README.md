Rewrite of [Brian](https://brian2.readthedocs.io/en/stable/), the SNN library from python into Rust.

# Why

1. Macros
1. Types

# TODO

1. [x] Write the unit library as an external thing. -- TYVM [dimensioned](https://docs.rs/dimensioned/0.7.0/dimensioned/index.html)
1. [ ] Design the neuron and synapse definition DSL
1. [ ] Figure out monitors in a nice interface. `trait Monitor`? How to handle neuronal internal state?
1. [ ] Implement the sick macros and builtins to make this rockin'

# BNF for the Macros

Here are the BNF descriptions of the macros. So `|` is a special symbol for options in re-write rules.
Rust's `|` is not used explicitly here so there's no confusion... yet.
Non-terminals on the right of the `:=` are in angle brackets `<>`.
This is in terms of Rust's [token tree](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html).
At least... sort of.

Some special non-terminals:

| Non-terminal | Meaning |
| --- | --- |
| `ident` | A Rust variable identifier. |
| `type` | A Rust type specification (with lifetimes, if needed). |
| `value` | A literal value. Types will be explained later on. |
| `equation` | A Rust expression. Usually should evaluate to the unit of the identifier. Special variables are noted where necessary. |
| `condition` | A Rust expression that evaluates to `bool`. |

## Derivatives

The derivatives are assumed to be time derivatives, so the equations therein are multiplied
by `si::Hz`. These would be in the format `<ident>'`.

## BNF for Neurons


```
neuron := <ident>, params {
             <list of types>
         },
         initialize {
            <list of values>
         },
         time_step(<ident>, <ident>) {
            <list of eqns>
         },
         spike_when {
            <condition>
         },
         reset {
            <list of eqns>
         }
list of types := <ident>: <type>, <list of types> | <ident>: <type>
list of values := <ident>: <value>, <list of values> | <ident>: <value>
list of eqns := <ident>: <eqn>, <list of eqns> | <ident>: <eqn>
eqn := <ident or derivative> = <equation>
ident or derivative := <ident> | <ident>'
```

The two identifiers passed into `time_step` are the current and time step respectively.

## What Neurons are

(TODO: formalize.)

A neuron should be a `SpikeGeneratorWithInput`.

The `params` are taken to be initialization parameters.
The `initialize` values are initial values of other state variables essential to the neuron
(TODO: understand how to get their type).
The `time_step` is essentially the contents of `handle_input` and `spike_when` is the contents of `did_spike`.
`reset` will happen if that condition is met.

Concretely:
```rust
define_neuron!(
    Izekhevich,
    params {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    },
    initialize {
        v: c,
        u: 0
    },
    time_step(I, dt) {
        //TODO: add units
        v' = 0.04 v * v + 5 * v + 140 - u + I;
        u' = a * (b * v - u);
    },
    spike_when { v > 30 * si::mV },
    reset {
        v = c;
        u += d;
    }
)
```
should become:
```rust
pub struct Izekhevich {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    v: f64,
    u: f64
}

impl Izekhevich{
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> self {
        Izekhevich {
            a: a,
            b: b,
            c: c,
            d: d,
            v: c,
            u: 0
        }
    }
}

impl SpikeGenerator for Izekehvich {
    pub fn did_spike(&self) {
        self.v > 30 * si::mV
    }

    pub fn try_advance(&mut self, dt: impl dimensions::Time) -> bool {
        false
    }
}

impl<si::Amp<f64>> SpikeGeneratorWithInput for Izekehvich {
    pub fn handle_input(&mut self, I: si::Amp<f64>, dt: impl dimensions::Time){
        if(self.v > 30 * si::mV){
            self.v = self.c;
            self.u += self.d;
        }

        self.v += (0.04 * self.v * self.v + 5 * self.v + 140 - self.u + I) * si::Hz * dt;
        u += self.a * (b * self.v - self.u) * si::Hz * dt;
    }
}
```
