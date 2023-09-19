use crate::Temp;
use std::ops::Add;

impl Add for Temp {
    type Output = Temp;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::C(val) => {
                let target: f32 = rhs.as_c().into();

                Self::C(val + target)
            }
            Self::F(val) => {
                let target: f32 = rhs.as_f().into();

                Self::F(val + target)
            }
            Self::K(val) => {
                let target: f32 = rhs.as_k().into();

                Self::F(val + target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_two_f_temps() {
        let res = Temp::F(100.) + Temp::F(100.);

        assert_eq!(res, Temp::F(200.));
    }

    #[test]
    fn should_add_two_c_temps() {
        let res = Temp::C(32.) + Temp::C(32.);

        assert_eq!(res, Temp::C(64.));
    }

    #[test]
    fn should_add_c_to_f() {
        let res = Temp::F(86.) + Temp::C(30.);

        assert_eq!(res, Temp::F(172.));
    }

    #[test]
    fn should_add_f_to_c() {
        let res = Temp::C(30.) + Temp::F(86.);

        assert_eq!(res, Temp::C(60.));
    }
}
