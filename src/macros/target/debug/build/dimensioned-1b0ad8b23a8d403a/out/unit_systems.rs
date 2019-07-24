/**
Predefined unit systems

When it makes sense, conversions are defined between unit systems. See the `conversion` module for
more information.

## Structure

Each unit system contained herein lists in tables its base units, derived units, and constants.

Each item in the "Constant" column is the name of a constant created in both the `f32consts` and
`f64consts` submodules. Everything in the `f64consts` submodule is also re-exported in the unit
system module for ease of use.

Each item in the "Unit" column is the name of a type alias for that unit in the unit system. It
needs to be parametrized to be used.

For example, in the SI system, there is a defined unit `Meter` with accompanying constant `M`. We
can use them as follows.

```rust
extern crate dimensioned as dim;
use dim::si::{self, Meter, M};

fn main() {
    let x: Meter<f64> = 3.0 * M;
    let y = Meter::new(3.0);
    let z = 3.0 * M;

    assert_eq!(x, y);
    assert_eq!(x, z);

    let x32: Meter<f32> = 3.0 * si::f32consts::M;
    let y32 = Meter::new(3.0);

    assert_eq!(x32, y32);
}
```

## Naming conventions

When a unit has a proper name, we use that. When it does not, we use the following naming
convention:

For the type definition of a derived unit, the name will all of the units in the numerator listed,
each followed by the number of its power, and then, if there are units in the denominator, the word
`Per` and all of the units in the denominator.

The accompanying constants follow a similar convention, but use the constant instead of unit name
and the letter `P` instead of `Per`.

For example, we define `MeterPerSecond2` for acceleration in the SI system, with the accompanying
constant `MPS2`.

## Completeness

Note that the unit systems included here should not be considered complete. New units and
systems will be added. If there are any particular units or unit systems that you think should be
added, please submit an issue on github.

---

All of these unit systems were generated using the `make_units!` macro. See its documentation for
more information.

*/

pub mod unit_systems {
    include!(concat!(env!("OUT_DIR"), "/si.rs"));
    include!(concat!(env!("OUT_DIR"), "/ucum.rs"));
    include!(concat!(env!("OUT_DIR"), "/mks.rs"));
    include!(concat!(env!("OUT_DIR"), "/cgs.rs"));
    include!(concat!(env!("OUT_DIR"), "/fps.rs"));
}