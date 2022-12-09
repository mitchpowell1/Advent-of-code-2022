#[derive(Clone, Debug, Default, Copy)]
pub struct RealRange {
    end: usize,
    current: usize,
    ascending: bool,
    hit_bottom: bool,
}

impl Iterator for RealRange {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.current;

        if self.ascending {
            if curr >= self.end {
                return None;
            }

            self.current += 1;
        } else {
            if (self.current < self.end) || (self.current == self.end && self.hit_bottom) {
                return None
            }

            if self.current == self.end {
                self.hit_bottom = true;
            }

            if self.current != 0 {
                self.current -= 1;
            }
        }

        Some(curr)
    }
}

impl RealRange {
    pub fn ascending(start: usize, end: usize) -> Self {
        RealRange {
            end,
            ascending: true,
            current: start,
            hit_bottom: false,
        }
    }

    pub fn descending(start: usize, end: usize) -> Self {
        RealRange {
            end,
            ascending: false,
            current: start,
            hit_bottom: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_normal_vals() {
        let mut rng = RealRange::ascending(0, 5);
        assert_eq!(rng.next(), Some(0));
        assert_eq!(rng.next(), Some(1));
        assert_eq!(rng.next(), Some(2));
        assert_eq!(rng.next(), Some(3));
        assert_eq!(rng.next(), Some(4));
        assert_eq!(rng.next(), None);
    }

    #[test]
    fn ascending_bad_vals() {
        let mut rng = RealRange::ascending(5, 0);
        assert_eq!(rng.next(), None);
    }

    #[test]
    fn ascending_same_vals() {
        let mut rng = RealRange::ascending(3, 3);
        assert_eq!(rng.next(), None);
    }

    #[test]
    fn descending_normal_vals() {
        let mut rng = RealRange::descending(3, 0);
        assert_eq!(rng.next(), Some(3));
        assert_eq!(rng.next(), Some(2));
        assert_eq!(rng.next(), Some(1));
        assert_eq!(rng.next(), Some(0));
        assert_eq!(rng.next(), None);
    }

    #[test]
    fn descending_bad_vals() {
        let mut rng = RealRange::descending(0, 3);
        assert_eq!(rng.next(), None);
    }

    #[test]
    fn descending_same_vals() {
        let mut rng = RealRange::descending(3, 3);
        assert_eq!(rng.next(), Some(3));
    }
}
