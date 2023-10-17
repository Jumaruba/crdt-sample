# Sample CRDTs

A repository containing some CRDTs implementations in Rust. 

| CRDT | Type | Tests | 
| --- | --- | --- | 
| **[Aworset]** [Add-Wins Observed Remove Set](./rust/src/aworset.rs)| State-Based | [aworset.rs](./rust/tests/aworset.rs) | 
| **[Aworset Optimized]** [Add-Wins Observed Remove Set](./rust/src/aworset_opt.rs) | State-Based | --- | 
| **[GCounter]** [Grow-Only Counter](./rust/src/gcounter.rs)| State-Based | -- |
| **[PnCounter]** [Positive-Negative Counter](./rust/src/pncounter.rs)| State-Based | -- |
| **[MvReg]** Multi-Value Register [on progress]| State-Based | --- | --- | 

## Auxiliary structures 

Some auxiliary structures were built to create some CRDTs: 

| Name | Tests | Explanation | Reference | 
| --- | --- | --- | --- | 
| [DotContext](./rust/src/dotcontext.rs) | [dotcontext.rs](./rust/tests/dotcontext.rs) | [Bartosz Sypytkowski Blog](https://www.bartoszsypytkowski.com/optimizing-state-based-crdts-part-2/) | [delta-enabled-crdts](https://github.com/CBaquero/delta-enabled-crdts/blob/master/delta-crdts.cc) | 

# Usage
The cargo package is available at: 
- https://crates.io/crates/crdt-sample

Add the following piece of code to your `Cargo.toml`:

```toml
[dependencies]
crdt-sample = "0.1.0"
```
## Example
```rust
use crdt_sample::{AworsetOpt, NodeId};
fn main() {
    let node_id = NodeId::new(1, "addr".to_string());
    let mut  aworset: AworsetOpt<i32> = AworsetOpt::new(node_id);
    aworset.add(1);
    println!("{:?}", aworset);
    // AworsetOpt { id: addr1, set: {(addr1, 1, 1)}, cc: DotContext { cc: {addr1: 1}, dc: {} } }
}

```
# References
- [1] N. Preguiça, C. Baquero, and M. Shapiro, “Conflict-free Replicated Data Types (CRDTs),” arXiv:1805.06358 [cs], May 2018, doi: 10.1007/978-3-319-63962-8\_185-1.
- [2] P. S. Almeida, A. Shoker, and C. Baquero, “Delta state replicated data types,” Journal of Parallel and Distributed Computing, vol. 111, pp. 162–173, Jan. 2018, doi: 10.1016/j.jpdc.2017.08.003.
- [3] P. S. Almeida, A. Shoker, and C. Baquero, “Efficient state-based CRDTs by delta-mutation,” in Networked systems, Cham, 2015, pp. 62–76.
- [4] P. S. Almeida, A. Shoker, and C. Baquero, “Delta state replicated data types,” Journal of Parallel and Distributed Computing, vol. 111, pp. 162–173, Jan. 2018, doi: 10.1016/j.jpdc.2017.08.003.

