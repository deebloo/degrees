use crate::Temp;
use std::ops::Add;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_two_temps() {
        let res = Temp::F(100.) + Temp::F(100.);

        assert_eq!(res, Temp::F(200.));
    }
}
