use prim_rec::extensions::logic::if_then_else;

fn main() {
    let test = if_then_else();
    println!("{:?}", test(vec![1, 2, 3]));
}
