use crate::Temp;
use std::ops::Sub;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_subtract_two_f_temps() {
        let res = Temp::F(100.) - Temp::F(100.);

        assert_eq!(res, Temp::F(0.));
    }

    #[test]
    fn should_subtract_two_c_temps() {
        let res = Temp::C(32.) - Temp::C(32.);

        assert_eq!(res, Temp::C(0.));
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
}
