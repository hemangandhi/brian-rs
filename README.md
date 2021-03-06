Rewrite of [Brian](https://brian2.readthedocs.io/en/stable/), the SNN library from python into Rust.

# Why

Using Brian in Python has two main pain-points:

1. A lack of types makes it possible for a fully-trained model to abruptly crash, potentially wasting time.
1. A lack of formality in the string-based specification of neurons adds some uncertainty about the correctness
   of implementations.
   
This Rust crate hopes to fix these pain points by the power of Rust. In particular, this crate will:

1. support strongly-typed SNNs (where weights, neuronal activation functions, and other equations have proper units);
1. include a DSL to specify neurons (and the differential equations that arise) readably, concisely, and, most of all, testably;
1. and have feature-parity with Brain (where possible).

## Why Rust

| Programming language | Types? | Macros? | Conclusion |
|---|---|---|---|
| C++ | With C++20 concepts, this might be at parity with Rust | Nothing similar -- text-based macros are unsafe and template metaprogramming is not expressive enough | Does not have any way to add a DSL |
| Java | Probably good enough (with some pain since there's no operator overloading) | Nothing similar to macros | No DSL support |
| Lisps | Dynamically typed, so no additional guarantees | Very good DSL support | Type systems are too weak |
| Haskell | Yes | Through template Haskell, most likely | A little too niche. Execution may limit opportunities for future work in targetting niche hardware too. |

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
| `fn body` | See below for details. This is a `fn(&self) -> ()` (where `Self` is a neuron or synapse). |

## Derivatives and Equations

The derivatives are assumed to be time derivatives, so the equations therein are multiplied
by `si::Hz`. These would be in the format `<ident>'`.

The following are generally useful:
```
list of types := <ident>: <type>, <list of types> | <ident>: <type>
list of values := <ident>: <type> = <value>, <list of values> | <ident>: <type> = <value>
eqn := <ident or derivative> = <equation>
ident or derivative := <ident> | <ident>'
```

Derivatives are expected to be time derivatives and `x' = f(self)` evaluates to `let tmp_x = self.x + f(self) * dt; self.x = tmp_x;`
where temporaries are assigned so that updates are consistent (otherwise ordering would be difficult if not incorrect).
In general, `<list of eqns>` will have similar semantics where a temporary is made per left-hand-side identifier and then that
identifier is assigned.

### Temporary variables and the limits of `fn body`

Really, `fn body` could likely be any Rust function body with a few special rules for scoping. There would be three sets of `ident`ifiers
that should be handled in a special way:

1. State parameters are the set of identifiers used to encapsulate a state of the system at a particular time.
1. The values of the state parameters in the next step. These will be state parameters with a `~` prepended. This only exists after an equation with
   the identifier on the left hand side.
1. Derivative lvalues. This is `x'` where `x` is a state parameter.

The first type of variables cannot be aliased. That is, if you have `v` in your state, you cannot assign to something called `v` (`let v` would not
be allowed).

Note: there is no `~x'` for "the time derivative of `x` in the next time step" since time derivatives are not explicitly stored.

Otherwise, `let not_bound_in_state = f(non_state_stuff, state);` is a valid assignment.

## BNF for Neurons


```
neuron := define_neuron!(<type>: params {
             <list of types>
         }
         initialize {
            <list of values>
         }
         time_step(<ident>, <ident>) {
		    <fn body>
         }
         spike_when {
            <condition>
         }
		 get_current { <equation> }
         reset {
		    <fn body>
         })
```

The two identifiers passed into `time_step` are the current and time step respectively.

## What Neurons are

A neuron should be a `SpikeGeneratorWithInput`.

The `params` are taken to be initialization parameters.
The `initialize` values are initial values of other state variables essential to the neuron
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
    }
    initialize {
        v: f64 = c,
        u: f64 = 0
    }
	get_current { v }
    time_step(I, dt) {
        //TODO: add units
        v' = 0.04 v * v + 5 * v + 140 - u + I;
        u' = a * (b * v - u);
    }
    spike_when { v > 30 * si::mV }
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
    pub fn did_spike(&self) -> bool {
        self.v > 30 * si::mV
    }

    pub fn try_advance(&mut self, dt: impl dimensions::Time) -> bool {
        false
    }
	
	pub fn get_current(&self) -> si::Amp<f64> { self.v }
}

