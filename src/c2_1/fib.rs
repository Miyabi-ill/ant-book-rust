extern crate test;

struct Cache {
    fib_cache: Vec<usize>,
}

impl Default for Cache {
    fn default() -> Self {
        let mut cache = Self { fib_cache: Vec::with_capacity(100) };
        cache.fib_cache.push(1);
        return cache
    }
}

impl Cache {
    fn fib(&mut self, n: usize) -> usize {
        if n <= 0 {
            return n;
        }
        
        if self.fib_cache.len() >= n {
            return self.fib_cache[n - 1];
        }

        let result = self.fib(n - 2) + self.fib(n - 1);
        self.fib_cache.push(result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn fib_answer_is_correct() {
        let mut cache: Cache = Default::default();
        assert_eq!(1, cache.fib(1));
        assert_eq!(1, cache.fib(2));
        assert_eq!(2, cache.fib(3));
        assert_eq!(3, cache.fib(4));
        assert_eq!(5, cache.fib(5));
        assert_eq!(8, cache.fib(6));
        assert_eq!(13, cache.fib(7));
    }

    #[bench]
    fn bench_fib(b: &mut Bencher) {
        let mut cache: Cache = Default::default();
        cache.fib(50);
        b.iter(|| cache.fib(50))
    }
}
