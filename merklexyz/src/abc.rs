use lurk::{
    eval::lang::Coproc,
    field::LurkField,
    lem::{eval::evaluate_simple_with_env, pointers::Ptr, store::Store},
};
use pasta_curves::Fq;

// fn sum_expr<F: LurkField>(store: &Store<F>) -> Ptr<F> {
//     let program = r#"
// (letrec ((plus (lambda (a b c) (+ a b c)))
//          (sum (lambda (list) (plus (car list) (cadr list) (caddr list)))))
//   sum)
// "#;
//     println!("program: {}", program);
//     store.read_with_default_state(&program).unwrap()
// }

fn sum_expr<F: LurkField>(store: &Store<F>) -> Ptr<F> {
    let program = r#"
(letrec ((reduce (lambda (acc f list)
                   (if (eq list nil)
                       acc
                       (reduce (f acc (car list))
                               f
                               (cdr list)))))
         (plus (lambda (a b) (+ a b)))
         (sum (lambda (list) (reduce 0 plus list))))
  sum)
"#;
    println!("program: {}", program);
    store.read_with_default_state(&program).unwrap()
}

fn lurk_sum(store: &Store<Fq>, params: &[u64]) -> Ptr<Fq> {
    let sum_expr = sum_expr(store);
    let limit = 1000; // TBD
                      // let a = Ptr::<Fq>::num(params[0].into());
                      // let b = Ptr::<Fq>::num(params[1].into());
                      // let c = Ptr::<Fq>::num(params[2].into());
    let a = Ptr::<Fq>::num(Fq::from(params[0]));
    let b = Ptr::<Fq>::num(Fq::from(params[1]));
    let c = Ptr::<Fq>::num(Fq::from(params[2]));
    let env_list = store.list(vec![a, b, c]);
    let (output, ..) =
        evaluate_simple_with_env::<Fq, Coproc<Fq>>(None, sum_expr, env_list, store, limit).unwrap();
    let target_env = &output[1];
    let (first_binding, _) = store.car_cdr(target_env).unwrap();
    store.car_cdr(&first_binding).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_prints() {
        let store = &Store::<Fq>::default();
        lurk_sum(store, &[1, 2, 3]);
    }

    // #[test]
    // fn basic_ok() {
    //     let store = &Store::<Fq>::default();
    //     let array = [1, 2, 3];
    //     let sum = lurk_sum(store, &array).expect("Failed to calculate sum");
    //     assert_eq!(sum.fmt_to_string(store, &state), "6");
    // }
}
