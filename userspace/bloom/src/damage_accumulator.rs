use crate::damage::Rect;

pub const MAX_LOCAL_DAMAGE_RECTS: usize = 256;
const MERGE_FUZZ_MARGIN: i32 = 2;

pub struct DamageAccumulator<const MAX: usize> {
    rects: [Rect; MAX],
    len: usize,
    overflowed: bool,
}

impl<const MAX: usize> DamageAccumulator<MAX> {
    pub const fn new() -> Self {
        Self {
            rects: [Rect::new(0, 0, 0, 0); MAX],
            len: 0,
            overflowed: false,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.overflowed = false;
    }

    pub fn add(&mut self, rect: Rect) {
        if rect.is_empty() || self.overflowed {
            return;
        }

        if self.try_merge(rect) {
            return;
        }

        if self.len < MAX {
            self.rects[self.len] = rect;
            self.len += 1;
            self.consolidate();
        } else {
            self.overflowed = true;
        }
    }

    pub fn as_slice(&self) -> &[Rect] {
        &self.rects[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [Rect] {
        &mut self.rects[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_overflowed(&self) -> bool {
        self.overflowed
    }

    fn try_merge(&mut self, rect: Rect) -> bool {
        for i in 0..self.len {
            if Self::should_merge(self.rects[i], rect) {
                self.rects[i] = self.rects[i].union(rect);
                self.consolidate();
                return true;
            }
        }
        false
    }

    fn consolidate(&mut self) {
        if self.len <= 1 {
            return;
        }

        let mut changed = true;
        while changed {
            changed = false;
            'outer: for i in 0..self.len {
                for j in (i + 1)..self.len {
                    if Self::should_merge(self.rects[i], self.rects[j]) {
                        self.rects[i] = self.rects[i].union(self.rects[j]);
                        self.swap_remove(j);
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    fn swap_remove(&mut self, index: usize) {
        if index + 1 == self.len {
            self.len -= 1;
            return;
        }
        self.rects[index] = self.rects[self.len - 1];
        self.len -= 1;
    }

    fn should_merge(a: Rect, b: Rect) -> bool {
        if !a.intersect(b).is_empty() {
            return true;
        }

        let expanded_a = a.expand(MERGE_FUZZ_MARGIN);
        let expanded_b = b.expand(MERGE_FUZZ_MARGIN);
        !expanded_a.intersect(expanded_b).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_merges_overlapping_rects() {
        let mut acc = DamageAccumulator::<4>::new();
        acc.add(Rect::new(0, 0, 10, 10));
        acc.add(Rect::new(5, 5, 10, 10));
        assert_eq!(acc.as_slice().len(), 1);
        assert_eq!(acc.as_slice()[0], Rect::new(0, 0, 15, 15));
    }

    #[test]
    fn add_merges_touching_within_fuzz() {
        let mut acc = DamageAccumulator::<4>::new();
        acc.add(Rect::new(0, 0, 10, 10));
        acc.add(Rect::new(12, 0, 5, 5));
        assert_eq!(acc.as_slice().len(), 1);
        assert_eq!(acc.as_slice()[0], Rect::new(0, 0, 17, 10));
    }

    #[test]
    fn overflow_sets_flag() {
        let mut acc = DamageAccumulator::<2>::new();
        acc.add(Rect::new(0, 0, 1, 1));
        acc.add(Rect::new(10, 0, 1, 1));
        assert!(!acc.is_overflowed());
        acc.add(Rect::new(20, 0, 1, 1));
        assert!(acc.is_overflowed());
        assert_eq!(acc.as_slice().len(), 2);
    }
}
