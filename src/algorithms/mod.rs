pub mod slow_lcs;
pub mod closest_offset_sum_lcs;
pub mod increasing_state_lcs;


#[cfg(test)]
mod lcs_tests {
    use rand::distributions::{Distribution, Uniform};

    use crate::lcs_trait::Lcs;

    use super::{slow_lcs::SlowLcs, closest_offset_sum_lcs::ClosestOffsetSumLcs, increasing_state_lcs::IncreasingStateLcs};

    
    #[test]
    fn equality() {
        let mut rng = rand::thread_rng();
        let die: Uniform<u8> = Uniform::from(0..=255);
        let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
        let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
        let slow_lcs = SlowLcs::new(&source, &target);
        let closest_offset_sum_lcs = ClosestOffsetSumLcs::new(&source, &target);
        let increasing_state_lcs = IncreasingStateLcs::new(&source, &target);
        assert_eq!(slow_lcs.len(), increasing_state_lcs.len());
        println!("Slow lcs length: {}", slow_lcs.len());
        println!("Closest offset sum lcs length: {}", closest_offset_sum_lcs.len());
        println!("Increasing state lcs length: {}", increasing_state_lcs.len());
    }
}