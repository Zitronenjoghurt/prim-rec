use crate::extensions::basic::pred;
use crate::*;

/// ```
/// use prim_rec::extensions::arithmetic::add;
///
/// let f_add = add();
///
/// assert_eq!(f_add(vec![3, 4]), 7);
/// assert_eq!(f_add(vec![12, 2]), 14);
/// assert_eq!(f_add(vec![0, 0]), 0);
/// ```
pub fn add() -> FnPrimRec {
    prim_rec(proj(0), comp(succ(), vec![proj(0)]))
}

/// Reversed subtraction, sub_rev(t,x) = x - t
///
/// ```
/// use prim_rec::extensions::arithmetic::sub_rev;
///
/// let f_sub = sub_rev();
///
/// assert_eq!(f_sub(vec![2, 3]), 1);
/// assert_eq!(f_sub(vec![3, 2]), 0);
/// assert_eq!(f_sub(vec![12, 54]), 42);
/// ```
pub fn sub_rev() -> FnPrimRec {
    prim_rec(proj(0), comp(pred(), vec![proj(0)]))
}
