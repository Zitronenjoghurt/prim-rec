use crate::extensions::arithmetic::sub;
use crate::extensions::basic::one;
use crate::{comp, prim_rec, zero, FnPrimRec};

/// ```
/// use prim_rec::extensions::logic::is_positive;
///
/// let f_is_positive = is_positive();
///
/// assert_eq!(f_is_positive(vec![0]), 0);
/// assert_eq!(f_is_positive(vec![1]), 1);
/// ```
pub fn is_positive() -> FnPrimRec {
    prim_rec(zero(), one())
}

/// ```
/// use prim_rec::extensions::logic::is_zero;
///
/// let f_is_zero = is_zero();
///
/// assert_eq!(f_is_zero(vec![0]), 1);
/// assert_eq!(f_is_zero(vec![1]), 0);
/// ```
pub fn is_zero() -> FnPrimRec {
    prim_rec(one(), zero())
}

/// ```
/// use prim_rec::extensions::logic::greater_than;
///
/// let f_gt = greater_than();
///
/// assert_eq!(f_gt(vec![5, 3]), 1);
/// assert_eq!(f_gt(vec![3, 5]), 0);
/// ```
pub fn greater_than() -> FnPrimRec {
    comp(is_positive(), vec![sub()])
}
