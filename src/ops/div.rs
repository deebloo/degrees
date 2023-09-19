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
            Self::K(val) => {
                let target: f32 = rhs.as_k().into();

                Self::K(val / target).round()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_div_from_c() {
        let cc = Temp::C(10.) / Temp::C(2.);
        let cf = Temp::C(10.) / Temp::F(35.6);
        let ck = Temp::C(10.) / Temp::K(275.15);

        assert_eq!(cc, Temp::C(5.));
        assert_eq!(cf, Temp::C(5.));
        assert_eq!(ck, Temp::C(5.));
    }

    #[test]
    fn should_div_from_f() {
        let ff = Temp::F(2.) / Temp::F(32.);
        let fc = Temp::F(2.) / Temp::C(0.);
        let fk = Temp::F(2.) / Temp::K(273.15);

        assert_eq!(ff, Temp::F(0.063));
        assert_eq!(fc, Temp::F(0.063));
        assert_eq!(fk, Temp::F(0.063));
    }

    #[test]
    fn should_div_from_k() {
        let kk = Temp::K(15.) / Temp::K(273.15);
        let kc = Temp::K(15.) / Temp::C(0.);
        let kf = Temp::K(15.) / Temp::F(32.);

        assert_eq!(kk, Temp::K(0.055));
        assert_eq!(kf, Temp::K(0.055));
        assert_eq!(kc, Temp::K(0.055));
    }
}
