/**
The Gaussian MKS unit system

Note: this system is incomplete. More derived units and constants are coming.





Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
SQRTM | SqrtMeter | sqrtm | 
SQRTKG | SqrtKilogram | sqrtkg | 
S | Second | s | Time
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
M | Meter | SqrtMeter * SqrtMeter | Length
KG | Kilogram | SqrtKilogram * SqrtKilogram | Mass
MPS | MeterPerSecond | Meter / Second | Velocity
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
*/
pub mod mks {
    make_units! {
        MKS;
        ONE: Unitless;

        base {
            SQRTM: SqrtMeter, "sqrtm";
            SQRTKG: SqrtKilogram, "sqrtkg";
            S: Second, "s", Time;
        }

        derived {
            M: Meter = (SqrtMeter * SqrtMeter), Length;
            KG: Kilogram = (SqrtKilogram * SqrtKilogram), Mass;
            MPS: MeterPerSecond = (Meter / Second), Velocity;
        }

        constants {
        }

        fmt = false;
    }

    #[cfg(feature = "serde")]
    impl_serde!(MKS);
    #[cfg(feature = "clapme")]
    impl_clapme!(MKS);

    pub use self::f64consts::*;


    #[test]
    fn test_mks_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

    }
    /// Test that serializing/deserializing values with units is
    /// equivalent to doing so with raw numeric types.
    #[cfg(feature = "serde")]
    #[test]
    fn test_mks_serde() {
        use ::serde_test::{assert_tokens, Token};

        let value = 1.0 * SQRTM;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * SQRTKG;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * S;
        assert_tokens(&value, &[Token::F64(1.0)]);

    }

    /// Test that clapme can generate a help message, and can produce a value.
    #[cfg(feature = "clapme")]
    #[test]
    fn test_mks_clapme() {

        let value = 3.0 * SQRTM;
        assert_eq!(value,
                   <SqrtMeter<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * SQRTKG;
        assert_eq!(value,
                   <SqrtKilogram<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * S;
        assert_eq!(value,
                   <Second<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

    }
}