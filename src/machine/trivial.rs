use crate::jet::application::Turing;
use crate::machine::Machine;
use crate::util;
use simplicity::core::Context;
use simplicity::CommitNode;
use std::rc::Rc;

/// Trivial Turing machine with two states.
///
/// The machine transitions to the second state upon reading `1` and stays there forever.
/// The cursor is shifted right in each step. The tape is left unchanged.
///
/// States are encoded as `m = 1`-bit strings.
pub struct Trivial {}

impl Machine for Trivial {
    /// `left: 2 × 2 → 2`
    fn left(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // A → 2
        CommitNode::bit_false(context).unwrap()
    }

    /// `state: 2 × 2 → 2`
    fn state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 2 → 2
        CommitNode::cond(context, bit_true, iden_2).unwrap()
    }

    /// `write: 2 × 2 → 2`
    fn write(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 × 2 → 2
        CommitNode::take(context, iden_2).unwrap()
    }

    /// `initial: 2 → 2`
    fn initial(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        CommitNode::not(context, iden_2).unwrap()
    }

    /// `accepting: 2 → 2`
    fn accepting(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        CommitNode::iden(context).unwrap()
    }

    /// `eq_state: 2 × 2 → 2`
    fn eq_state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 × 2 → 2
        util::eq_2(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::core::Value;
    use simplicity::exec::BitMachine;

    #[test]
    fn left() {
        let mut context = Context::default();
        let program = Trivial::left(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::Unit);
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::u1(0), output);
    }

    #[test]
    fn state() {
        let mut context = Context::default();
        let program = Trivial::state(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let b_q_state = vec![
            (Value::u1(0), Value::u1(0), Value::u1(0)),
            (Value::u1(0), Value::u1(1), Value::u1(1)),
            (Value::u1(1), Value::u1(0), Value::u1(1)),
            (Value::u1(1), Value::u1(1), Value::u1(1)),
        ];

        for (b, q, state) in b_q_state {
            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(b, q));
            let output = mac.exec(&program, &()).unwrap();
            assert_eq!(state, output);
        }
    }

    #[test]
    fn write() {
        let mut context = Context::default();
        let program = Trivial::write(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(Value::Unit, Value::Unit));
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::Unit, output);
    }
}
