use crate::utils::integer_utils::gcd;

#[derive(Debug, Clone)]
pub struct Rationals {
    pub under: i64,
    pub over: i64,
}

impl Rationals {
    pub fn put_to_denominator(&self, n: i64) -> Self {
        Rationals {
            under: self.under * n,
            over: self.over * n,
        }
    }
    pub fn new(under: i64, over: i64) -> Self {
        Rationals { under, over }
    }
    pub fn reduce(self) -> Self {
        let mut minus = false;
        let i1;
        let i2;
        if self.over < 0 && self.under > 0 {
            minus = true;
            i1 = -self.over;
            i2 = self.under;
        } else if self.over > 0 && self.under < 0 {
            minus = true;
            i1 = self.over;
            i2 = -self.under;
        } else {
            minus = false;
            i1 = self.over;
            i2 = self.under;
        }

        if i1 == 0 && i2 == 0 {
            return Rationals { under: 0, over: 0 };
        } else if i1 == 0 {
            return Rationals { under: 1, over: 0 };
        } else if i2 == 0 {
            return Rationals {
                under: 1,
                over: i64::MAX,
            };
        } else {
            let gcd = gcd(i1, i2);
            let new_under = self.under / gcd;
            let new_over = if minus {
                -(self.over / gcd)
            } else {
                self.over / gcd
            };
            return Rationals {
                under: new_under,
                over: new_over,
            };
        }
    }
}

impl PartialEq for Rationals {
    fn eq(&self, other: &Self) -> bool {
        if self.under == other.under {
            return self.over == other.over;
        } else {
            let i1 = self.put_to_denominator(other.under);
            let i2 = other.put_to_denominator(self.under);
            return i1.over == i2.over;
        }
    }
    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl PartialOrd for Rationals {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !(self.under == other.under) {
            if self.over == other.over {
                return Some(std::cmp::Ordering::Equal);
            }
            if self.over > other.over {
                return Some(std::cmp::Ordering::Greater);
            }
            if self.over < other.over {
                return Some(std::cmp::Ordering::Less);
            }
            None
        } else {
            let i1 = self.put_to_denominator(other.under);
            let i2 = other.put_to_denominator(self.under);
            if i1.over == i2.over {
                return Some(std::cmp::Ordering::Equal);
            }
            if i1.over > i2.over {
                return Some(std::cmp::Ordering::Greater);
            }
            if i1.over < i2.over {
                return Some(std::cmp::Ordering::Less);
            }
            None
        }
    }
    fn le(&self, other: &Self) -> bool {
        return self < other || self == other;
    }
    fn ge(&self, other: &Self) -> bool {
        return self > other || self == other;
    }
}

#[cfg(test)]
mod test {
    use super::Rationals;

    #[test]
    pub fn test_denominator() {
        let expected = Rationals::new(10, 30);
        let result = Rationals::new(2, 6).put_to_denominator(5);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_equality_simple() {
        let r1 = Rationals::new(2, 5);
        let r2 = Rationals::new(2, 3);
        assert_eq!(r1 > r2, true);
    }

    #[test]
    pub fn test_equality_hard() {
        let f1 = Rationals::new(2, 3);
        let f2 = Rationals::new(3, 1);
        assert_eq!(f2 < f1, true);
    }

    #[test]
    pub fn test_reduce_one() {
        let f1 = Rationals::new(10, 30);
        assert_eq!(Rationals::new(1, 3), f1.reduce());
    }

    #[test]
    pub fn test_reduce_two() {
        let f1 = Rationals::new(15, 9);
        assert_eq!(Rationals::new(5, 3), f1.reduce());
    }
}
