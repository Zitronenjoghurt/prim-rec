pub mod extensions;

pub type Nat = u64;
pub type FnPrimRec = Box<dyn Fn(Vec<Nat>) -> Nat>;

pub fn zero() -> FnPrimRec {
    Box::new(|_args: Vec<Nat>| -> Nat { 0 })
}

pub fn succ() -> FnPrimRec {
    Box::new(|args: Vec<Nat>| -> Nat {
        assert!(
            !args.is_empty(),
            "Successor function requires at least one argument."
        );
        args[0].saturating_add(1)
    })
}

pub fn proj(i: usize) -> FnPrimRec {
    Box::new(move |args: Vec<Nat>| -> Nat {
        assert!(args.len() > i, "Projection index out of bounds.");
        args[i]
    })
}

/// comp(f, g0, g1, ...) builds a function h(x)
/// h(x) = f(g0(x), g1(x), ...)
pub fn comp(outer_fn: FnPrimRec, inner_fns: Vec<FnPrimRec>) -> FnPrimRec {
    Box::new(move |args: Vec<Nat>| {
        let mut inner_results = Vec::new();

        for inner_fn in &inner_fns {
            let result = inner_fn(args.clone());
            inner_results.push(result);
        }
        outer_fn(inner_results)
    })
}

pub fn prim_rec(base_case: FnPrimRec, inductive_step: FnPrimRec) -> FnPrimRec {
    Box::new(move |args: Vec<Nat>| {
        assert!(
            !args.is_empty(),
            "Primitive recursion requires at least one argument"
        );

        let t = args[0];
        let rest = args[1..].to_vec();

        if t == 0 {
            return base_case(rest);
        }

        let mut result = base_case(rest.clone());

        for i in 1..=t {
            let mut rec_args = Vec::new();
            rec_args.push(result);
            rec_args.push(i - 1);
            rec_args.extend_from_slice(&rest);

            result = inductive_step(rec_args);
        }

        result
    })
}
