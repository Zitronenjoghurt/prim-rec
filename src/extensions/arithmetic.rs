use crate::extensions::basic::{one, pred};
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
    prim_rec(proj(0), succ())
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

/// ```
/// use prim_rec::extensions::arithmetic::sub;
///
/// let f_sub = sub();
///
/// assert_eq!(f_sub(vec![5, 2]), 3);
/// assert_eq!(f_sub(vec![2, 3]), 0);
/// ```
pub fn sub() -> FnPrimRec {
    comp(sub_rev(), vec![proj(1), proj(0)])
}

/// ```
/// use prim_rec::extensions::arithmetic::mult;
///
/// let f_mult = mult();
///
/// assert_eq!(f_mult(vec![5, 3]), 15);
/// assert_eq!(f_mult(vec![3, 5]), 15);
/// assert_eq!(f_mult(vec![14, 20]), 280);
/// assert_eq!(f_mult(vec![20, 14]), 280);
/// assert_eq!(f_mult(vec![0, 5]), 0);
/// assert_eq!(f_mult(vec![5, 0]), 0);
/// ```
pub fn mult() -> FnPrimRec {
    prim_rec(zero(), comp(add(), vec![proj(0), proj(2)]))
}

/// ```
/// use prim_rec::extensions::arithmetic::exp_rev;
///
/// let f_exp = exp_rev();
///
/// assert_eq!(f_exp(vec![2, 2]), 4);
/// assert_eq!(f_exp(vec![5, 2]), 32);
/// ```
pub fn exp_rev() -> FnPrimRec {
    prim_rec(one(), comp(mult(), vec![proj(0), proj(2)]))
}

/// ```
/// use prim_rec::extensions::arithmetic::exp;
///
/// let f_exp = exp();
///
/// assert_eq!(f_exp(vec![2, 5]), 32);
/// ```
pub fn exp() -> FnPrimRec {
    comp(exp_rev(), vec![proj(1), proj(0)])
}
