/**
The foot, pound, second system, using the mass pound as a base unit.

Note: this system is incomplete. More derived units and constants are coming.





Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
SQRTFT | SqrtFoot | sqrtft | 
SQRTLB | SqrtPound | sqrtlb | 
S | Second | s | Time
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
FT | Foot | SqrtFoot * SqrtFoot | Length
LB | Pound | SqrtPound * SqrtPound | Mass
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
*/
pub mod fps {
    make_units! {
        FPS;
        ONE: Unitless;

        base {
            SQRTFT: SqrtFoot, "sqrtft";
            SQRTLB: SqrtPound, "sqrtlb";
            S: Second, "s", Time;
        }

        derived {
            FT: Foot = (SqrtFoot * SqrtFoot), Length;
            LB: Pound = (SqrtPound * SqrtPound), Mass;
        }

        constants {
        }

        fmt = false;
    }

    #[cfg(feature = "serde")]
    impl_serde!(FPS);
    #[cfg(feature = "clapme")]
    impl_clapme!(FPS);

    pub use self::f64consts::*;


    #[test]
    fn test_fps_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

    }
    /// Test that serializing/deserializing values with units is
    /// equivalent to doing so with raw numeric types.
    #[cfg(feature = "serde")]
    #[test]
    fn test_fps_serde() {
        use ::serde_test::{assert_tokens, Token};

        let value = 1.0 * SQRTFT;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * SQRTLB;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * S;
        assert_tokens(&value, &[Token::F64(1.0)]);

    }

    /// Test that clapme can generate a help message, and can produce a value.
    #[cfg(feature = "clapme")]
    #[test]
    fn test_fps_clapme() {

        let value = 3.0 * SQRTFT;
        assert_eq!(value,
                   <SqrtFoot<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * SQRTLB;
        assert_eq!(value,
                   <SqrtPound<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * S;
        assert_eq!(value,
                   <Second<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

    }
}