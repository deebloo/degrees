use crate::Temp;

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source: f32 = (*self).as_c().into();
        let target: f32 = (*other).as_c().into();

        let mut res = std::cmp::Ordering::Equal;

        if source > target {
            res = std::cmp::Ordering::Greater;
        } else if source < target {
            res = std::cmp::Ordering::Less;
        }

        Some(res)
    }
}

impl PartialEq for Temp {
    fn eq(&self, other: &Self) -> bool {
        let source: f32 = (*self).as_c().into();
        let target: f32 = (*other).as_c().into();

        source == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        assert_eq!(Temp::F(86.) == Temp::C(30.), true);
        assert_eq!(Temp::F(86.) == Temp::K(303.15), true);
        assert_eq!(Temp::C(30.) == Temp::K(303.15), true);
    }

    #[test]
    fn gte() {
        assert_eq!(Temp::F(85.) >= Temp::F(80.), true);
    }

    #[test]
    fn lte() {
        assert_eq!(Temp::F(85.) <= Temp::F(112.), true);
        assert_eq!(Temp::F(85.) <= Temp::F(87.), true);
    }

    #[test]
    fn should_be_gt() {
        assert_eq!(Temp::F(86.) > Temp::C(0.), true);
        assert_eq!(Temp::F(86.) > Temp::C(100.), false);
    }
}
