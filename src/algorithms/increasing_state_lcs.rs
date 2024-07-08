use crate::lcs_trait::Lcs;

#[derive(Debug, Clone, PartialEq)]
pub struct IncreasingStateLcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    state: Vec<usize>,
}

impl<'a> IncreasingStateLcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self {
            source,
            target,
            state: Self::create_state(source, target),
        }
    }

    fn create_state(source: &'a [u8], target: &'a [u8]) -> Vec<usize> {
        let source_len = source.len();
        let target_len = target.len();
        let mut previous = vec![0; target_len + 1];
        let mut current = vec![0; target_len + 1];
        for source_index in 0..source_len {
            for target_index in 0..target_len {
                if source[source_index] == target[target_index] {
                    current[target_index + 1] = previous[target_index] + 1;
                } else {
                    current[target_index + 1] =
                        current[target_index].max(previous[target_index + 1]);
                }
            }
            previous = current.clone();
        }
        current
    }
}

impl Lcs for IncreasingStateLcs<'_> {
    type Item = u8;

    fn subsequence(&self) -> Vec<Self::Item> {
        let mut last = 0;
        let length = self.len();
        let mut lcs: Vec<Self::Item> = Vec::with_capacity(length);
        for (target_item, state_item) in self.target.iter().zip(self.state.iter()) {
            if *state_item > last {
                last = *state_item;
                lcs.push(*target_item);
            }
        }
        lcs
    }

    fn len(&self) -> usize
    where
        Self: Sized,
    {
        *self.state.last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod increasing_state_lcs_tests {
    use crate::lcs_trait::Lcs;

    use super::IncreasingStateLcs;

    #[test]
    fn subsequence() {
        let source = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = vec![0, 0, 3, 1, 2, 1, 3, 4];
        let lcs = IncreasingStateLcs::new(&source, &target);
        assert_eq!(lcs.subsequence(), vec![0, 1, 1, 4]);
    }
}
