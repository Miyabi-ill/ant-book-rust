extern crate test;

fn partial_sum(n: usize, a: Vec<i32>, k: i32) -> bool {
    search(&a, k, 0, 0)
}

// sumはindex - 1までのaの合計
fn search(a: &Vec<i32>, k: i32, index: usize, sum: i32) -> bool {
    if index == a.len() {
        return k == sum;
    }

    return search(a, k, index + 1, sum + a[index]) || search(a, k, index + 1, sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn partial_sum_answer_is_correct() {
        assert_eq!(true, partial_sum(3, vec![1, 2, 3], 6));
        assert_eq!(true, partial_sum(3, vec![3, 2, 1], 4));
        assert_eq!(true, partial_sum(4, vec![1, 1, 5, 2], 4));
        assert_eq!(true, partial_sum(4, vec![1, 1, 5, 2], 8));
        assert_eq!(false, partial_sum(4, vec![1, 1, 5, 2], 10));
        assert_eq!(true, partial_sum(4, vec![-1, 1, 5, -2], 4));
        assert_eq!(false, partial_sum(4, vec![-1, 1, 5, -2], 7));
    }

    #[bench]
    fn bench_partial_sum(b: &mut Bencher) {
        b.iter(|| partial_sum(20, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 500))
    }
}