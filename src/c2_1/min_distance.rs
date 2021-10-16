extern crate test;

const INF: u32 = 100_000_000;

#[derive(PartialEq, Eq, Clone, Copy)]
enum MapInfo {
    Empty,
    Wall,
    Start,
    Goal,
}

struct Map {
    start: (usize, usize),
    goal: (usize, usize),
    map: Vec<Vec<MapInfo>>,
    distance: Vec<Vec<u32>>,
}

impl Map {
    fn create(map: Vec<Vec<MapInfo>>) -> Self {
        assert!(map.len() > 0);
        let mut distance = Vec::with_capacity(map.len());
        let mut start = None;
        let mut goal = None;
        for i in 0..map.len() {
            let mut vec = Vec::with_capacity(map[i].len());
            for j in 0..map[i].len() {
                vec.push(INF);
                if map[i][j] == MapInfo::Start {
                    start = Some((i, j));
                }
                if map[i][j] == MapInfo::Goal {
                    goal = Some((i, j));
                }
            }
            distance.push(vec);
        }
        return Self {
            start: start.unwrap(),
            goal: goal.unwrap(),
            map,
            distance,
        };
    }

    fn min_distance(mut self) -> Result<u32, ()> {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push(self.start);
        self.distance[self.start.0][self.start.1] = 0;
        while queue.len() > 0 {
            const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let current = queue.pop().unwrap();
            if current == self.goal {
                return Ok(self.distance[self.goal.0][self.goal.1]);
            }
            for direction in DIRECTIONS {
                let to = (
                    current.0 as i32 + direction.0,
                    current.1 as i32 + direction.1,
                );
                // 範囲外なら次の方向へ
                if to.0 < 0
                    || to.0 >= self.map.len() as i32
                    || to.1 < 0
                    || to.1 >= self.map[0].len() as i32
                {
                    continue;
                }
                let to = (to.0 as usize, to.1 as usize);
                // 既に訪れた場所、または行き先が壁なら次の方向へ
                if self.distance[to.0][to.1] != INF
                    || self.map[to.0][to.1] == MapInfo::Wall
                {
                    continue;
                }

                self.distance[to.0][to.1] = self.distance[current.0][current.1] + 1;
                queue.push(to);
            }
        }

        return Err(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use super::MapInfo::*;

    #[test]
    fn min_distance_answer_is_correct() {
        let correct_map = vec![
            vec![Wall, Start, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall],
            vec![Empty, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Wall, Wall, Wall, Empty, Wall, Wall, Wall, Empty],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Goal, Wall]
        ];
        let wrong_map = vec![
            vec![Wall, Start, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall],
            vec![Empty, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Wall, Wall, Wall, Empty, Wall, Wall, Wall, Empty],
            vec![Empty, Empty, Empty, Empty, Wall, Wall, Empty, Empty, Goal, Wall]
        ];
        let correct = Map::create(correct_map);
        let wrong = Map::create(wrong_map);
        assert_eq!(Ok(22), correct.min_distance());
        assert_eq!(Err(()), wrong.min_distance());
    }

    #[bench]
    fn bench_min_distance(b: &mut Bencher) {
        let correct_map = vec![
            vec![Wall, Start, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Wall],
            vec![Empty, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Empty, Wall],
            vec![Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            vec![Wall, Wall, Empty, Wall, Wall, Empty, Wall, Wall, Wall, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall],
            vec![Empty, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Empty, Wall],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty],
            vec![Empty, Wall, Wall, Wall, Wall, Empty, Wall, Wall, Wall, Empty],
            vec![Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Goal, Wall]
        ];
        
        b.iter(|| {
            let correct = Map::create(correct_map.clone());
            correct.min_distance()
        })
    }
}