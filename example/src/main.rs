use prim_rec::*;

fn main() {
    let test = prim_rec(proj(0), proj(0));
    println!("{:?}", test(vec![1, 2, 3]));
}
