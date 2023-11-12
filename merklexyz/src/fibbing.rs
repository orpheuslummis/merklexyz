use lurk::state::State;
use lurk::{
    eval::lang::Coproc,
    field::LurkField,
    lem::{eval::evaluate_simple, pointers::Ptr, store::Store},
};
use pasta_curves::Fq;

// 11 is the base offset for initial setup frames
// 16 is the number of frames for each Fibonacci computation
fn fib_frame(n: usize) -> usize {
    11 + 16 * n
}

fn fib_expr<F: LurkField>(store: &Store<F>) -> Ptr<F> {
    let program = r#"
(letrec ((next (lambda (a b) (next b (+ a b))))
           (fib (next 0 1)))
  (fib))
"#;
    store.read_with_default_state(program).unwrap()
}

fn lurk_fib(store: &Store<Fq>, n: usize, _rc: usize) -> Ptr<Fq> {
    println!("Starting lurk_fib function");
    let frame_idx = fib_frame(n);
    println!("frame_idx: {}", frame_idx);
    // let limit = fib_limit(n, rc);
    let limit = frame_idx;
    println!("limit: {}", limit);
    let fib_expr = fib_expr(store);
    println!("fib_expr: {:?}", fib_expr);

    let (output, ..) = evaluate_simple::<Fq, Coproc<Fq>>(None, fib_expr, store, limit).unwrap();
    println!("output: {:?}", output);
    let target_env = &output[1];
    println!("target_env: {:?}", target_env);

    let (_, rest_bindings) = store.car_cdr(target_env).unwrap();
    println!("rest_bindings: {:?}", rest_bindings);
    let (second_binding, _) = store.car_cdr(&rest_bindings).unwrap();
    println!("second_binding: {:?}", second_binding);
    let fib = store.car_cdr(&second_binding).unwrap().1;
    println!("fib: {:?}", fib);
    fib
}

fn lurk_fib_with_frames(store: &Store<Fq>, n: usize, _rc: usize) -> (Ptr<Fq>, usize) {
    let frame_idx = fib_frame(n);
    let limit = frame_idx;
    let fib_expr = fib_expr(store);

    let (output, frames, _) =
        evaluate_simple::<Fq, Coproc<Fq>>(None, fib_expr, store, limit).unwrap();

    let target_env = &output[1];
    let (_, rest_bindings) = store.car_cdr(target_env).unwrap();
    let (second_binding, _) = store.car_cdr(&rest_bindings).unwrap();
    let fib = store.car_cdr(&second_binding).unwrap().1;

    (fib, frames)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ok() {
        let store = &Store::<Fq>::default();
        let state = State::init_lurk_state();
        let n = 10;
        let fib = lurk_fib(store, n, 100);
        println!("{}", fib.fmt_to_string(store, &state));
        assert_eq!(fib.fmt_to_string(store, &state), "55");
    }

    #[test]
    fn frame_count_confirmationf() {
        let store = &Store::<Fq>::default();
        let state = State::init_lurk_state();
        for n in 1..=10 {
            let (fib, frames) = lurk_fib_with_frames(store, n, 100);
            assert_eq!(
                frames,
                11 + 16 * n,
                "Frame count does not match the expected pattern for n = {}",
                n
            );
            println!(
                "For n = {}, fib = {}, frames = {}",
                n,
                fib.fmt_to_string(store, &state),
                frames
            );
        }
    }

    #[test]
    fn more_fib() {
        let store = &Store::<Fq>::default();
        let state = State::init_lurk_state();
        let n = 50;
        let fib = lurk_fib(store, n, 100);
        assert_eq!(fib.fmt_to_string(store, &state), "12586269025");
    }
}
