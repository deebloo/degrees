use crate::Temp;
use std::ops::Div;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_divide_two_temps() {
        let res = Temp::F(10.) / Temp::F(2.);

        assert_eq!(res, Temp::F(5.));
    }
}
