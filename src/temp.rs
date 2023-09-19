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
            Self::K(val) => Temp::C(val - 273.15),
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
    let res = (val * 100.).round() / 100.;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_c() {
        assert_eq!(Temp::F(86.).as_c(), Temp::C(30.));
    }

    #[test]
    fn should_convert_to_f() {
        assert_eq!(Temp::C(30.).as_f(), Temp::F(86.));
    }
}
