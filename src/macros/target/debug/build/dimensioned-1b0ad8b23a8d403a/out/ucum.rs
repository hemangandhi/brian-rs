/**


The Unified Code for Units of Measure (UCUM)

This is an attempt to define a unit system for [The Unified Code for Units of
Measure](http://unitsofmeasure.org/ucum.html). It does not perfectly match the specification in the
ways mentioned below.

---

The UCUM specification uses NIST values from 1980 for experimentally determined constants. For some
of these, we use the 2014 values instead, which can be found
[here](http://physics.nist.gov/cuu/Constants/).

---

There are a few classifications of units that UCUM defines but that it does not make sense to
define here, and so we do not. They are as follows:

* Units defined in terms of characters that we can't use. For example, the
symbol for minutes as measure of angle is given by a single quote, '.

* Units that require conversions that involve more than multiplication. These include some
temperature units (such as degrees Celcius) and logrithmic units (such as decibels).

---

Conflicts:

While the UCUM specification aims to minimize conflicts, there are still some. Units that may
conflict are indicated by the UCUM spec by square brackets. When there is a conflict, for the unit
in square brackets, we name it with a trailing underscore. For example, the speed of light
conflicts with coulomb, so we use `C_` for the speed of light.

---

A note on derived units:

Mass is involved in a great many units. As the SI unit of mass is the kilogram, there are many
units defined in terms of tffhe kilogram. To create a derived unit in dimensioned, it must have a
value of 1.0. Since UCUM uses the gram and not the kilogram as a base unit, that leads to many
common derived units being off by a factor of 1000. As a result, you will see many derived units
with prefixes, such as `MilliNewton` and `KiloFarad`. The constants for the more common units (`N`
and `F` for the two mentioned) are still defined, but if you need to refer to their types (such as
in function signatures), this is something to bear in mind.

---




Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.
# Base Units
Constant | Unit | Print Token | Dimensionn
---|---|---|---
M | Meter | m | Length
S | Second | s | Time
G | Gram | g | Mass
RAD | Radian | rad | 
K | Kelvin | K | Temperature
C | Coulomb | C | Charge
CD | Candela | cd | LuminousIntensity
# Derived Units
Constant | Unit | Unit Definition | Dimension
---|---|---|---
SR | Steradian | Radian * Radian | 
HZ | Hertz | Unitless / Second | Frequency
MILLIN | MilliNewton | Gram * Meter / Second / Second | Force
MILLIPA | MilliPascal | MilliNewton / Meter / Meter | Pressure
MILLIJ | MilliJoule | MilliNewton * Meter | Energy
MILLIW | MilliWatt | MilliJoule / Second | Power
A | Ampere | Coulomb / Second | Current
MILLIV | MilliVolt | MilliJoule / Coulomb | ElectricPotential
KILOF | KiloFarad | Coulomb / MilliVolt | Capacitance
MILLIOHM | MilliOhm | MilliVolt / Ampere | Resistance
KILOSIE | KiloSiemens | Unitless / MilliOhm | Conductance
MILLIWB | MilliWeber | MilliVolt * Second | MagneticFlux
MILLIT | MilliTesla | MilliWeber / Meter / Meter | 
MILLIH | MilliHenry | MilliWeber / Ampere | Inductance
LM | Lumen | Candela * Steradian | 
LX | Lux | Lumen / Meter / Meter | 
BQ | Becquerel | Unitless / Second | 
GY | Gray | MilliJoule / Gram | 
SV | Sievert | MilliJoule / Gram | 
S2 | Second2 | Second * Second | 
S3 | Second3 | Second2 * Second | 
M2 | Meter2 | Meter * Meter | Area
M3 | Meter3 | Meter2 * Meter | Volume
PM | PerMeter | Unitless / Meter | ReciprocalLength
MPS | MeterPerSecond | Meter / Second | Velocity
MPS2 | MeterPerSecond2 | Meter / Second2 | Acceleration
MPS3 | MeterPerSecond3 | Meter / Second3 | Jerk
M2PS | Meter2PerSecond | Meter2 / Second | 
M2PS2 | Meter2PerSecond2 | Meter2 / Second2 | 
APM | AmperePerMeter | Ampere / Meter | 
CPM2 | CandelaPerMeter2 | Candela / Meter2 | 
CPG | CoulombPerGram | Coulomb / Gram | 
GPM | GramPerMeter | Gram / Meter | 
GPM4S | GramPerMeter4Second | Gram / Meter3 / Meter / Second | 
MILLIJS | MilliJouleSecond | MilliJoule * Second | 
MILLIJPK | MilliJoulePerKelvin | MilliJoule / Kelvin | 
KILOFPM | KiloFaradPerMeter | KiloFarad / Meter | 
MILLINPA2 | MilliNewtonPerAmpere2 | MilliNewton / Ampere / Ampere | 
M3PGS2 | Meter3PerGramSecond2 | Meter3 / Gram / Second2 | 
MILLIPS | MilliPascalSecond | MilliPascal * Second | 
M3PSG | Meter3PerSecondGram | Meter3 / Second / Gram | 
# Constants
Name | Constant | Value | Unit | Dimension 
---|---|---|---|---
Centimeter | CM | CENTI * M | Meter | Length
Astronomic unit | ASU | 149597.870691e6 * M | Meter | Length
Parsec | PRS | 3.085678e16 * M | Meter | Length
Light-year | LY | C_ * ANN_J | Meter | Length
International inch | IN_I | 2.54 * CM | Meter | Length
International foot | FT_I | 12.0 * IN_I | Meter | Length
International yard | YD_I | 3.0 * FT_I | Meter | Length
International mile | MI_I | 5280.0 * FT_I | Meter | Length
International fathom | FTH_I | 6.0 * FT_I | Meter | Length
International nautical mile | NMI_I | 1852.0 * M | Meter | Length
International mil | MIL_I | 1.0e-3 * IN_I | Meter | Length
International hand | HD_I | 4.0 * IN_I | Meter | Length
US foot | FT_US | 1200.0 / 3937.0 * M | Meter | Length
US yard | YD_US | 3.0 * FT_US | Meter | Length
US inch | IN_US | FT_US / 12.0 | Meter | Length
US rod | RD_US | 16.5 * FT_US | Meter | Length
US Gunter's chain | CH_US | 4.0 * RD_US | Meter | Length
US Gunter's chain | LK_US | CH_US / 100.0 | Meter | Length
US Ramden's chain | RCH_US | 100.0 * FT_US | Meter | Length
US Link for Ramden's chain | RLK_US | RCH_US / 100.0 | Meter | Length
US fathom | FTH_US | 6.0 * FT_US | Meter | Length
US furlong | FUR_US | 40.0 * RD_US | Meter | Length
US mile | MI_US | 8.0 * FUR_US | Meter | Length
US mil | MIL_US | 1.0e-3 * IN_US | Meter | Length
British inch | IN_BR | 2.539998 * CM | Meter | Length
British foot | FT_BR | 12.0 * IN_BR | Meter | Length
British rod | RD_BR | 16.5 * FT_BR | Meter | Length
British Gunter's chain | CH_BR | 4.0 * RD_BR | Meter | Length
British link for Gunter's chain | LK_BR | CH_BR / 100.0 | Meter | Length
British fathom | FTH_BR | 6.0 * FT_BR | Meter | Length
British pace | PC_BR | 2.5 * FT_BR | Meter | Length
British yard | YD_BR | 3.0 * FT_BR | Meter | Length
British mile | MI_BR | 5280.0 * FT_BR | Meter | Length
British nautical mile | NMI_BR | 6080.0 * FT_BR | Meter | Length
Line | LNE | IN_I / 12.0 | Meter | Length
Point | PNT | LNE / 6.0 | Meter | Length
Pica | PCA | 12.0 * PNT | Meter | Length
Printer's point | PNT_PR | 0.013837 * IN_I | Meter | Length
Printer's pica | PCA_PR | 12.0 * PNT_PR | Meter | Length
Pied | PIED | 32.48 * CM | Meter | Length
Pounce | POUNCE | PIED / 12.0 | Meter | Length
Ligne | LIGNE | POUNCE / 12.0 | Meter | Length
Didot | DIDOT | LIGNE / 6.0 | Meter | Length
Cicero | CICERO | 12.0 * DIDOT | Meter | Length
Charrière | CH | 1.0 / 3.0 * MILLI * M | Meter | Length
Ångström | AO | 0.1 * NANO * M | Meter | Length
Smoot | SMOOT | 67.0 * IN_I | Meter | Length
|
Minute | MIN | 60.0 * S | Second | Time
Hour | HR | 60.0 * MIN | Second | Time
Day | D | 24.0 * HR | Second | Time
Tropical year | ANN_T | 365.24219 * D | Second | Time
Mean Julian year | ANN_J | 365.25 * D | Second | Time
Mean Gregorian year | ANN_G | 365.2425 * D | Second | Time
year | ANN | ANN_J | Second | Time
week | WK | 7.0 * D | Second | Time
Synodal month | MO_S | 29.53059 * D | Second | Time
Mean Julian month | MO_J | ANN_J / 12.0 | Second | Time
Mean Gregorian month | MO_G | ANN_G / 12.0 | Second | Time
Month | MO | MO_J | Second | Time
Svedberg unit | S_ | 1.0e-13 * S | Second | Time
|
Kilogram | KG | KILO * G | Gram | Mass
Tonne | TNE | 1.0e3 * KG | Gram | Mass
Unified atomic mass unit | AMU | 1.6605402e-24 * G | Gram | Mass
Electron mass | M_E | 9.10938356e-31 * KG | Gram | Mass
Proton mass | M_P | 1.6726231e-24 * G | Gram | Mass
Grain | GR | 64.79891 * MILLI * G | Gram | Mass
Avoirdupois pound | LB_AV | 7000.0 * GR | Gram | Mass
Avoirdupois ounce | OZ_AV | LB_AV / 16.0 | Gram | Mass
Avoirdupois dram | DR_AV | OZ_AV / 16.0 | Gram | Mass
Avoirdupois short hundredweight | SCWT_AV | 100.0 * LB_AV | Gram | Mass
Avoirdupois long hundredweight | LCWT_AV | 112.0 * LB_AV | Gram | Mass
Avoirdupois short ton | STON_AV | 20.0 * SCWT_AV | Gram | Mass
Avoirdupois long ton | LTON_AV | 20.0 * LCWT_AV | Gram | Mass
Avoirdupois stone | STONE_AV | 14.0 * LB_AV | Gram | Mass
Troy pennyweight | PWT_TR | 24.0 * GR | Gram | Mass
Troy ounce | OZ_TR | 20.0 * PWT_TR | Gram | Mass
Troy pound | LB_TR | 12.0 * OZ_TR | Gram | Mass
Apothecary scruple | SC_AP | 20.0 * GR | Gram | Mass
Apothecary dram | DR_AP | 3.0 * SC_AP | Gram | Mass
Apothecary ounce | OZ_AP | 8.0 * DR_AP | Gram | Mass
Apothecary pound | LB_AP | 12.0 * OZ_AP | Gram | Mass
Apothecary ounce | OZ_M | 28.0 * G | Gram | Mass
Metric carat | CAR_M | 0.2 * G | Gram | Mass
|
Gon, grade | GON | 0.9 * DEG | Radian | 
Degree | DEG | 2.0 * consts::PI / 360.0 * RAD | Radian | 
Circle | CIRC | 2.0 * consts::PI * RAD | Radian | 
|
Degree Rankine | DEGR | 5.0 / 9.0 * K | Kelvin | Temperature
|
Elementary charge | E | 1.6021766208e-19 * C | Coulomb | Charge
|
Sphere | SPH | 4.0 * consts::PI * SR | Steradian | 
|
Katal | KAT | MOL / S | Hertz | Frequency
Unit | U | MICRO * MOL / MIN | Hertz | Frequency
|
Newton | N | KILO * MILLIN | MilliNewton | Force
Gram force | GF | G * G_ | MilliNewton | Force
Pound force | LBF_AV | LB_AV * G_ | MilliNewton | Force
Dyne | DYN | G * CM / S2 | MilliNewton | Force
|
Pascal | PA | KILO * MILLIPA | MilliPascal | Pressure
Bar | BAR | 1.0e5 * PA | MilliPascal | Pressure
Standard atmosphere | ATM | 101325.0 * PA | MilliPascal | Pressure
Meter of water column | MH20 | 9.80665 * KILO * PA | MilliPascal | Pressure
Meter of mercury column | MHG | 133.3220 * KILO * PA | MilliPascal | Pressure
Technical atmosphere | ATT | KILO * GF / CM / CM | MilliPascal | Pressure
Pound per square inch | PSI | LBF_AV / IN_I / IN_I | MilliPascal | Pressure
|
Joule | J | KILO * MILLIJ | MilliJoule | Energy
Electronvolt | EV | E * V | MilliJoule | Energy
Erg | ERG | DYN * CM | MilliJoule | Energy
Calorie at 15 °C | CAL_15 | 4.18580 * J | MilliJoule | Energy
Calorie at 20 °C | CAL_20 | 4.18190 * J | MilliJoule | Energy
Mean calorie | CAL_M | 4.19002 * J | MilliJoule | Energy
International table calorie | CAL_IT | 4.1868 * J | MilliJoule | Energy
Thermochemical calorie | CAL_TH | 4.184 * J | MilliJoule | Energy
Calorie | CAL | CAL_TH | MilliJoule | Energy
Nutrition label calorie | CAL_ | KILO * CAL | MilliJoule | Energy
British thermal unit at 39 °F | BTU_39 | 1.05967 * KILO * J | MilliJoule | Energy
British thermal unit at 59 °F | BTU_59 | 1.05480 * KILO * J | MilliJoule | Energy
British thermal unit at 60 °F | BTU_60 | 1.05468 * KILO * J | MilliJoule | Energy
Mean British thermal unit | BTU_M | 1.05587 * KILO * J | MilliJoule | Energy
International table British thermal unit | BTU_IT | 1.05505585262 * KILO * J | MilliJoule | Energy
Thermochemical British thermal unit | BTU_TH | 1.054350 * KILO * J | MilliJoule | Energy
British thermal unit | BTU | BTU_TH | MilliJoule | Energy
|
Watt | W | KILO * MILLIW | MilliWatt | Power
Horsepower | HP | 550.0 * FT_I * LBF_AV / S | MilliWatt | Power
|
Biot | BI | 10.0 * A | Ampere | Current
Gilbert | GB | OE * CM | Ampere | Current
|
Volt | V | KILO * MILLIV | MilliVolt | ElectricPotential
|
Farad | F | MILLI * KILOF | KiloFarad | Capacitance
|
Ohm | OHM | KILO * MILLIOHM | MilliOhm | Resistance
|
Siemens | SIE | MILLI * KILOSIE | KiloSiemens | Conductance
Mho | MHO | MILLI * SIE | KiloSiemens | Conductance
|
Weber | WB | KILO * MILLIWB | MilliWeber | MagneticFlux
Maxwell | MX | 1.0e-8 * WB | MilliWeber | MagneticFlux
|
Tesla | T | KILO * MILLIT | MilliTesla | 
Gauss | GS | 1.0e-4 * T | MilliTesla | 
|
Henry | H | KILO * MILLIH | MilliHenry | Inductance
|
Phot | PHT | 1.0e-4 * LX | Lux | 
|
Curie | CI | 3.7e10 * BQ | Becquerel | 
|
Are | AR | 100.0 * M2 | Meter2 | Area
International square inch | SIN_I | IN_I * IN_I | Meter2 | Area
International square foot | SFT_I | FT_I * FT_I | Meter2 | Area
International square yard | SYD_I | YD_I * YD_I | Meter2 | Area
International circular mil | CML_I | consts::PI / 4.0 * MIL_I * MIL_I | Meter2 | Area
US acre | ACR_US | 160.0 * RD_US * RD_US | Meter2 | Area
US square rod | SRD_US | RD_US * RD_US | Meter2 | Area
US square mile | SMI_US | MI_US * MI_US | Meter2 | Area
Section | SCT | MI_US * MI_US | Meter2 | Area
Township | TWP | 36.0 * SCT | Meter2 | Area
British acre | ACR_BR | 4840.0 * YD_BR * YD_BR | Meter2 | Area
Barn | BRN | 100.0 * FEMTO * M * FEMTO * M | Meter2 | Area
|
Liter | L | 0.1 * 0.1 * 0.1 * M3 | Meter3 | Volume
International cubic inch | CIN_I | IN_I * IN_I * IN_I | Meter3 | Volume
International cubic foot | CFT_I | FT_I * FT_I * FT_I | Meter3 | Volume
International cubic yard | CYD_I | YD_I * YD_I * YD_I | Meter3 | Volume
International board foot | BF_I | 144.0 * CIN_I | Meter3 | Volume
International cord | CR_I | 128.0 * CFT_I | Meter3 | Volume
Queen Anne's wine gallon | GAL_US | 231.0 * IN_I * IN_I * IN_I | Meter3 | Volume
US barrel | BBL_US | 42.0 * GAL_US | Meter3 | Volume
US quart | QT_US | GAL_US / 4.0 | Meter3 | Volume
US pint | PT_US | QT_US / 2.0 | Meter3 | Volume
US gill | GIL_US | PT_US / 4.0 | Meter3 | Volume
US fluid Ounce | FOZ_US | GIL_US / 4.0 | Meter3 | Volume
US fluid Dram | FDR_US | FOZ_US / 8.0 | Meter3 | Volume
US minim | MIN_US | FDR_US / 60.0 | Meter3 | Volume
US cord | CRD_US | CR_I | Meter3 | Volume
US bushel | BU_US | 2150.42 * IN_I * IN_I * IN_I | Meter3 | Volume
Historical winchester gallon | GAL_WI | BU_US / 8.0 | Meter3 | Volume
US peck | PK_US | BU_US / 4.0 | Meter3 | Volume
US dry quart | DQT_US | PK_US / 8.0 | Meter3 | Volume
US dry pint | DPT_US | DQT_US / 2.0 | Meter3 | Volume
US tablespoon | TBS_US | FOZ_US / 2.0 | Meter3 | Volume
US teaspoon | TSP_US | TBS_US / 3.0 | Meter3 | Volume
US cup | CUP_US | 16.0 * TBS_US | Meter3 | Volume
Metric fluid ounce | FOZ_M | 30.0 * MILLI * L | Meter3 | Volume
Metric cup | CUP_M | 240.0 * MILLI * L | Meter3 | Volume
Metric teaspoon | TSP_M | 5.0 * MILLI * L | Meter3 | Volume
Metric tablespoon | TBS_M | 15.0 * MILLI * L | Meter3 | Volume
British gallon | GAL_BR | 4.54609 * L | Meter3 | Volume
British peck | PK_BR | 2.0 * GAL_BR | Meter3 | Volume
British bushel | BU_BR | 4.0 * PK_BR | Meter3 | Volume
British quart | QT_BR | GAL_BR / 4.0 | Meter3 | Volume
British pint | PT_BR | QT_BR / 2.0 | Meter3 | Volume
British gill | GIL_BR | PT_BR / 4.0 | Meter3 | Volume
British fluid ounce | FOZ_BR | GIL_BR / 5.0 | Meter3 | Volume
British fluid dram | FDR_BR | FOZ_BR / 8.0 | Meter3 | Volume
British minim | MIN_BR | FDR_BR / 60.0 | Meter3 | Volume
Drop | DRP | MILLI * L / 20.0 | Meter3 | Volume
Stere | STR | 1.0 * M3 | Meter3 | Volume
|
Kayser | KY | 1.0 / CM | PerMeter | ReciprocalLength
Diopter | DIOP | 1.0 / M | PerMeter | ReciprocalLength
Mesh | MESH_I | 1.0 / IN_I | PerMeter | ReciprocalLength
|
Speed of light in a vacuum | C_ | 299792458.0 * MPS | MeterPerSecond | Velocity
International knot | KN_I | NMI_I / HR | MeterPerSecond | Velocity
British knot | KN_BR | NMI_BR / HR | MeterPerSecond | Velocity
|
Standard acceleration of free fall | G_ | 9.80665 * M / S2 | MeterPerSecond2 | Acceleration
Gal | GL | CM / S2 | MeterPerSecond2 | Acceleration
|
Stokes | ST | CM * CM / S | Meter2PerSecond | 
|
Radiation absorbed dose | RAD_ | 100.0 * ERG / G | Meter2PerSecond2 | 
Radiation equivalent man | REM_ | RAD_ | Meter2PerSecond2 | 
|
Oersted | OE | 250.0 / consts::PI * A / M | AmperePerMeter | 
|
Stilb | SB | CD / CM / CM | CandelaPerMeter2 | 
Lambert | LMB | SB / consts::PI | CandelaPerMeter2 | 
|
Roentgen | ROE | 2.58e-4 * C / KG | CoulombPerGram | 
|
Tex | TEX | 1.0 * G / (KILO * M) | GramPerMeter | 
Denier | DEN | TEX / 9.0 | GramPerMeter | 
|
Peripheral vascular resistance unit | PRU | MHG * S / L | GramPerMeter4Second | 
|
Planck constant | H_ | 6.6260755e-34 * J * S | MilliJouleSecond | 
|
Boltzmann constant | K_ | 1.380658e-23 * J / K | MilliJoulePerKelvin | 
|
Permittivity of vacuum | EPS_0 | 8.854187817e-12 * F / M | KiloFaradPerMeter | 
|
Permeability of vacuum | MU_0 | 4.0e-7 * consts::PI * N / A / A | MilliNewtonPerAmpere2 | 
|
Newtonian constant of gravitation | GC | 6.67259e-11 * M3 / KG / S2 | Meter3PerGramSecond2 | 
|
Poise | P | DYN * S / CM / CM | MilliPascalSecond | 
|
Metabolic equivalent | MET | 3.5 * MILLI * L / MIN / KG | Meter3PerSecondGram | 
|
*/
pub mod ucum {
    make_units! {
        UCUM;
        ONE: Unitless;

        base {
            M: Meter, "m", Length;
            S: Second, "s", Time;
            G: Gram, "g", Mass;
            RAD: Radian, "rad";
            K: Kelvin, "K", Temperature;
            C: Coulomb, "C", Charge;
            CD: Candela, "cd", LuminousIntensity;
        }

        derived {
            SR: Steradian = (Radian * Radian);
            HZ: Hertz = (Unitless / Second), Frequency;
            MILLIN: MilliNewton = (Gram * Meter / Second / Second), Force;
            MILLIPA: MilliPascal = (MilliNewton / Meter / Meter), Pressure;
            MILLIJ: MilliJoule = (MilliNewton * Meter), Energy;
            MILLIW: MilliWatt = (MilliJoule / Second), Power;
            A: Ampere = (Coulomb / Second), Current;
            MILLIV: MilliVolt = (MilliJoule / Coulomb), ElectricPotential;
            KILOF: KiloFarad = (Coulomb / MilliVolt), Capacitance;
            MILLIOHM: MilliOhm = (MilliVolt / Ampere), Resistance;
            KILOSIE: KiloSiemens = (Unitless / MilliOhm), Conductance;
            MILLIWB: MilliWeber = (MilliVolt * Second), MagneticFlux;
            MILLIT: MilliTesla = (MilliWeber / Meter / Meter);
            MILLIH: MilliHenry = (MilliWeber / Ampere), Inductance;
            LM: Lumen = (Candela * Steradian);
            LX: Lux = (Lumen / Meter / Meter);
            BQ: Becquerel = (Unitless / Second);
            GY: Gray = (MilliJoule / Gram);
            SV: Sievert = (MilliJoule / Gram);
            S2: Second2 = (Second * Second);
            S3: Second3 = (Second2 * Second);
            M2: Meter2 = (Meter * Meter), Area;
            M3: Meter3 = (Meter2 * Meter), Volume;
            PM: PerMeter = (Unitless / Meter), ReciprocalLength;
            MPS: MeterPerSecond = (Meter / Second), Velocity;
            MPS2: MeterPerSecond2 = (Meter / Second2), Acceleration;
            MPS3: MeterPerSecond3 = (Meter / Second3), Jerk;
            M2PS: Meter2PerSecond = (Meter2 / Second);
            M2PS2: Meter2PerSecond2 = (Meter2 / Second2);
            APM: AmperePerMeter = (Ampere / Meter);
            CPM2: CandelaPerMeter2 = (Candela / Meter2);
            CPG: CoulombPerGram = (Coulomb / Gram);
            GPM: GramPerMeter = (Gram / Meter);
            GPM4S: GramPerMeter4Second = (Gram / Meter3 / Meter / Second);
            MILLIJS: MilliJouleSecond = (MilliJoule * Second);
            MILLIJPK: MilliJoulePerKelvin = (MilliJoule / Kelvin);
            KILOFPM: KiloFaradPerMeter = (KiloFarad / Meter);
            MILLINPA2: MilliNewtonPerAmpere2 = (MilliNewton / Ampere / Ampere);
            M3PGS2: Meter3PerGramSecond2 = (Meter3 / Gram / Second2);
            MILLIPS: MilliPascalSecond = (MilliPascal * Second);
            M3PSG: Meter3PerSecondGram = (Meter3 / Second / Gram);
        }

        constants {
            KG: Gram = KILO * G.value_unsafe;
            CM: Meter = CENTI * M.value_unsafe;
            MOL: Unitless = 6.0221367e23 * ONE.value_unsafe;
            N: MilliNewton = KILO * MILLIN.value_unsafe;
            PA: MilliPascal = KILO * MILLIPA.value_unsafe;
            J: MilliJoule = KILO * MILLIJ.value_unsafe;
            W: MilliWatt = KILO * MILLIW.value_unsafe;
            V: MilliVolt = KILO * MILLIV.value_unsafe;
            F: KiloFarad = MILLI * KILOF.value_unsafe;
            OHM: MilliOhm = KILO * MILLIOHM.value_unsafe;
            SIE: KiloSiemens = MILLI * KILOSIE.value_unsafe;
            WB: MilliWeber = KILO * MILLIWB.value_unsafe;
            T: MilliTesla = KILO * MILLIT.value_unsafe;
            H: MilliHenry = KILO * MILLIH.value_unsafe;
            GON: Radian = 0.9 * DEG.value_unsafe;
            DEG: Radian = 2.0 * consts::PI / 360.0 * RAD.value_unsafe;
            L: Meter3 = 0.1 * 0.1 * 0.1 * M3.value_unsafe;
            AR: Meter2 = 100.0 * M2.value_unsafe;
            MIN: Second = 60.0 * S.value_unsafe;
            HR: Second = 60.0 * MIN.value_unsafe;
            D: Second = 24.0 * HR.value_unsafe;
            ANN_T: Second = 365.24219 * D.value_unsafe;
            ANN_J: Second = 365.25 * D.value_unsafe;
            ANN_G: Second = 365.2425 * D.value_unsafe;
            ANN: Second = ANN_J.value_unsafe;
            WK: Second = 7.0 * D.value_unsafe;
            MO_S: Second = 29.53059 * D.value_unsafe;
            MO_J: Second = ANN_J.value_unsafe / 12.0;
            MO_G: Second = ANN_G.value_unsafe / 12.0;
            MO: Second = MO_J.value_unsafe;
            TNE: Gram = 1.0e3 * KG.value_unsafe;
            BAR: MilliPascal = 1.0e5 * PA.value_unsafe;
            AMU: Gram = 1.6605402e-24 * G.value_unsafe;
            EV: MilliJoule = E.value_unsafe * V.value_unsafe;
            ASU: Meter = 149597.870691e6 * M.value_unsafe;
            PRS: Meter = 3.085678e16 * M.value_unsafe;
            C_: MeterPerSecond = 299792458.0 * MPS.value_unsafe;
            H_: MilliJouleSecond = 6.6260755e-34 * J.value_unsafe * S.value_unsafe;
            K_: MilliJoulePerKelvin = 1.380658e-23 * J.value_unsafe / K.value_unsafe;
            EPS_0: KiloFaradPerMeter = 8.854187817e-12 * F.value_unsafe / M.value_unsafe;
            MU_0: MilliNewtonPerAmpere2 = 4.0e-7 * consts::PI * N.value_unsafe / A.value_unsafe / A.value_unsafe;
            E: Coulomb = 1.6021766208e-19 * C.value_unsafe;
            M_E: Gram = 9.10938356e-31 * KG.value_unsafe;
            M_P: Gram = 1.6726231e-24 * G.value_unsafe;
            GC: Meter3PerGramSecond2 = 6.67259e-11 * M3.value_unsafe / KG.value_unsafe / S2.value_unsafe;
            G_: MeterPerSecond2 = 9.80665 * M.value_unsafe / S2.value_unsafe;
            ATM: MilliPascal = 101325.0 * PA.value_unsafe;
            LY: Meter = C_.value_unsafe * ANN_J.value_unsafe;
            GF: MilliNewton = G.value_unsafe * G_.value_unsafe;
            LBF_AV: MilliNewton = LB_AV.value_unsafe * G_.value_unsafe;
            KY: PerMeter = 1.0 / CM.value_unsafe;
            GL: MeterPerSecond2 = CM.value_unsafe / S2.value_unsafe;
            DYN: MilliNewton = G.value_unsafe * CM.value_unsafe / S2.value_unsafe;
            ERG: MilliJoule = DYN.value_unsafe * CM.value_unsafe;
            P: MilliPascalSecond = DYN.value_unsafe * S.value_unsafe / CM.value_unsafe / CM.value_unsafe;
            BI: Ampere = 10.0 * A.value_unsafe;
            ST: Meter2PerSecond = CM.value_unsafe * CM.value_unsafe / S.value_unsafe;
            MX: MilliWeber = 1.0e-8 * WB.value_unsafe;
            GS: MilliTesla = 1.0e-4 * T.value_unsafe;
            OE: AmperePerMeter = 250.0 / consts::PI * A.value_unsafe / M.value_unsafe;
            GB: Ampere = OE.value_unsafe * CM.value_unsafe;
            SB: CandelaPerMeter2 = CD.value_unsafe / CM.value_unsafe / CM.value_unsafe;
            LMB: CandelaPerMeter2 = SB.value_unsafe / consts::PI;
            PHT: Lux = 1.0e-4 * LX.value_unsafe;
            CI: Becquerel = 3.7e10 * BQ.value_unsafe;
            ROE: CoulombPerGram = 2.58e-4 * C.value_unsafe / KG.value_unsafe;
            RAD_: Meter2PerSecond2 = 100.0 * ERG.value_unsafe / G.value_unsafe;
            REM_: Meter2PerSecond2 = RAD_.value_unsafe;
            IN_I: Meter = 2.54 * CM.value_unsafe;
            FT_I: Meter = 12.0 * IN_I.value_unsafe;
            YD_I: Meter = 3.0 * FT_I.value_unsafe;
            MI_I: Meter = 5280.0 * FT_I.value_unsafe;
            FTH_I: Meter = 6.0 * FT_I.value_unsafe;
            NMI_I: Meter = 1852.0 * M.value_unsafe;
            KN_I: MeterPerSecond = NMI_I.value_unsafe / HR.value_unsafe;
            SIN_I: Meter2 = IN_I.value_unsafe * IN_I.value_unsafe;
            SFT_I: Meter2 = FT_I.value_unsafe * FT_I.value_unsafe;
            SYD_I: Meter2 = YD_I.value_unsafe * YD_I.value_unsafe;
            CIN_I: Meter3 = IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
            CFT_I: Meter3 = FT_I.value_unsafe * FT_I.value_unsafe * FT_I.value_unsafe;
            CYD_I: Meter3 = YD_I.value_unsafe * YD_I.value_unsafe * YD_I.value_unsafe;
            BF_I: Meter3 = 144.0 * CIN_I.value_unsafe;
            CR_I: Meter3 = 128.0 * CFT_I.value_unsafe;
            MIL_I: Meter = 1.0e-3 * IN_I.value_unsafe;
            CML_I: Meter2 = consts::PI / 4.0 * MIL_I.value_unsafe * MIL_I.value_unsafe;
            HD_I: Meter = 4.0 * IN_I.value_unsafe;
            FT_US: Meter = 1200.0 / 3937.0 * M.value_unsafe;
            YD_US: Meter = 3.0 * FT_US.value_unsafe;
            IN_US: Meter = FT_US.value_unsafe / 12.0;
            RD_US: Meter = 16.5 * FT_US.value_unsafe;
            CH_US: Meter = 4.0 * RD_US.value_unsafe;
            LK_US: Meter = CH_US.value_unsafe / 100.0;
            RCH_US: Meter = 100.0 * FT_US.value_unsafe;
            RLK_US: Meter = RCH_US.value_unsafe / 100.0;
            FTH_US: Meter = 6.0 * FT_US.value_unsafe;
            FUR_US: Meter = 40.0 * RD_US.value_unsafe;
            MI_US: Meter = 8.0 * FUR_US.value_unsafe;
            ACR_US: Meter2 = 160.0 * RD_US.value_unsafe * RD_US.value_unsafe;
            SRD_US: Meter2 = RD_US.value_unsafe * RD_US.value_unsafe;
            SMI_US: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe;
            SCT: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe;
            TWP: Meter2 = 36.0 * SCT.value_unsafe;
            MIL_US: Meter = 1.0e-3 * IN_US.value_unsafe;
            IN_BR: Meter = 2.539998 * CM.value_unsafe;
            FT_BR: Meter = 12.0 * IN_BR.value_unsafe;
            RD_BR: Meter = 16.5 * FT_BR.value_unsafe;
            CH_BR: Meter = 4.0 * RD_BR.value_unsafe;
            LK_BR: Meter = CH_BR.value_unsafe / 100.0;
            FTH_BR: Meter = 6.0 * FT_BR.value_unsafe;
            PC_BR: Meter = 2.5 * FT_BR.value_unsafe;
            YD_BR: Meter = 3.0 * FT_BR.value_unsafe;
            MI_BR: Meter = 5280.0 * FT_BR.value_unsafe;
            NMI_BR: Meter = 6080.0 * FT_BR.value_unsafe;
            KN_BR: MeterPerSecond = NMI_BR.value_unsafe / HR.value_unsafe;
            ACR_BR: Meter2 = 4840.0 * YD_BR.value_unsafe * YD_BR.value_unsafe;
            GAL_US: Meter3 = 231.0 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
            BBL_US: Meter3 = 42.0 * GAL_US.value_unsafe;
            QT_US: Meter3 = GAL_US.value_unsafe / 4.0;
            PT_US: Meter3 = QT_US.value_unsafe / 2.0;
            GIL_US: Meter3 = PT_US.value_unsafe / 4.0;
            FOZ_US: Meter3 = GIL_US.value_unsafe / 4.0;
            FDR_US: Meter3 = FOZ_US.value_unsafe / 8.0;
            MIN_US: Meter3 = FDR_US.value_unsafe / 60.0;
            CRD_US: Meter3 = CR_I.value_unsafe;
            BU_US: Meter3 = 2150.42 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
            GAL_WI: Meter3 = BU_US.value_unsafe / 8.0;
            PK_US: Meter3 = BU_US.value_unsafe / 4.0;
            DQT_US: Meter3 = PK_US.value_unsafe / 8.0;
            DPT_US: Meter3 = DQT_US.value_unsafe / 2.0;
            TBS_US: Meter3 = FOZ_US.value_unsafe / 2.0;
            TSP_US: Meter3 = TBS_US.value_unsafe / 3.0;
            CUP_US: Meter3 = 16.0 * TBS_US.value_unsafe;
            FOZ_M: Meter3 = 30.0 * MILLI * L.value_unsafe;
            CUP_M: Meter3 = 240.0 * MILLI * L.value_unsafe;
            TSP_M: Meter3 = 5.0 * MILLI * L.value_unsafe;
            TBS_M: Meter3 = 15.0 * MILLI * L.value_unsafe;
            GAL_BR: Meter3 = 4.54609 * L.value_unsafe;
            PK_BR: Meter3 = 2.0 * GAL_BR.value_unsafe;
            BU_BR: Meter3 = 4.0 * PK_BR.value_unsafe;
            QT_BR: Meter3 = GAL_BR.value_unsafe / 4.0;
            PT_BR: Meter3 = QT_BR.value_unsafe / 2.0;
            GIL_BR: Meter3 = PT_BR.value_unsafe / 4.0;
            FOZ_BR: Meter3 = GIL_BR.value_unsafe / 5.0;
            FDR_BR: Meter3 = FOZ_BR.value_unsafe / 8.0;
            MIN_BR: Meter3 = FDR_BR.value_unsafe / 60.0;
            GR: Gram = 64.79891 * MILLI * G.value_unsafe;
            LB_AV: Gram = 7000.0 * GR.value_unsafe;
            OZ_AV: Gram = LB_AV.value_unsafe / 16.0;
            DR_AV: Gram = OZ_AV.value_unsafe / 16.0;
            SCWT_AV: Gram = 100.0 * LB_AV.value_unsafe;
            LCWT_AV: Gram = 112.0 * LB_AV.value_unsafe;
            STON_AV: Gram = 20.0 * SCWT_AV.value_unsafe;
            LTON_AV: Gram = 20.0 * LCWT_AV.value_unsafe;
            STONE_AV: Gram = 14.0 * LB_AV.value_unsafe;
            PWT_TR: Gram = 24.0 * GR.value_unsafe;
            OZ_TR: Gram = 20.0 * PWT_TR.value_unsafe;
            LB_TR: Gram = 12.0 * OZ_TR.value_unsafe;
            SC_AP: Gram = 20.0 * GR.value_unsafe;
            DR_AP: Gram = 3.0 * SC_AP.value_unsafe;
            OZ_AP: Gram = 8.0 * DR_AP.value_unsafe;
            LB_AP: Gram = 12.0 * OZ_AP.value_unsafe;
            OZ_M: Gram = 28.0 * G.value_unsafe;
            LNE: Meter = IN_I.value_unsafe / 12.0;
            PNT: Meter = LNE.value_unsafe / 6.0;
            PCA: Meter = 12.0 * PNT.value_unsafe;
            PNT_PR: Meter = 0.013837 * IN_I.value_unsafe;
            PCA_PR: Meter = 12.0 * PNT_PR.value_unsafe;
            PIED: Meter = 32.48 * CM.value_unsafe;
            POUNCE: Meter = PIED.value_unsafe / 12.0;
            LIGNE: Meter = POUNCE.value_unsafe / 12.0;
            DIDOT: Meter = LIGNE.value_unsafe / 6.0;
            CICERO: Meter = 12.0 * DIDOT.value_unsafe;
            DEGR: Kelvin = 5.0 / 9.0 * K.value_unsafe;
            CAL_15: MilliJoule = 4.18580 * J.value_unsafe;
            CAL_20: MilliJoule = 4.18190 * J.value_unsafe;
            CAL_M: MilliJoule = 4.19002 * J.value_unsafe;
            CAL_IT: MilliJoule = 4.1868 * J.value_unsafe;
            CAL_TH: MilliJoule = 4.184 * J.value_unsafe;
            CAL: MilliJoule = CAL_TH.value_unsafe;
            CAL_: MilliJoule = KILO * CAL.value_unsafe;
            BTU_39: MilliJoule = 1.05967 * KILO * J.value_unsafe;
            BTU_59: MilliJoule = 1.05480 * KILO * J.value_unsafe;
            BTU_60: MilliJoule = 1.05468 * KILO * J.value_unsafe;
            BTU_M: MilliJoule = 1.05587 * KILO * J.value_unsafe;
            BTU_IT: MilliJoule = 1.05505585262 * KILO * J.value_unsafe;
            BTU_TH: MilliJoule = 1.054350 * KILO * J.value_unsafe;
            BTU: MilliJoule = BTU_TH.value_unsafe;
            HP: MilliWatt = 550.0 * FT_I.value_unsafe * LBF_AV.value_unsafe / S.value_unsafe;
            TEX: GramPerMeter = 1.0 * G.value_unsafe / (KILO * M.value_unsafe);
            DEN: GramPerMeter = TEX.value_unsafe / 9.0;
            MH20: MilliPascal = 9.80665 * KILO * PA.value_unsafe;
            MHG: MilliPascal = 133.3220 * KILO * PA.value_unsafe;
            PRU: GramPerMeter4Second = MHG.value_unsafe * S.value_unsafe / L.value_unsafe;
            DIOP: PerMeter = 1.0 / M.value_unsafe;
            MESH_I: PerMeter = 1.0 / IN_I.value_unsafe;
            CH: Meter = 1.0 / 3.0 * MILLI * M.value_unsafe;
            DRP: Meter3 = MILLI * L.value_unsafe / 20.0;
            MET: Meter3PerSecondGram = 3.5 * MILLI * L.value_unsafe / MIN.value_unsafe / KG.value_unsafe;
            EQ: Unitless = MOL.value_unsafe;
            OSM: Unitless = MOL.value_unsafe;
            S_: Second = 1.0e-13 * S.value_unsafe;
            HPF: Unitless = 1.0 * ONE.value_unsafe;
            LPF: Unitless = 100.0 * ONE.value_unsafe;
            KAT: Hertz = MOL.value_unsafe / S.value_unsafe;
            U: Hertz = MICRO * MOL.value_unsafe / MIN.value_unsafe;
            STR: Meter3 = 1.0 * M3.value_unsafe;
            AO: Meter = 0.1 * NANO * M.value_unsafe;
            BRN: Meter2 = 100.0 * FEMTO * M.value_unsafe * FEMTO * M.value_unsafe;
            ATT: MilliPascal = KILO * GF.value_unsafe / CM.value_unsafe / CM.value_unsafe;
            MHO: KiloSiemens = MILLI * SIE.value_unsafe;
            PSI: MilliPascal = LBF_AV.value_unsafe / IN_I.value_unsafe / IN_I.value_unsafe;
            CIRC: Radian = 2.0 * consts::PI * RAD.value_unsafe;
            SPH: Steradian = 4.0 * consts::PI * SR.value_unsafe;
            CAR_M: Gram = 0.2 * G.value_unsafe;
            CAR_AU: Unitless = 1.0 / 24.0 * ONE.value_unsafe;
            SMOOT: Meter = 67.0 * IN_I.value_unsafe;
        }

        fmt = true;
    }

