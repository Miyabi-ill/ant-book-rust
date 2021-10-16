extern crate test;

fn lake_counting(n: usize, m: usize, map: Vec<Vec<bool>>) -> usize {
    let mut mut_map = map.clone();
    let mut count = 0;
    for x in 0..m {
        for y in 0..n {
            if mut_map[x][y] {
                fill(x as isize, y as isize, &mut mut_map);
                count += 1;
            }
        }
    }
    return count
}

fn fill(x: isize, y: isize, map: &mut Vec<Vec<bool>>) {
    for i in -1..=1 {
        for j in -1..=1 {
            if (i == -1 && x == 0) || (j == -1 && y == 0) {
                continue;
            }

            if x + i >= map.len() as isize || y + j >= map[0].len() as isize {
                continue;
            }

            if i == 0 && j == 0 {
                map[x as usize][y as usize] = false;
                continue;
            }

            if map[(x + i) as usize][(y + j) as usize] {
                fill(x + i, y + j, map);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn lake_counting_answer_is_correct() {
        let map = vec![
            vec![true, false, false, false, false, false, false, false, false, true, true, false],
            vec![false, true, true, true, false, false, false, false, false, true, true, true],
            vec![false, false, false, false, true, true, false, false, false, true, true, false],
            vec![false, false, false, false, false, false, false, false, false, true, true, false],
            vec![false, false, false, false, false, false, false, false, false, true, false, false],
            vec![false, false, true, false, false, false, false, false, false, true, false, false],
            vec![false, true, false, true, false, false, false, false, false, true, true, false],
            vec![true, false, true, false, true, false, false, false, false, false, true, false],
            vec![false, true, false, true, false, false, false, false, false, false, true, false],
            vec![false, false, true, false, false, false, false, false, false, false, true, false],
        ];
        assert_eq!(3, lake_counting(12, 10, map));
    }

    #[bench]
    fn bench_lake_counting(b: &mut Bencher) {
        let map = vec![
            vec![true, false, false, false, false, false, false, false, false, true, true, false],
            vec![false, true, true, true, false, false, false, false, false, true, true, true],
            vec![false, false, false, false, true, true, false, false, false, true, true, false],
            vec![false, false, false, false, false, false, false, false, false, true, true, false],
            vec![false, false, false, false, false, false, false, false, false, true, false, false],
            vec![false, false, true, false, false, false, false, false, false, true, false, false],
            vec![false, true, false, true, false, false, false, false, false, true, true, false],
            vec![true, false, true, false, true, false, false, false, false, false, true, false],
            vec![false, true, false, true, false, false, false, false, false, false, true, false],
            vec![false, false, true, false, false, false, false, false, false, false, true, false],
        ];
        b.iter(|| lake_counting(12, 10, map.clone()))
    }
}