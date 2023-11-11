use lurk::{
    eval::lang::{Coproc, Lang},
    field::LurkField,
    lem::{pointers::Ptr, store::Store},
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
         (plus (lambda (a b) (+ a b)))
         (sum (lambda (list) (reduce 0 plus list))))
  (sum '({array_str})))
"#,
        array_str = array_str
    );
    store.read_with_default_state(&program).unwrap()
}

fn lurk_sum(store: &Store<Fq>, array: &[usize]) -> Ptr<Fq> {
    let sum_expr = sum_expr(store, array);
    let mut lang = Lang::<Fq, Coproc<Fq>>::new();
    let limit = 1000;
    let (output, ..) = evaluate(None, sum_expr, store, limit).unwrap();
    let target_env = &output[1];
    let (first_binding, _) = store.car_cdr(target_env).unwrap();
    store.car_cdr(&first_binding).unwrap().1
}

fn sum_array(store: &Store<Fq>, array: &[usize]) -> Ptr<Fq> {
    lurk_sum(store, array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ok() {
        let store = &Store::<Fq>::default();
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sum = sum_array(store, &array).expect("Failed to calculate sum");
        assert_eq!(sum.fmt_to_string(store, &state), "55");
    }
}
