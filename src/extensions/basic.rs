use crate::{comp, prim_rec, proj, succ, zero, FnPrimRec};

///```
/// use prim_rec::extensions::basic::pred;
///
/// let f_pred = pred();
///
/// assert_eq!(f_pred(vec![0]), 0);
/// assert_eq!(f_pred(vec![1]), 0);
/// assert_eq!(f_pred(vec![2]), 1);
///```
pub fn pred() -> FnPrimRec {
    prim_rec(zero(), proj(1))
}

/// ```
/// use prim_rec::extensions::basic::one;
///
/// let f_one = one();
///
/// assert_eq!(f_one(vec![]), 1);
/// ```
pub fn one() -> FnPrimRec {
    comp(succ(), vec![zero()])
}
