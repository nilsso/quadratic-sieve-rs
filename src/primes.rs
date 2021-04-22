pub struct PrimeBank {
    grow_by: usize,
    last_found_prime: usize,
    sieve: Vec<bool>,
    primes: Vec<i32>,
}

impl PrimeBank {
    pub fn new(grow_by: usize) -> Self {
        let mut sieve = vec![true; grow_by];
        sieve[0] = false;
        sieve[1] = false;

        let mut res = Self {
            grow_by,
            last_found_prime: 0,
            sieve,
            primes: Vec::new(),
        };
        res.mark();
        res
    }

    pub fn mark(&mut self) {
        for i in 2..self.sieve.len() {
            if self.sieve[i] && i > self.last_found_prime {
                self.last_found_prime = i;
                self.primes.push(i as i32);
            }
            for j in ((i * 2)..self.sieve.len()).step_by(i) {
                self.sieve[j] = false;
            }
        }
    }

    pub fn extend(&mut self) {
        use std::iter::repeat;

        self.sieve.extend(repeat(true).take(self.grow_by));
        self.mark();
    }

    pub fn extend_until(&mut self, i: usize) {
        while self.primes.len() <= i {
            self.extend();
        }
    }

    pub fn ith_prime(&mut self, i: usize) -> i32 {
        self.extend_until(i);
        self.primes[i]
    }
}

impl Default for PrimeBank {
    fn default() -> PrimeBank {
        PrimeBank::new(1000)
    }
}

pub struct PrimeIter<'a> {
    prime_bank: &'a mut PrimeBank,
    i: usize,
}

impl PrimeBank {
    pub fn iter(&mut self) -> PrimeIter {
        PrimeIter {
            prime_bank: self,
            i: 0,
        }
    }
}

impl<'a> Iterator for PrimeIter<'a> {
    type Item = i32;
    //
    fn next(&mut self) -> Option<i32> {
        let res = self.prime_bank.ith_prime(self.i);
        self.i += 1;
        Some(res)
    }
}
