#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Temp {
    C(f32),
    F(f32),
}

impl Temp {
    pub fn to_f(&self) -> Self {
        match self {
            Temp::C(temp) => {
                let raw = (temp * 9.) / 5. + 32.;

                Temp::F(round(raw))
            }
            Temp::F(val) => Temp::F(round(*val)),
        }
    }

    pub fn to_c(&self) -> Self {
        match self {
            Temp::C(val) => Temp::C(round(*val)),
            Temp::F(val) => {
                let raw = (val - 32.) * (5. / 9.);

                Temp::C(round(raw))
            }
        }
    }
}

impl Into<f32> for Temp {
    fn into(self) -> f32 {
        match self {
            Self::C(val) => val,
            Self::F(val) => val,
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
        assert_eq!(Temp::F(86.).to_c(), Temp::C(30.));
    }

    #[test]
    fn should_convert_to_f() {
        assert_eq!(Temp::C(30.).to_f(), Temp::F(86.));
    }
}
