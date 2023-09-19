use crate::Temp;
use std::ops::Div;

impl Div for Temp {
    type Output = Temp;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.as_c().into();

                Self::C(val / target).round()
            }
            Self::F(val) => {
                let target: f32 = rhs.as_f().into();

                Self::F(val / target).round()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_div_two_f_temps() {
        let res = Temp::F(10.) / Temp::F(2.);

        assert_eq!(res, Temp::F(5.));
    }

    #[test]
    fn should_div_two_c_temps() {
        let res = Temp::C(10.) / Temp::C(2.);

        assert_eq!(res, Temp::C(5.));
    }

    #[test]
    fn should_div_c_and_f() {
        let res = Temp::F(10.) / Temp::C(2.);

        assert_eq!(res, Temp::F(0.28));
    }

    #[test]
    fn should_div_f_and_c() {
        let res = Temp::C(30.) / Temp::F(86.);

        assert_eq!(res, Temp::C(1.));
    }
}
