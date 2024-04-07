/// A function that takes an integer greater than 1 and returns a vector of its
/// prime factors, which are the prime numbers that divide that integer exactly, in
/// ascending order. For example, the prime factors of 12 are 2, 2, 3.

pub fn prime_factors(num: i32) -> Vec<i32> {
    if num % 2 == 0 && num > 2 {
        let mut answer = vec![2];
        answer.extend(&prime_factors(num / 2));
        return answer;
    }
    vec![num]
}

pub fn prime_factors2(mut num: i32) -> Vec<i32> {
    let mut answer = vec![];
    while num != 1 {
        let mut candidate = 2;
        while candidate <= num {
            if num % candidate == 0 {
                answer.push(candidate);
                num = num / candidate;
                break;
            } else {
                candidate += 1;
            }
        }
    }
    answer
}

pub fn prime_factors3(mut num: i32) -> Vec<i32> {
    let mut answer = vec![];
    while num != 1 {
        for candidate in 2..=num {
            if num % candidate == 0 {
                answer.push(candidate);
                num = num / candidate;
                break;
            }
        }
    }
    answer
}

pub fn prime_factors4(mut num: i32) -> Vec<i32> {
    let mut answer = vec![];
    let mut candidate = 2;
    while num > 1 {
        while num % candidate == 0 {
            answer.push(candidate);
            num /= candidate;
        }
        candidate += 1;
    }
    answer
}

pub fn prime_factors5(num: i32) -> Vec<i32> {
    for candidate in 2..num {
        if num % candidate == 0 {
            let mut answer = vec![candidate];
            answer.extend(&prime_factors(num / candidate));
            return answer;
        }
    }
    vec![num]
}

pub struct PrimeFactorsIterator {
    num: i32,
    candidate: i32,
}

impl PrimeFactorsIterator {
    pub fn new(num: i32) -> PrimeFactorsIterator {
        PrimeFactorsIterator { num, candidate: 2 }
    }
}

impl Iterator for PrimeFactorsIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        while self.num > 1 {
            while self.num % self.candidate == 0 {
                self.num /= self.candidate;
                return Some(self.candidate);
            }
            self.candidate += 1;
        }
        None
    }
}

pub fn prime_factors6(num: i32) -> Vec<i32> {
    PrimeFactorsIterator::new(num).collect()
}

#[cfg(test)]
mod tests {
    use crate::prime_factors;
    use crate::prime_factors2;
    use crate::prime_factors3;
    use crate::prime_factors4;
    use crate::prime_factors5;
    use crate::prime_factors6;

    #[test]
    fn prime_factors_of_two() {
        assert_eq!(prime_factors(3), [3]);
        assert_eq!(prime_factors(4), [2, 2]);
        assert_eq!(prime_factors(5), [5]);
        assert_eq!(prime_factors(6), [2, 3]);
        assert_eq!(prime_factors(7), [7]);
        assert_eq!(prime_factors(8), [2, 2, 2]);

        assert_eq!(prime_factors2(9), [3, 3]);
        assert_eq!(prime_factors2(48), [2, 2, 2, 2, 3]);
        assert_eq!(prime_factors2(101), [101]);

        assert_eq!(prime_factors3(101), [101]);

        assert_eq!(prime_factors4(101), [101]);

        assert_eq!(prime_factors5(101), [101]);

        assert_eq!(prime_factors6(101), [101]);
    }
}
