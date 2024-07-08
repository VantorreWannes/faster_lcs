
pub fn index_lut(slice: &[u8]) -> [Vec<u8>; 256] {
    let mut lut: [Vec<u8>; 256] =
        count_lut(slice).map(|count| Vec::with_capacity(count as usize));
    for (i, num) in slice.iter().enumerate() {
        lut[*num as usize].push(i as u8);
    }
    lut
}

pub fn count_lut(slice: &[u8]) -> [u8; 256] {
    let mut lut = [0; 256];
    for num in slice.iter() {
        lut[*num as usize] += 1;
    }
    return lut;
}

pub fn filter_non_occuring(slice: &[u8], other: &[u8]) -> Vec<u8> {
    let other_count_lut = count_lut(other);
    slice
        .iter()
        .filter(|item| other_count_lut[**item as usize] != 0)
        .copied()
        .collect()
}

#[cfg(test)]
mod filter_tests {
    use super::{count_lut, filter_non_occuring, index_lut};

    #[test]
    fn test_count_lut() {
        let slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let counts = count_lut(&slice);
        assert_eq!(counts[..10], vec![1; 10]);
    }

    #[test]
    fn test_index_lut() {
        let slice = [0, 0, 0, 1, 1, 2, 3];
        assert_eq!(index_lut(&slice)[..3], vec![vec![0, 1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_filter_non_occuring() {
        let slice = [0, 0, 1, 2, 3, 4, 3];
        let other = [0, 2, 4];
        assert_eq!(filter_non_occuring(&slice, &other), vec![0, 0, 2, 4]);
    }
}