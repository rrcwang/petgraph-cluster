//! Katex rendering test.
//!
//! ## Inline
//!
//! For a graph $G = \left(V, E\right)$, a *cut* $C = \left(S, \bar{S}\right)$ of $G$ is a partition of $V$ such that $S\subseteq V$ and $\bar{S} = V \setminus S$.
//!
//! ## Display
//!     
//! For a vector space of finite dimension $n$, a spanning basis $B = \lbrace\mathbf{v}_1, \dots, \mathbf{v}_n \rbrace$ has linearly independent members:
//!    $$ \mathbf{v}_i \perp \mathbf{v}_j \  \forall i,j \in \lbrace 1,\dots,n \vert i \neq j \rbrace $$

// Crate-wide Clippy lints
#![deny(missing_docs)]

pub mod error;
pub mod math;
#[allow(unused_imports, unused_variables, dead_code)] // TEMP lint ignores
pub mod partition;
