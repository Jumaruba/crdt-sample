# Sample CRDTs

A repository containing some CRDTs implementations in Rust. 

| CRDT | Type | File | Tests |  
| --- | --- | --- | --- | 
| Aworset | State-Based | [Here](./src/aworset.rs) | [Here](./tests/aworset.rs) |
| GCounter | State-Based | [Here](./src/gcounter.rs) | -- | 

# References
- [1] N. Preguiça, C. Baquero, and M. Shapiro, “Conflict-free Replicated Data Types (CRDTs),” arXiv:1805.06358 [cs], May 2018, doi: 10.1007/978-3-319-63962-8\_185-1.
- [2] P. S. Almeida, A. Shoker, and C. Baquero, “Delta state replicated data types,” Journal of Parallel and Distributed Computing, vol. 111, pp. 162–173, Jan. 2018, doi: 10.1016/j.jpdc.2017.08.003.
- [3] P. S. Almeida, A. Shoker, and C. Baquero, “Efficient state-based CRDTs by delta-mutation,” in Networked systems, Cham, 2015, pp. 62–76.
- [4] P. S. Almeida, A. Shoker, and C. Baquero, “Delta state replicated data types,” Journal of Parallel and Distributed Computing, vol. 111, pp. 162–173, Jan. 2018, doi: 10.1016/j.jpdc.2017.08.003.

