/*
use anyhow::{bail, ensure, format_err, Context, Result};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// struct to represent merkle tree
// implementation with
// implement sum of two lists of integers

pub struct HashValue {
    value: [u8; 32],
}

impl HashValue {
    pub fn new(value: [u8; 32]) -> Self {
        Self { value }
    }
}

/// This is the LurkMerkleProof struct. It is used to authenticate an element in an accumulator given trusted root hash.
/// It contains a vector of siblings and a PhantomData type.
/// The siblings are all siblings in this proof, including the default ones. Siblings are ordered from the bottom level to the root level.
/// The PhantomData is a zero-sized type used to mark things that "act like" they own a T.
#[derive(Clone, Serialize, Deserialize)]
pub struct LurkMerkleProof<H> {
    siblings: Vec<HashValue>,
    phantom: PhantomData<H>,
}

impl<H: CryptoHasher> LurkMerkleProof<H> {
    pub fn new(siblings: Vec<HashValue>) -> Self {
        Self {
            siblings,
            phantom: PhantomData,
        }
    }

    pub fn siblings(&self) -> &[HashValue] {
        &self.siblings
    }

    pub fn verify(
        &self,
        expected_root_hash: HashValue,
        element_hash: HashValue,
        element_index: u64,
    ) -> Result<()> {
        // ensure!(
        // Here we would call the Lurk ZK engine to produce the verification
        Ok(())
    }
}

// TBD
impl<H> Eq for LurkMerkleProof<H> {}

// #[cfg(any(test, feature = "fuzzing"))]
// pub type TestLurkAccumulatorProof = AccumulatorProof<TestOnlyHasher>;

impl<H> std::fmt::Debug for LurkMerkleProof<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccumulatorProof {{ siblings: {:?} }}", self.siblings)
    }
}

// for the dummy test
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // --------------
    // Selected tests from the aptos proof system.

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
``
*/
