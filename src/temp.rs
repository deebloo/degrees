use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
}
