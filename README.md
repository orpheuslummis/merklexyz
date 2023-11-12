# (merkle xyz)

<img src="merklexyz_logo.png" alt="Merkle XYZ Logo" height="200px"/>

Merkle proof verification in Lurk, for Aptos Light Client.

Made during ZK Hack Istanbul.

## Problem it solves
Developing an efficient Merkle proof verification in Lurk for Aptos light clients contributes to a more robust and scalable blockchain ecosystem.

It enables secure verification of blockchain data without accessing the entire blockchain, crucial for maintaining trust in decentralized systems. It allows light clients to verify transactions efficiently. It can reduce network overhead, supporting scalability.


## Challenges during implementation
We encountered multiple struggles with the tooling. Therefore, we unfortunately spent most of our time was fighting with the tooling, and not with the core of the problem.

In particular:
- Lack of documentation of Lurk.
- Using Lurk as an imported crate was problematic. Last tagged version is very old. Using latest source also didn’t work.
- We tried using `Store` to pass data without changing the Lurk expression, but struggled with the lack of documentation, related tests, and failing data conversions.
- Similarly, we ran into with extracting data from the output of running Lurk proofs because of the lack of examples that do that.
- We had trouble debugging the Lurk code when writing the Merkle proof.


## How to run

```
cd merklexyz && cargo test
```

## Authors and licensing
- Orpheus (https://github.com/orpheuslummis)
- Varun (https://github.com/vhawk19)

Our added commmits are licensed MIT.