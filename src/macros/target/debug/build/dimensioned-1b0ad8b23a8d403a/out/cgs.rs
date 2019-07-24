/**
The Gaussian CGS unit system

Note: this system is incomplete. More derived units and constants are coming.





Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
SQRTCM | SqrtCentimeter | sqrtcm | 
SQRTG | SqrtGram | sqrtg | 
S | Second | s | Time
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
CM | Centimeter | SqrtCentimeter * SqrtCentimeter | Length
G | Gram | SqrtGram * SqrtGram | Mass
CM2 | Centimeter2 | Centimeter * Centimeter | Area
CM3 | Centimeter3 | Centimeter2 * Centimeter | Volume
S2 | Second2 | Second * Second | 
S3 | Second3 | Second2 * Second | 
S4 | Second4 | Second3 * Second | 
CMPS | CentimeterPerSecond | Centimeter / Second | Velocity
CMPS3 | CentimeterPerSecond3 | Centimeter / Second3 | Jerk
CMPS4 | CentimeterPerSecond4 | Centimeter / Second4 | 
CM2PS | Centimeter2PerSecond | Centimeter2 / Second | 
CM2PS2 | Centimeter2PerSecond2 | Centimeter2 / Second2 | 
CM2PS3 | Centimeter2PerSecond3 | Centimeter2 / Second3 | 
CM3PS | Centimeter3PerSecond | Centimeter3 / Second | 
CM3PS2 | Centimeter3PerSecond2 | Centimeter3 / Second2 | 
CM3PS3 | Centimeter3PerSecond3 | Centimeter3 / Second3 | 
GAL | Gal | Centimeter / Second2 | Acceleration
DYN | Dyne | Gram * Gal | Force
ERG | Erg | Dyne * Centimeter | Energy
ERGPS | ErgPerSecond | Erg / Second | Power
BA | Barye | Dyne / Centimeter2 | Pressure
P | Poise | Gram / Centimeter / Second | 
ST | Stokes | Centimeter2 / Second | 
K | Kayser | Unitless / Centimeter | ReciprocalLength
STATC | StatCoulomb | SqrtGram * SqrtCentimeter * Centimeter / Second | 
STATA | StatAmpere | StatCoulomb / Second | 
STATV | StatVolt | Erg / StatCoulomb | 
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
Meter | M | HECTO * CM | Centimeter | Length
|
*/
pub mod cgs {
    make_units! {
        CGS;
        ONE: Unitless;

        base {
            SQRTCM: SqrtCentimeter, "sqrtcm";
            SQRTG: SqrtGram, "sqrtg";
            S: Second, "s", Time;
        }

        derived {
            CM: Centimeter = (SqrtCentimeter * SqrtCentimeter), Length;
            G: Gram = (SqrtGram * SqrtGram), Mass;
            CM2: Centimeter2 = (Centimeter * Centimeter), Area;
            CM3: Centimeter3 = (Centimeter2 * Centimeter), Volume;
            S2: Second2 = (Second * Second);
            S3: Second3 = (Second2 * Second);
            S4: Second4 = (Second3 * Second);
            CMPS: CentimeterPerSecond = (Centimeter / Second), Velocity;
            CMPS3: CentimeterPerSecond3 = (Centimeter / Second3), Jerk;
            CMPS4: CentimeterPerSecond4 = (Centimeter / Second4);
            CM2PS: Centimeter2PerSecond = (Centimeter2 / Second);
            CM2PS2: Centimeter2PerSecond2 = (Centimeter2 / Second2);
            CM2PS3: Centimeter2PerSecond3 = (Centimeter2 / Second3);
            CM3PS: Centimeter3PerSecond = (Centimeter3 / Second);
            CM3PS2: Centimeter3PerSecond2 = (Centimeter3 / Second2);
            CM3PS3: Centimeter3PerSecond3 = (Centimeter3 / Second3);
            GAL: Gal = (Centimeter / Second2), Acceleration;
            DYN: Dyne = (Gram * Gal), Force;
            ERG: Erg = (Dyne * Centimeter), Energy;
            ERGPS: ErgPerSecond = (Erg / Second), Power;
            BA: Barye = (Dyne / Centimeter2), Pressure;
            P: Poise = (Gram / Centimeter / Second);
            ST: Stokes = (Centimeter2 / Second);
            K: Kayser = (Unitless / Centimeter), ReciprocalLength;
            STATC: StatCoulomb = (SqrtGram * SqrtCentimeter * Centimeter / Second);
            STATA: StatAmpere = (StatCoulomb / Second);
            STATV: StatVolt = (Erg / StatCoulomb);
        }

        constants {
            M: Centimeter = HECTO * CM.value_unsafe;
        }

        fmt = false;
    }

    #[cfg(feature = "serde")]
    impl_serde!(CGS);
    #[cfg(feature = "clapme")]
    impl_clapme!(CGS);

    pub use self::f64consts::*;


    #[test]
    fn test_cgs_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

        assert_eq!(M, HECTO * CM);
    }
    /// Test that serializing/deserializing values with units is
    /// equivalent to doing so with raw numeric types.
    #[cfg(feature = "serde")]
    #[test]
    fn test_cgs_serde() {
        use ::serde_test::{assert_tokens, Token};

        let value = 1.0 * SQRTCM;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * SQRTG;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * S;
        assert_tokens(&value, &[Token::F64(1.0)]);

    }

    /// Test that clapme can generate a help message, and can produce a value.
    #[cfg(feature = "clapme")]
    #[test]
    fn test_cgs_clapme() {

        let value = 3.0 * SQRTCM;
        assert_eq!(value,
                   <SqrtCentimeter<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * SQRTG;
        assert_eq!(value,
                   <SqrtGram<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * S;
        assert_eq!(value,
                   <Second<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

    }
}