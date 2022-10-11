use crate::jet::application::Turing;
use crate::machine::Machine;
use crate::util;
use simplicity::core::{Context, Value};
use simplicity::CommitNode;
use std::rc::Rc;

/// Turing machine with three states that writes the most symbols upon the empty input.
///
/// It realizes the behaviour of `BusyBeaver(3)`.
/// See [Wikipedia](https://en.wikipedia.org/w/index.php?title=Busy_beaver&oldid=1112201946).
///
/// States are encoded as `m = 2`-bit strings.
pub struct ThreeBeavers {}

impl Machine for ThreeBeavers {
    /// `left: 2 × 2^2 → 2`
    fn left(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 × 2 → 2
        let take_2_2 = CommitNode::take(context, iden_2).unwrap();
        // 2 × 2^2 → 2
        CommitNode::drop(context, take_2_2).unwrap()
    }

    /// `state: 2 × 2^2 → 2`
    fn state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let zero = Value::u2(0);
        let one = Value::u2(1);
        let two = Value::u2(2);

        // 1 × 2 → 2^2
        let scribe_zero = CommitNode::scribe(context, &zero).unwrap();
        // 1 × 2 → 2^2
        let scribe_one = CommitNode::scribe(context, &one).unwrap();
        // 2 × 2 → 2^2
        let case_one_zero = CommitNode::case(context, scribe_one, scribe_zero).unwrap();

        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 1
        let unit_2 = CommitNode::unit(context).unwrap();
        // 2 → 2 × 1
        let adaptor = CommitNode::pair(context, iden_2, unit_2).unwrap();

        // 1 × 1 → 2^2
        let scribe_one = CommitNode::scribe(context, &one).unwrap();
        // 1 × 1 → 2^2
        let scribe_two = CommitNode::scribe(context, &two).unwrap();
        // 2 × 1 → 2^2
        let case_one_two = CommitNode::case(context, scribe_one, scribe_two).unwrap();
        // 2 → 2^2
        let adapted_case_one_two = CommitNode::comp(context, adaptor, case_one_two).unwrap();

        // 2 → 2^2
        let scribe_two = CommitNode::scribe(context, &two).unwrap();
        // 2 × 2 → 2^2
        let cond_two_case_one_two =
            CommitNode::cond(context, scribe_two, adapted_case_one_two).unwrap();

        // 2 × 2^2 → 2^2
        CommitNode::cond(context, case_one_zero, cond_two_case_one_two).unwrap()
    }

    /// `write: 2 × 2^2 → 2`
    fn write(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        let not_2 = CommitNode::not(context, iden_2).unwrap();
        // 2 × 2 → 2
        let cond_true_not = CommitNode::cond(context, bit_true, not_2).unwrap();

        // 2^2 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 2^2 → 2
        CommitNode::cond(context, bit_true, cond_true_not).unwrap()
    }

    /// `initial: 2^2 → 2`
    fn initial(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 1 × 1 → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        let not_2 = CommitNode::not(context, iden_2).unwrap();
        // 2 × 2 → 2
        CommitNode::cond(context, bit_false, not_2).unwrap()
    }

    /// `accepting: 2^2 → 2`
    fn accepting(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2^2 → 2
        Self::initial(context)
    }

    /// `eq_state: 2^2 × 2^2 → 2`
    fn eq_state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 × 2 → 2
        util::eq_22(context)
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
        let program = ThreeBeavers::left(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(Value::Unit, Value::Unit));
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::Unit, output);
    }

    #[test]
    fn state() {
        let mut context = Context::default();
        let program = ThreeBeavers::state(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let b_q_state = vec![
            (Value::u1(0), Value::u2(0), Value::u2(1)),
            (Value::u1(0), Value::u2(1), Value::u2(2)),
            (Value::u1(1), Value::u2(1), Value::u2(1)),
            (Value::u1(0), Value::u2(2), Value::u2(2)),
            (Value::u1(1), Value::u2(2), Value::u2(0)),
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
        let program = ThreeBeavers::write(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(Value::Unit, Value::Unit));
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::u1(1), output);
    }
}