impl<si::Amp<f64>> SpikeGeneratorWithInput for Izekehvich {
    pub fn handle_input(&mut self, I: si::Amp<f64>, dt: impl dimensions::Time){
        if(self.v > 30 * si::mV){
            self.v = self.c;
            self.u += self.d;
        }

        let tmp_v = v + (0.04 * self.v * self.v + 5 * self.v + 140 - self.u + I) * si::Hz * dt;
        let tmp_u = u + self.a * (b * self.v - self.u) * si::Hz * dt;
		self.v = tmp_v; self.u = tmp_u;
    }
}
```

More generally, the `params` are parameters to a `new` function and `initialize` values are the remaining fields
in the struct and will be set to the value provided (which can be based on `params`).

`time_step` is a function that handles current and time deltas. Times are guaranteed to be from an Abelian group
and convertible from seconds.
(Which is `std::ops::Add + std::ops::Sub + std::ops::Neg + std::ops::AddAssign` and `From<SI::Second<f64>>`.)
Current (the first identifier in `time_step`) is also an Abelian group but convertible `From<si::Amp<f64>>`.

`spike_when` takes a reference to the struct being built so can reference all the identifiers in `params` and `initialize`.
It must evaluate to a boolean. `reset` runs if `spike_when` returns true, but within the `time_step` function
(so that spike detection does not have to mutate the neuron).

## BNF for Synapses

The meta-language is explained [above](#bnf-for-macros) as are [equations](#derivatives-and-equations).

```
synapse := define_synapse!(<type>: params {
            <list of types>
         }
         initialize {
            <list of values>
         }
         current_weight {
            <equation>
         }
         on_pre (<ident>) {
            <fn body>
         }
         on_post (<ident>) {
            <fn body>
         }
         time_step(<ident>, <ident>) {
            <fn body>
         })
```

`on_pre` and `on_post` merely mutate the state of the synapse while the `current_weight` is a "getter". `time_step` is the same mutation as
above.

# Trait Organization

There are three fundamental sorts of things involved:

- Spike generators (neurons and input neurons)
- Synapses
- SNNs (with inputs are "intermediates")

Each is explained below, represented in their own module.

## Spike generators

There are two spike generators: input neurons and "real" neurons.

It is worth noting that spiking and the actual output current are independent since STDP and Hebbian learning weight the output based on
a boolean predicate about the input or output neuron firing. It is theoretically impossible to determine if a neuron fired solely based on
current and (more obviously) vice versa.

### Input Neurons

Input neurons abstract the inputs into the brain and, hence, do not have an input current. They have a fixed set of times that they will
spike at. Currently, two encodings are implemented:

- Spiking at a fixed time (with a pre-determined current).
- Spiking at a fixed rate (with a pre-determined spike current).

For continuous neurons, currents will exponentially decay based on `a * spike_current * exp(- b * time_since_spiked)`
where `a` and `b` are user-provided. Before any spike, the current is 0.

Instead of this, a discrete neuron would always provide 0 current except when spiking, when it will provide the spike current.

### "Real" Neurons

These neurons are the ones that are actually "in the brain" and take input currents and produce outputs.

To ease abstraction, neurons will only take one input and the SNN will handle the weighted sum of current a neuron may recieve through multiple
synapses.

These neurons can only propagate state based on an input current and time step.

## Synapses

These don't have much variety: they just have a weight and may include a learning rate, but unlike neurons, there are no fundamentally different
synapse types.

## SNNs

SNNs will have 6 associated types:

- Input neuron type
- "Hidden" neuron type (regular neuron)
- Output neuron type (regular neuron)
- Trainer neuron type (also an input neuron, but can differ from the input neuron type above)
- Synapse type for all the internal connections
- Synapse type for the trainer neurons

Furthermore, there will be two SNN types:

- Input SNNs -- input SNNs are input spike generators.
- Middle SNNs -- spike generators that take input.

The SNNs will be buildable through builder so that layers can be initialized.



