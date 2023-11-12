use lurk::state::State;
use lurk::{
    eval::lang::Coproc,
    field::LurkField,
    lem::{eval::evaluate_simple, pointers::Ptr, store::Store},
};
use pasta_curves::Fq;

fn sum_expr<F: LurkField>(store: &Store<F>, array: &[usize]) -> Ptr<F> {
    let array_str = array
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let program = format!(
        r#"
(letrec ((reduce (lambda (acc f list)
                   (if (eq list nil)
                       acc
                       (reduce (f acc (car list))
                               f
                               (cdr list)))))
         (sum (reduce 0 (lambda (a b) (+ a b)))))
  (sum '({array_str})))
"#,
        array_str = array_str
    );
    println!("program: {}", program);
    store.read_with_default_state(&program).unwrap()
}

fn lurk_sum(store: &Store<Fq>, array: &[usize]) -> Ptr<Fq> {
    println!("Starting lurk_sum function");
    let sum_expr = sum_expr(store, array);
    println!("sum_expr: {:?}", sum_expr);
    let limit = 10000; // TBD
    println!("limit: {}", limit);
    let (output, ..) = evaluate_simple::<Fq, Coproc<Fq>>(None, sum_expr, store, limit).unwrap();
    println!("output: {:?}", output);
    let sum = output[0];
    println!("sum: {:?}", sum);
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn program_prints() {
        let store = &Store::<Fq>::default();
        let state = State::init_lurk_state();
        let array = [1, 2, 3, 4, 5];
        let sum = lurk_sum(store, &array);
        assert_eq!(sum.fmt_to_string(store, &state), "15");
    }

    // #[test]
    // fn basic_ok() {
    //     let store = &Store::<Fq>::default();
    //     let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //     let sum = lurk_sum(store, &array).expect("Failed to calculate sum");
    //     assert_eq!(sum.fmt_to_string(store, &state), "55");
    // }
}
