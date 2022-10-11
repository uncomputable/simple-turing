mod default;

use crate::jet::application::Turing;
pub use default::Computation256;
use simplicity::core::Context;
use simplicity::CommitNode;
use std::rc::Rc;

/// Computation of a Turing Machine, i.e., sequence of configurations.
/// A configuration is the current state, current tape and index which points to the current symbol.
/// Tapes are _conceptually_ infinite to the RHS,
/// but are _practically_ limited by the maximum tape length during any step of the concrete computation.
///
/// Tapes are encoded as `l`-bit strings.
/// The tape is indexed via `k`-bit strings.
pub trait Computation {
    /// Get tape symbol at index
    ///
    /// `get: 2^l × 2^k → 2`
    fn get(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Set tape symbol at index to given symbol
    ///
    /// `set: 2 × (2^l × 2^k) → 2^l`
    fn set(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Check equality of two tapes
    ///
    /// `eq_tape: 2^l × 2^l → 2`
    fn eq_tape(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Check equality of two indices
    ///
    /// `eq_index: 2^k × 2^k → 2`
    fn eq_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Increment index by one (modulo 2^k)
    ///
    /// `inc_index: 2^k → 2^k`
    fn inc_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Decrement index by one (modulo 2^k)
    ///
    /// `inc_index: 2^k → 2^k`
    fn dec_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;
}
