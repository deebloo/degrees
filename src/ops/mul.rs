use crate::Temp;
use std::ops::Mul;

impl Mul for Temp {
    type Output = Temp;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.as_c().into();

                Self::C(val * target)
            }
            Self::F(val) => {
                let target: f32 = rhs.as_f().into();

                Self::F(val * target)
            }
            Self::K(val) => {
                let target: f32 = rhs.as_k().into();

                Self::K(val * target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mul_to_c() {
        let cc = Temp::C(10.) * Temp::C(2.);
        let cf = Temp::C(10.) * Temp::F(35.6);
        let ck = Temp::C(10.) * Temp::K(275.15);

        assert_eq!(cc, Temp::C(20.));
        assert_eq!(cf, Temp::C(20.));
        assert_eq!(ck, Temp::C(20.));
    }

    #[test]
    fn should_mul_to_f() {
        let ff = Temp::F(2.) * Temp::F(32.);
        let fc = Temp::F(2.) * Temp::C(0.);
        let fk = Temp::F(2.) * Temp::K(273.15);

        assert_eq!(ff, Temp::F(64.));
        assert_eq!(fc, Temp::F(64.));
        assert_eq!(fk, Temp::F(64.));
    }

    #[test]
    fn should_mul_to_k() {
        let kk = Temp::K(15.) * Temp::K(273.15);
        let kc = Temp::K(15.) * Temp::C(0.);
        let kf = Temp::K(15.) * Temp::F(32.);

        assert_eq!(kk, Temp::K(4097.25));
        assert_eq!(kf, Temp::K(4097.25));
        assert_eq!(kc, Temp::K(4097.25));
    }
}
