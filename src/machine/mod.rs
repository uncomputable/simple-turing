mod trivial;
mod two_beavers;

use crate::jet::application::Turing;
use simplicity::core::Context;
use simplicity::CommitNode;
use std::rc::Rc;
pub use trivial::Trivial;
pub use two_beavers::TwoBeavers;

/// Definition of a Turing Machine.
/// This includes the transition function
/// _(where to move the cursor, which symbol to write and which state to go to, upon which input)_,
/// as well as the initial and accepting state.
///
/// States are encoded as `m`-bit strings.
pub trait Machine {
    /// Go left, given the current state and read tape symbol?
    ///
    /// `left: 2^m × 2 → 2`
    fn left(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Successor state, given the current state and read tape symbol.
    ///
    /// `state: 2^m × 2 → 2^m`
    fn state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Tape symbol to write, given the current state and read tape symbol.
    ///
    /// `write: 2^m × 2 → 2`
    fn write(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Check if the given state is the initial state.
    ///
    /// `initial: 2^m → 2`
    fn initial(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Check if the given state is the accepting state.
    ///
    /// `accepting: 2^m → 2`
    fn accepting(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;

    /// Check equality of two states
    ///
    /// `eq_state: 2^m × 2^m → 2`
    fn eq_state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>>;
}
