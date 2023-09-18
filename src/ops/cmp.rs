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
