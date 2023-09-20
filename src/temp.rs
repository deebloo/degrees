#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Temp {
    C(f32),
    F(f32),
    K(f32),
}

impl Temp {
    pub fn as_c(&self) -> Self {
        match self {
            Self::C(val) => Self::C(round(*val)),
            Self::F(val) => {
                let raw = (val - 32.) * (5. / 9.);

                Temp::C(round(raw))
            }
            Self::K(val) => Temp::C(round(val - 273.15)),
        }
    }

    pub fn as_f(&self) -> Self {
        match self {
            Self::C(temp) => {
                let raw = (temp * (9. / 5.)) + 32.;

                Temp::F(round(raw))
            }
            Self::F(val) => Self::F(round(*val)),
            Self::K(temp) => {
                let raw = (temp - 273.15) * (9. / 5.) + 32.;

                Self::F(round(raw))
            }
        }
    }

    pub fn as_k(&self) -> Self {
        match self {
            Self::C(val) => Self::K(val + 273.15),
            Self::F(_) => {
                let c: f32 = self.as_c().into();

                Temp::K(c + 273.15)
            }
            Self::K(val) => Self::K(round(*val)),
        }
    }

    pub fn round(&self) -> Self {
        match self {
            Self::C(val) => Self::C(round(*val)),
            Self::F(val) => Self::F(round(*val)),
            Self::K(val) => Self::K(round(*val)),
        }
    }
}

impl Into<f32> for Temp {
    fn into(self) -> f32 {
        match self {
            Self::C(val) => val,
            Self::F(val) => val,
            Self::K(val) => val,
        }
    }
}

pub fn round(val: f32) -> f32 {
    let res = (val * 1000.).round() / 1000.;

    res
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData {
        temperatureconversions: Vec<Conversion>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        celsius: f32,
        fahrenheit: f32,
        kelvin: f32,
    }

    #[test]
    fn should_convert_correctly() {
        let data_string = fs::read_to_string("data/conversions.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.temperatureconversions {
            // convert to celcius
            assert_eq!(Temp::F(entry.fahrenheit).as_c(), Temp::C(entry.celsius));
            assert_eq!(Temp::K(entry.kelvin).as_c(), Temp::C(entry.celsius));

            // convert to farenheit
            assert_eq!(Temp::C(entry.celsius).as_f(), Temp::F(entry.fahrenheit));
            assert_eq!(Temp::K(entry.kelvin).as_f(), Temp::F(entry.fahrenheit));

            // convert to kelvin
            assert_eq!(Temp::C(entry.celsius).as_k(), Temp::K(entry.kelvin));
            assert_eq!(Temp::F(entry.fahrenheit).as_k(), Temp::K(entry.kelvin));
        }
    }
}
