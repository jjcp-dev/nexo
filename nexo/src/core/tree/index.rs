use std::fmt;

type UInt = u16;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index {
    index: UInt,
}

impl Index {
    #[inline]
    pub fn new(index: usize) -> Option<Index> {
        if index <= Index::max() as usize {
            Some(Index {
                index: index as UInt,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn zero() -> Index {
        Index { index: 0 }
    }

    // pub unsafe fn new_unchecked(index: usize) -> Index {
    //     Index {
    //         index: index as UInt,
    //     }
    // }

    #[inline]
    pub fn value(&self) -> usize {
        self.index as usize
    }

    #[inline]
    pub fn null() -> Index {
        Index { index: UInt::MAX }
    }

    #[inline]
    pub fn max() -> usize {
        UInt::MAX as usize - 1
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Index::null()
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_null() {
            f.debug_tuple("Index").field(&self.index).finish()
        } else {
            f.debug_tuple("Index").field(&(-1)).finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_given_a_valid_index_returns_some_index() {
        assert!(Index::new(0).is_some());
        assert!(Index::new(1).is_some());
    }

    #[test]
    fn new_given_an_out_of_bounds_index_returns_none() {
        assert!(Index::new(usize::MAX).is_none());
        assert!(Index::new(Index::max() + 1).is_none());
    }

    // #[test]
    // fn new_unchecked_given_any_number_returns_an_index() {
    //     unsafe {
    //         assert_eq!(Index::new_unchecked(0).value(), 0);
    //         assert_eq!(Index::new_unchecked(Index::max()).value(), Index::max());
    //         assert_eq!(
    //             Index::new_unchecked(Index::max() + 1).value(),
    //             Index::max() + 1
    //         );
    //     }
    // }

    #[test]
    fn zero_returns_an_index_with_value_zero() {
        let a = Index::zero();
        assert_eq!(a.index, 0);
    }

    #[test]
    fn value_given_an_index_returns_a_usable_index_number() {
        assert_eq!(Index::new(10).unwrap().value(), 10);
        assert_eq!(Index::new(Index::max()).unwrap().value(), Index::max());
    }

    #[test]
    fn null_returns_a_null_index() {
        let a = Index::null();
        assert_eq!(a.index, UInt::MAX);
    }

    #[test]
    fn is_null_given_a_null_index_returns_true() {
        let a = Index::null();
        assert!(a.is_null());
    }

    #[test]
    fn is_null_given_a_not_null_index_returns_false() {
        let a = Index::new(0).unwrap();
        assert!(!a.is_null());

        let a = Index::new(1).unwrap();
        assert!(!a.is_null());

        let a = Index::new(10).unwrap();
        assert!(!a.is_null());

        let a = Index::new(Index::max()).unwrap();
        assert!(!a.is_null());
    }

    #[test]
    fn max_returns_the_biggest_index_possible() {
        assert!(Index::max() > 0);

        let a = Index::new(Index::max());
        assert!(a.is_some());

        let a = Index::new(Index::max() + 1);
        assert!(a.is_none());
    }

    #[test]
    fn debug_fmt() {
        assert_eq!(format!("{:?}", Index::new(10).unwrap()), "Index(10)");
        assert_eq!(format!("{:?}", Index::null()), "Index(-1)");
    }
}
