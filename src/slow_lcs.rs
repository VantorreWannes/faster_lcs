use crate::lcs_trait::Lcs;


#[derive(Debug, Clone, PartialEq)]
pub struct SlowLcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    table: Vec<Vec<usize>>,
}

impl<'a> SlowLcs<'a> {
    ///Max length for Source and target is `u8::MAX`!
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        let source_length = source.len();
        let target_length = target.len();
        let mut table: Vec<Vec<usize>> =
            vec![vec![0; target_length + 1]; source_length + 1];

        for x in 0..=source_length {
            for y in 0..=target_length {
                if x == 0 || y == 0 {
                    table[x][y] = 0
                } else if source[x - 1] == target[y - 1] {
                    table[x][y] = table[x - 1][y - 1] + 1
                } else {
                    table[x][y] = table[x - 1][y].max(table[x][y - 1])
                }
            }
        }

        Self {
            table,
            source,
            target,
        }
    }
}

impl<'a> Lcs for SlowLcs<'a> {
    type Item = u8;
    ///Returns the longest common subsequence in source and target
    fn subsequence(&self) -> Vec<u8> {
        let mut index = self.len();
        let mut subsequence: Vec<u8> = vec![0; index + 1];

        let mut x = self.source.len();
        let mut y = self.target.len();
        while x > 0 && y > 0 {
            if self.source[x - 1] == self.target[y - 1] {
                subsequence[index - 1] = self.source[x - 1];
                x -= 1;
                y -= 1;
                index -= 1
            } else if self.table[x - 1][y] > self.table[x][y - 1] {
                x -= 1
            } else {
                y -= 1
            }
        }

        subsequence.pop();
        subsequence
    }
}

#[cfg(test)]
mod lcs_tests {
    use std::vec;

    use super::*;

    #[test]
    fn is_empty() {
        let source = vec![0; 100];
        let target = vec![];
        assert_eq!(SlowLcs::new(&source, &target).len(), 0);
    }

    #[test]
    fn len() {
        let source = vec![0; 100];
        let target = source.clone();
        assert_eq!(SlowLcs::new(&source, &target).len(), 100);
    }

    #[test]
    fn equal_subsequence() {
        let source: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = source.clone();
        let lcs = SlowLcs::new(&source, &target);
        let subsequence = lcs.subsequence();
        assert_eq!(subsequence, source);
        assert_eq!(subsequence.len(), lcs.len());
    }
    
    #[test]
    fn custom_subsequence() {
        let source: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = vec![0, 1, 0, 3, 3, 5, 7, 7, 8, 9];
        let lcs = SlowLcs::new(&source, &target);
        assert_eq!(lcs.subsequence(), vec![0, 1, 3, 5, 7, 8, 9]);
    }
}