    #[cfg(feature = "serde")]
    impl_serde!(UCUM);
    #[cfg(feature = "clapme")]
    impl_clapme!(UCUM);

    pub use self::f64consts::*;


    #[test]
    fn test_ucum_constants() {
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts;

        assert_eq!(KG, KILO * G);
        assert_eq!(CM, CENTI * M);
        assert_eq!(MOL, 6.0221367e23 * ONE);
        assert_eq!(N, KILO * MILLIN);
        assert_eq!(PA, KILO * MILLIPA);
        assert_eq!(J, KILO * MILLIJ);
        assert_eq!(W, KILO * MILLIW);
        assert_eq!(V, KILO * MILLIV);
        assert_eq!(F, MILLI * KILOF);
        assert_eq!(OHM, KILO * MILLIOHM);
        assert_eq!(SIE, MILLI * KILOSIE);
        assert_eq!(WB, KILO * MILLIWB);
        assert_eq!(T, KILO * MILLIT);
        assert_eq!(H, KILO * MILLIH);
        assert_eq!(GON, 0.9 * DEG);
        assert_eq!(DEG, 2.0 * consts::PI / 360.0 * RAD);
        assert_eq!(L, 0.1 * 0.1 * 0.1 * M3);
        assert_eq!(AR, 100.0 * M2);
        assert_eq!(MIN, 60.0 * S);
        assert_eq!(HR, 60.0 * MIN);
        assert_eq!(D, 24.0 * HR);
        assert_eq!(ANN_T, 365.24219 * D);
        assert_eq!(ANN_J, 365.25 * D);
        assert_eq!(ANN_G, 365.2425 * D);
        assert_eq!(ANN, ANN_J);
        assert_eq!(WK, 7.0 * D);
        assert_eq!(MO_S, 29.53059 * D);
        assert_eq!(MO_J, ANN_J / 12.0);
        assert_eq!(MO_G, ANN_G / 12.0);
        assert_eq!(MO, MO_J);
        assert_eq!(TNE, 1.0e3 * KG);
        assert_eq!(BAR, 1.0e5 * PA);
        assert_eq!(AMU, 1.6605402e-24 * G);
        assert_eq!(EV, E * V);
        assert_eq!(ASU, 149597.870691e6 * M);
        assert_eq!(PRS, 3.085678e16 * M);
        assert_eq!(C_, 299792458.0 * MPS);
        assert_eq!(H_, 6.6260755e-34 * J * S);
        assert_eq!(K_, 1.380658e-23 * J / K);
        assert_eq!(EPS_0, 8.854187817e-12 * F / M);
        assert_eq!(MU_0, 4.0e-7 * consts::PI * N / A / A);
        assert_eq!(E, 1.6021766208e-19 * C);
        assert_eq!(M_E, 9.10938356e-31 * KG);
        assert_eq!(M_P, 1.6726231e-24 * G);
        assert_eq!(GC, 6.67259e-11 * M3 / KG / S2);
        assert_eq!(G_, 9.80665 * M / S2);
        assert_eq!(ATM, 101325.0 * PA);
        assert_eq!(LY, C_ * ANN_J);
        assert_eq!(GF, G * G_);
        assert_eq!(LBF_AV, LB_AV * G_);
        assert_eq!(KY, 1.0 / CM);
        assert_eq!(GL, CM / S2);
        assert_eq!(DYN, G * CM / S2);
        assert_eq!(ERG, DYN * CM);
        assert_eq!(P, DYN * S / CM / CM);
        assert_eq!(BI, 10.0 * A);
        assert_eq!(ST, CM * CM / S);
        assert_eq!(MX, 1.0e-8 * WB);
        assert_eq!(GS, 1.0e-4 * T);
        assert_eq!(OE, 250.0 / consts::PI * A / M);
        assert_eq!(GB, OE * CM);
        assert_eq!(SB, CD / CM / CM);
        assert_eq!(LMB, SB / consts::PI);
        assert_eq!(PHT, 1.0e-4 * LX);
        assert_eq!(CI, 3.7e10 * BQ);
        assert_eq!(ROE, 2.58e-4 * C / KG);
        assert_eq!(RAD_, 100.0 * ERG / G);
        assert_eq!(REM_, RAD_);
        assert_eq!(IN_I, 2.54 * CM);
        assert_eq!(FT_I, 12.0 * IN_I);
        assert_eq!(YD_I, 3.0 * FT_I);
        assert_eq!(MI_I, 5280.0 * FT_I);
        assert_eq!(FTH_I, 6.0 * FT_I);
        assert_eq!(NMI_I, 1852.0 * M);
        assert_eq!(KN_I, NMI_I / HR);
        assert_eq!(SIN_I, IN_I * IN_I);
        assert_eq!(SFT_I, FT_I * FT_I);
        assert_eq!(SYD_I, YD_I * YD_I);
        assert_eq!(CIN_I, IN_I * IN_I * IN_I);
        assert_eq!(CFT_I, FT_I * FT_I * FT_I);
        assert_eq!(CYD_I, YD_I * YD_I * YD_I);
        assert_eq!(BF_I, 144.0 * CIN_I);
        assert_eq!(CR_I, 128.0 * CFT_I);
        assert_eq!(MIL_I, 1.0e-3 * IN_I);
        assert_eq!(CML_I, consts::PI / 4.0 * MIL_I * MIL_I);
        assert_eq!(HD_I, 4.0 * IN_I);
        assert_eq!(FT_US, 1200.0 / 3937.0 * M);
        assert_eq!(YD_US, 3.0 * FT_US);
        assert_eq!(IN_US, FT_US / 12.0);
        assert_eq!(RD_US, 16.5 * FT_US);
        assert_eq!(CH_US, 4.0 * RD_US);
        assert_eq!(LK_US, CH_US / 100.0);
        assert_eq!(RCH_US, 100.0 * FT_US);
        assert_eq!(RLK_US, RCH_US / 100.0);
        assert_eq!(FTH_US, 6.0 * FT_US);
        assert_eq!(FUR_US, 40.0 * RD_US);
        assert_eq!(MI_US, 8.0 * FUR_US);
        assert_eq!(ACR_US, 160.0 * RD_US * RD_US);
        assert_eq!(SRD_US, RD_US * RD_US);
        assert_eq!(SMI_US, MI_US * MI_US);
        assert_eq!(SCT, MI_US * MI_US);
        assert_eq!(TWP, 36.0 * SCT);
        assert_eq!(MIL_US, 1.0e-3 * IN_US);
        assert_eq!(IN_BR, 2.539998 * CM);
        assert_eq!(FT_BR, 12.0 * IN_BR);
        assert_eq!(RD_BR, 16.5 * FT_BR);
        assert_eq!(CH_BR, 4.0 * RD_BR);
        assert_eq!(LK_BR, CH_BR / 100.0);
        assert_eq!(FTH_BR, 6.0 * FT_BR);
        assert_eq!(PC_BR, 2.5 * FT_BR);
        assert_eq!(YD_BR, 3.0 * FT_BR);
        assert_eq!(MI_BR, 5280.0 * FT_BR);
        assert_eq!(NMI_BR, 6080.0 * FT_BR);
        assert_eq!(KN_BR, NMI_BR / HR);
        assert_eq!(ACR_BR, 4840.0 * YD_BR * YD_BR);
        assert_eq!(GAL_US, 231.0 * IN_I * IN_I * IN_I);
        assert_eq!(BBL_US, 42.0 * GAL_US);
        assert_eq!(QT_US, GAL_US / 4.0);
        assert_eq!(PT_US, QT_US / 2.0);
        assert_eq!(GIL_US, PT_US / 4.0);
        assert_eq!(FOZ_US, GIL_US / 4.0);
        assert_eq!(FDR_US, FOZ_US / 8.0);
        assert_eq!(MIN_US, FDR_US / 60.0);
        assert_eq!(CRD_US, CR_I);
        assert_eq!(BU_US, 2150.42 * IN_I * IN_I * IN_I);
        assert_eq!(GAL_WI, BU_US / 8.0);
        assert_eq!(PK_US, BU_US / 4.0);
        assert_eq!(DQT_US, PK_US / 8.0);
        assert_eq!(DPT_US, DQT_US / 2.0);
        assert_eq!(TBS_US, FOZ_US / 2.0);
        assert_eq!(TSP_US, TBS_US / 3.0);
        assert_eq!(CUP_US, 16.0 * TBS_US);
        assert_eq!(FOZ_M, 30.0 * MILLI * L);
        assert_eq!(CUP_M, 240.0 * MILLI * L);
        assert_eq!(TSP_M, 5.0 * MILLI * L);
        assert_eq!(TBS_M, 15.0 * MILLI * L);
        assert_eq!(GAL_BR, 4.54609 * L);
        assert_eq!(PK_BR, 2.0 * GAL_BR);
        assert_eq!(BU_BR, 4.0 * PK_BR);
        assert_eq!(QT_BR, GAL_BR / 4.0);
        assert_eq!(PT_BR, QT_BR / 2.0);
        assert_eq!(GIL_BR, PT_BR / 4.0);
        assert_eq!(FOZ_BR, GIL_BR / 5.0);
        assert_eq!(FDR_BR, FOZ_BR / 8.0);
        assert_eq!(MIN_BR, FDR_BR / 60.0);
        assert_eq!(GR, 64.79891 * MILLI * G);
        assert_eq!(LB_AV, 7000.0 * GR);
        assert_eq!(OZ_AV, LB_AV / 16.0);
        assert_eq!(DR_AV, OZ_AV / 16.0);
        assert_eq!(SCWT_AV, 100.0 * LB_AV);
        assert_eq!(LCWT_AV, 112.0 * LB_AV);
        assert_eq!(STON_AV, 20.0 * SCWT_AV);
        assert_eq!(LTON_AV, 20.0 * LCWT_AV);
        assert_eq!(STONE_AV, 14.0 * LB_AV);
        assert_eq!(PWT_TR, 24.0 * GR);
        assert_eq!(OZ_TR, 20.0 * PWT_TR);
        assert_eq!(LB_TR, 12.0 * OZ_TR);
        assert_eq!(SC_AP, 20.0 * GR);
        assert_eq!(DR_AP, 3.0 * SC_AP);
        assert_eq!(OZ_AP, 8.0 * DR_AP);
        assert_eq!(LB_AP, 12.0 * OZ_AP);
        assert_eq!(OZ_M, 28.0 * G);
        assert_eq!(LNE, IN_I / 12.0);
        assert_eq!(PNT, LNE / 6.0);
        assert_eq!(PCA, 12.0 * PNT);
        assert_eq!(PNT_PR, 0.013837 * IN_I);
        assert_eq!(PCA_PR, 12.0 * PNT_PR);
        assert_eq!(PIED, 32.48 * CM);
        assert_eq!(POUNCE, PIED / 12.0);
        assert_eq!(LIGNE, POUNCE / 12.0);
        assert_eq!(DIDOT, LIGNE / 6.0);
        assert_eq!(CICERO, 12.0 * DIDOT);
        assert_eq!(DEGR, 5.0 / 9.0 * K);
        assert_eq!(CAL_15, 4.18580 * J);
        assert_eq!(CAL_20, 4.18190 * J);
        assert_eq!(CAL_M, 4.19002 * J);
        assert_eq!(CAL_IT, 4.1868 * J);
        assert_eq!(CAL_TH, 4.184 * J);
        assert_eq!(CAL, CAL_TH);
        assert_eq!(CAL_, KILO * CAL);
        assert_eq!(BTU_39, 1.05967 * KILO * J);
        assert_eq!(BTU_59, 1.05480 * KILO * J);
        assert_eq!(BTU_60, 1.05468 * KILO * J);
        assert_eq!(BTU_M, 1.05587 * KILO * J);
        assert_eq!(BTU_IT, 1.05505585262 * KILO * J);
        assert_eq!(BTU_TH, 1.054350 * KILO * J);
        assert_eq!(BTU, BTU_TH);
        assert_eq!(HP, 550.0 * FT_I * LBF_AV / S);
        assert_eq!(TEX, 1.0 * G / (KILO * M));
        assert_eq!(DEN, TEX / 9.0);
        assert_eq!(MH20, 9.80665 * KILO * PA);
        assert_eq!(MHG, 133.3220 * KILO * PA);
        assert_eq!(PRU, MHG * S / L);
        assert_eq!(DIOP, 1.0 / M);
        assert_eq!(MESH_I, 1.0 / IN_I);
        assert_eq!(CH, 1.0 / 3.0 * MILLI * M);
        assert_eq!(DRP, MILLI * L / 20.0);
        assert_eq!(MET, 3.5 * MILLI * L / MIN / KG);
        assert_eq!(EQ, MOL);
        assert_eq!(OSM, MOL);
        assert_eq!(S_, 1.0e-13 * S);
        assert_eq!(HPF, 1.0 * ONE);
        assert_eq!(LPF, 100.0 * ONE);
        assert_eq!(KAT, MOL / S);
        assert_eq!(U, MICRO * MOL / MIN);
        assert_eq!(STR, 1.0 * M3);
        assert_eq!(AO, 0.1 * NANO * M);
        assert_eq!(BRN, 100.0 * FEMTO * M * FEMTO * M);
        assert_eq!(ATT, KILO * GF / CM / CM);
        assert_eq!(MHO, MILLI * SIE);
        assert_eq!(PSI, LBF_AV / IN_I / IN_I);
        assert_eq!(CIRC, 2.0 * consts::PI * RAD);
        assert_eq!(SPH, 4.0 * consts::PI * SR);
        assert_eq!(CAR_M, 0.2 * G);
        assert_eq!(CAR_AU, 1.0 / 24.0 * ONE);
        assert_eq!(SMOOT, 67.0 * IN_I);
    }
    /// Test that serializing/deserializing values with units is
    /// equivalent to doing so with raw numeric types.
    #[cfg(feature = "serde")]
    #[test]
    fn test_ucum_serde() {
        use ::serde_test::{assert_tokens, Token};

        let value = 1.0 * M;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * S;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * G;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * RAD;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * K;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * C;
        assert_tokens(&value, &[Token::F64(1.0)]);

        let value = 1.0 * CD;
        assert_tokens(&value, &[Token::F64(1.0)]);

    }

    /// Test that clapme can generate a help message, and can produce a value.
    #[cfg(feature = "clapme")]
    #[test]
    fn test_ucum_clapme() {

        let value = 3.0 * M;
        assert_eq!(value,
                   <Meter<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * S;
        assert_eq!(value,
                   <Second<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * G;
        assert_eq!(value,
                   <Gram<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * RAD;
        assert_eq!(value,
                   <Radian<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * K;
        assert_eq!(value,
                   <Kelvin<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * C;
        assert_eq!(value,
                   <Coulomb<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

        let value = 3.0 * CD;
        assert_eq!(value,
                   <Candela<f64> as ClapMe>::from_iter(&["test", "3.0"]).unwrap());

    }
}