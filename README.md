# Rust Raw Pointer Modification After Move

This repository demonstrates a common error in Rust related to using raw pointers and modifying the memory they point to after the original data structure has been moved or dropped.

The `bug.rs` file contains code that exhibits this error. The `bugSolution.rs` file presents a corrected version, explaining how to avoid this issue.