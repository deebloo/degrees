use crate::Temp;

impl PartialOrd for Temp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let source: f32 = (*self).to_c().into();
        let target: f32 = (*other).to_c().into();

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
        let source: f32 = (*self).to_c().into();
        let target: f32 = (*other).to_c().into();

        source == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_equal() {
        assert_eq!(Temp::F(86.) == Temp::C(30.), true);
        assert_eq!(Temp::F(86.) == Temp::C(32.), false);
    }

    #[test]
    fn should_be_gt() {
        assert_eq!(Temp::F(86.) > Temp::C(0.), true);
        assert_eq!(Temp::F(86.) > Temp::C(100.), false);
    }
}
