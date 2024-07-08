pub mod slow_lcs;
pub mod closest_offset_sum_lcs;
pub mod increasing_state_lcs;


#[cfg(test)]
mod lcs_tests {
    use crate::lcs_trait::Lcs;

    use super::{slow_lcs::SlowLcs, closest_offset_sum_lcs::ClosestOffsetSumLcs, increasing_state_lcs::IncreasingStateLcs};

    
    #[test]
    fn equality() {
        let source = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = vec![0, 0, 3, 1, 2, 1, 3, 4];
        let slow_lcs = SlowLcs::new(&source, &target);
        let closest_offset_sum_lcs = ClosestOffsetSumLcs::new(&source, &target);
        let increasing_state_lcs = IncreasingStateLcs::new(&source, &target);
        assert_eq!(slow_lcs.len(), 5);
        assert_eq!(closest_offset_sum_lcs.len(), 5);
        assert_eq!(increasing_state_lcs.len(), 4);
    }
}