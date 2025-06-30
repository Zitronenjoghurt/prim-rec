use crate::extensions::arithmetic::sub;
use crate::extensions::basic::{one, pred};
use crate::{comp, prim_rec, proj, zero, FnPrimRec};

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
/// assert_eq!(f_gt(vec![5, 4]), 1);
/// assert_eq!(f_gt(vec![3, 5]), 0);
/// assert_eq!(f_gt(vec![5, 5]), 0);
/// ```
pub fn greater_than() -> FnPrimRec {
    comp(is_positive(), vec![sub()])
}

/// ```
/// use prim_rec::extensions::logic::less_than;
///
/// let f_lt = less_than();
///
/// assert_eq!(f_lt(vec![5, 3]), 0);
/// assert_eq!(f_lt(vec![3, 5]), 1);
/// assert_eq!(f_lt(vec![3, 4]), 1);
/// assert_eq!(f_lt(vec![5, 5]), 0);
/// ```
pub fn less_than() -> FnPrimRec {
    comp(
        is_zero(),
        vec![comp(sub(), vec![proj(0), comp(pred(), vec![proj(1)])])],
    )
}

/// ```
/// use prim_rec::extensions::logic::greater_equal;
///
/// let f_geq = greater_equal();
///
/// assert_eq!(f_geq(vec![5, 3]), 1);
/// assert_eq!(f_geq(vec![3, 5]), 0);
/// assert_eq!(f_geq(vec![4, 5]), 0);
/// assert_eq!(f_geq(vec![5, 5]), 1);
/// ```
pub fn greater_equal() -> FnPrimRec {
    comp(is_zero(), vec![less_than()])
}

/// ```
/// use prim_rec::extensions::logic::less_equal;
///
/// let f_leq = less_equal();
///
/// assert_eq!(f_leq(vec![5, 3]), 0);
/// assert_eq!(f_leq(vec![3, 5]), 1);
/// assert_eq!(f_leq(vec![4, 5]), 1);
/// assert_eq!(f_leq(vec![5, 5]), 1);
/// ```
pub fn less_equal() -> FnPrimRec {
    comp(is_zero(), vec![greater_than()])
}

/// ```
/// use prim_rec::extensions::logic::if_then_else;
///
/// let f_ite = if_then_else();
///
/// assert_eq!(f_ite(vec![1, 2, 3]), 2);
/// assert_eq!(f_ite(vec![0, 2, 3]), 3);
/// ```
pub fn if_then_else() -> FnPrimRec {
    prim_rec(proj(1), proj(2))
}
