use crate::Temp;
use std::ops::Sub;

impl Sub for Temp {
    type Output = Temp;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.as_c().into();

                Self::C(val - target)
            }
            Self::F(val) => {
                let target: f32 = rhs.as_f().into();

                Self::F(val - target)
            }
            Self::K(val) => {
                let target: f32 = rhs.as_k().into();

                Self::K(val - target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sub_from_c() {
        let cc = Temp::C(10.) - Temp::C(10.);
        let cf = Temp::C(10.) - Temp::F(50.);
        let ck = Temp::C(10.) - Temp::K(283.15);

        assert_eq!(cc, Temp::C(0.));
        assert_eq!(cf, Temp::C(0.));
        assert_eq!(ck, Temp::C(0.));
    }

    #[test]
    fn should_sub_from_f() {
        let ff = Temp::F(100.) - Temp::F(100.);
        let fc = Temp::F(100.) - Temp::C(37.778);
        let fk = Temp::F(100.) - Temp::K(310.928);

        assert_eq!(ff, Temp::F(0.));
        assert_eq!(fc, Temp::F(0.));
        assert_eq!(fk, Temp::F(0.));
    }

    #[test]
    fn should_sub_from_k() {
        let kk = Temp::K(300.) - Temp::K(300.);
        let kc = Temp::K(300.) - Temp::C(26.85);
        let kf = Temp::K(300.) - Temp::F(80.33);

        assert_eq!(kk, Temp::K(0.));
        assert_eq!(kf, Temp::K(0.));
        assert_eq!(kc, Temp::K(0.));
    }
}
