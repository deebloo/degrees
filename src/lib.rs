use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Temp {
    C(f32),
    F(f32),
}

impl Temp {
    pub fn to_f(&self) -> Self {
        match self {
            Temp::C(temp) => Temp::F((((temp * 9.) / 5. + 32.) * 100.).round() / 100.),
            Temp::F(val) => Temp::F(*val),
        }
    }

    pub fn to_c(&self) -> Self {
        match self {
            Temp::C(val) => Temp::C(*val),
            Temp::F(val) => Temp::C((((val - 32.) * (5. / 9.)) * 100.).round() / 100.),
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

impl Sub for Temp {
    type Output = Temp;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.to_c().into();

                Self::C(val - target)
            }
            Self::F(val) => {
                let target: f32 = rhs.to_f().into();

                Self::F(val - target)
            }
        }
    }
}

impl Add for Temp {
    type Output = Temp;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.to_c().into();

                Self::C(val + target)
            }
            Self::F(val) => {
                let target: f32 = rhs.to_f().into();

                Self::F(val + target)
            }
        }
    }
}

impl Mul for Temp {
    type Output = Temp;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.to_c().into();

                Self::C(val * target)
            }
            Self::F(val) => {
                let target: f32 = rhs.to_f().into();

                Self::F(val * target)
            }
        }
    }
}

impl Div for Temp {
    type Output = Temp;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.to_c().into();

                Self::C(val / target)
            }
            Self::F(val) => {
                let target: f32 = rhs.to_f().into();

                Self::F(val / target)
            }
        }
    }
}

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source: f32 = (*self).to_c().into();
        let target: f32 = (*other).to_c().into();

        let mut res = std::cmp::Ordering::Equal;

        if source > target {
            res = std::cmp::Ordering::Greater;
        } else if source < target {
            res = std::cmp::Ordering::Less;
        }

        Some(res)
    }
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

    #[test]
    fn should_subtract_two_f_temps() {
        let res = Temp::F(100.) - Temp::F(100.);

        assert_eq!(res, Temp::F(0.));
    }

    #[test]
    fn should_subtract_two_c_temps() {
        let res = Temp::F(32.) - Temp::F(32.);

        assert_eq!(res, Temp::F(0.));
    }

    #[test]
    fn should_subtract_c_from_f() {
        let res = Temp::F(86.) - Temp::C(30.);

        assert_eq!(res, Temp::F(0.));
    }

    #[test]
    fn should_subtract_f_from_c() {
        let res = Temp::C(30.) - Temp::F(86.);

        assert_eq!(res, Temp::C(0.));
    }

    #[test]
    fn should_add_two_temps() {
        let res = Temp::F(100.) + Temp::F(100.);

        assert_eq!(res, Temp::F(200.));
    }

    #[test]
    fn should_multiply_two_temps() {
        let res = Temp::F(10.) * Temp::F(2.);

        assert_eq!(res, Temp::F(20.));
    }

    #[test]
    fn should_divide_two_temps() {
        let res = Temp::F(10.) / Temp::F(2.);

        assert_eq!(res, Temp::F(5.));
    }
}
