use lurk::state::State;
use lurk::{
    eval::lang::Coproc,
    field::LurkField,
    lem::{eval::evaluate_simple, pointers::Ptr, store::Store},
};
use pasta_curves::Fq;

fn merklelurk_expr<F: LurkField>(store: &Store<F>, array: &[usize]) -> Ptr<F> {
    let array_str = array
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let program = format!(
        r#"
        (letrec
            ((combine (lambda (hash1 hash2)
                        (commit (concat hash1 hash2))))
          
             (computeRoot (lambda (currentHash siblings)
                            (if (null? siblings)
                                currentHash
                                (computeRoot (combine currentHash (car siblings)) (cdr siblings)))))
          
             (verifyMerkleProof
               (lambda (leaf siblings root)
                 (let ((computedRoot (computeRoot (commit leaf) siblings)))
                   (equal? computedRoot root))))
          
             (L1 "Data1") (L2 "Data2") (L3 "Data3") (L4 "Data4")
             (A (combine (commit L1) (commit L2)))
             (B (combine (commit L3) (commit L4)))
             (root (combine A B)))
          
            (proofValidL1 (verifyMerkleProof L1 (list (commit L2) B) root))
          
            (proofValidL3 (verifyMerkleProof L3 (list (commit L4) A) root))
          
            (list proofValidL1 proofValidL3))
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
    fn ok() {}

    /*
        // use crate::proof::{lurk_definition::TestMerkleLurkProof, TestAccumulatorInternalNode};
        // use aptos_crypto::hash::{CryptoHash, TestOnlyHash, ACCUMULATOR_PLACEHOLDER_HASH};

        #[test]
        fn test_verify_empty_accumulator() {
            let element_hash = b"hello".test_only_hash();
            let root_hash = *ACCUMULATOR_PLACEHOLDER_HASH;
            let proof = TestMerkleLurkProof::new(vec![]);
            assert!(proof.verify(root_hash, element_hash, 0).is_err());
        }

        #[test]
        fn test_verify_single_element_accumulator() {
            let element_hash = b"hello".test_only_hash();
            let root_hash = element_hash;
            let proof = TestMerkleLurkProof::new(vec![]);
            assert!(proof.verify(root_hash, element_hash, 0).is_ok());
        }

        #[test]
        fn test_verify_two_element_accumulator() {
            let element0_hash = b"hello".test_only_hash();
            let element1_hash = b"world".test_only_hash();
            let root_hash = TestAccumulatorInternalNode::new(element0_hash, element1_hash).hash();

            assert!(TestMerkleLurkProof::new(vec![element1_hash])
                .verify(root_hash, element0_hash, 0)
                .is_ok());
            assert!(TestMerkleLurkProof::new(vec![element0_hash])
                .verify(root_hash, element1_hash, 1)
                .is_ok());
        }

        #[test]
        fn test_verify_three_element_accumulator() {
            let element0_hash = b"hello".test_only_hash();
            let element1_hash = b"world".test_only_hash();
            let element2_hash = b"!".test_only_hash();
            let internal0_hash = TestAccumulatorInternalNode::new(element0_hash, element1_hash).hash();
            let internal1_hash =
                TestAccumulatorInternalNode::new(element2_hash, *ACCUMULATOR_PLACEHOLDER_HASH).hash();
            let root_hash = TestAccumulatorInternalNode::new(internal0_hash, internal1_hash).hash();

            assert!(
                TestMerkleLurkProof::new(vec![element1_hash, internal1_hash])
                    .verify(root_hash, element0_hash, 0)
                    .is_ok()
            );
            assert!(
                TestMerkleLurkProof::new(vec![element0_hash, internal1_hash])
                    .verify(root_hash, element1_hash, 1)
                    .is_ok()
            );
            assert!(
                TestMerkleLurkProof::new(vec![*ACCUMULATOR_PLACEHOLDER_HASH, internal0_hash])
                    .verify(root_hash, element2_hash, 2)
                    .is_ok()
            );
        }
    }
    */
}
