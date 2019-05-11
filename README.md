Rewrite of Brian, the SNN library from python into Rust.

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
by `si::Hz`.

## BNF for Neurons


```
neuron := params {
             <list of types>
         },
         initialize {
            <list of values>
         },
         time_step {
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

## What Neurons are


