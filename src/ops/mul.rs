use crate::Temp;
use std::ops::Mul;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mul_two_f_temps() {
        let res = Temp::F(10.) * Temp::F(2.);

        assert_eq!(res, Temp::F(20.));
    }

    #[test]
    fn should_mul_two_c_temps() {
        let res = Temp::C(10.) * Temp::C(2.);

        assert_eq!(res, Temp::C(20.));
    }

    #[test]
    fn should_mul_c_and_f() {
        let res = Temp::F(10.) * Temp::C(2.);

        assert_eq!(res, Temp::F(356.));
    }

    #[test]
    fn should_mul_f_and_c() {
        let res = Temp::C(30.) * Temp::F(86.);

        assert_eq!(res, Temp::C(900.));
    }
}
