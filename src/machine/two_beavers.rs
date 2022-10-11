use crate::jet::application::Turing;
use crate::machine::Machine;
use crate::util;
use simplicity::core::Context;
use simplicity::merkle::cmr::Cmr;
use simplicity::CommitNode;
use std::rc::Rc;

/// Turing machine with two states that writes the most symbols upon the empty input.
///
/// It realizes the behaviour of `BusyBeaver(2)`.
/// See [Wikipedia](https://en.wikipedia.org/w/index.php?title=Busy_beaver&oldid=1112201946).
///
/// States are encoded as `m = 1`-bit strings.
pub struct TwoBeavers {}

impl Machine for TwoBeavers {
    /// `left: 2 × 2 → 2`
    fn left(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 1 × 1 → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // 1 × 1 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 1 → 2
        let case_false_true = CommitNode::case(context, bit_false, bit_true).unwrap();
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        let unit_2 = CommitNode::unit(context).unwrap();
        // 2 → 2 × 1
        let adaptor = CommitNode::pair(context, iden_2, unit_2).unwrap();
        // 2 → 2
        let adapted_case_false_true = CommitNode::comp(context, adaptor, case_false_true).unwrap();
        // 2 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 2 → 2
        CommitNode::cond(context, bit_true, adapted_case_false_true).unwrap()
    }

    /// `state: 2 × 2 → 2`
    fn state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 1 × 1 → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // 1 × 1 → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 1 → 2
        let case_true_false = CommitNode::case(context, bit_true, bit_false).unwrap();
        // 2 → 2
        let iden_2 = CommitNode::iden(context).unwrap();
        // 2 × 2 → 2
        let drop_2_2 = CommitNode::drop(context, iden_2).unwrap();
        // 2 × 2 → 1
        let unit_2 = CommitNode::unit(context).unwrap();
        // 2 × 2 → 2 × 1
        let adaptor = CommitNode::pair(context, drop_2_2, unit_2).unwrap();
        // 2 × 2 → 2
        CommitNode::comp(context, adaptor, case_true_false).unwrap()
    }

    /// `write: 2 × 2 → 2`
    fn write(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 × 2 → 2
        CommitNode::bit_true(context).unwrap()
    }

    /// `initial: 2 → 2`
    fn initial(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden2 = CommitNode::iden(context).unwrap();
        // 2 → 1
        let unit2 = CommitNode::unit(context).unwrap();
        // 2 → 2 × 1
        let pair_iden2_unit2 = CommitNode::pair(context, iden2, unit2).unwrap();
        // A → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // B → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 1 → 2
        let case_true_false = CommitNode::case(context, bit_true, bit_false).unwrap();
        // 2 → 2
        CommitNode::comp(context, pair_iden2_unit2, case_true_false).unwrap()
    }

    /// `accepting: 2 → 2`
    fn accepting(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 → 2
        let iden2 = CommitNode::iden(context).unwrap();
        // 2 → 1
        let unit2 = CommitNode::unit(context).unwrap();
        // 2 → 2 × 1
        let pair_iden2_unit2 = CommitNode::pair(context, iden2, unit2).unwrap();
        // A → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // B → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × 1 → 2
        let case_false_true = CommitNode::case(context, bit_false, bit_true).unwrap();
        // 2 → 2
        CommitNode::comp(context, pair_iden2_unit2, case_false_true).unwrap()
    }

    /// `eq_state_verify: 2 × 2 → 2`
    fn eq_state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // 2 × 2 → 2
        util::eq_2(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::computation::Computation256;
    use crate::verifier::Verifier;
    use simplicity::core::Value;
    use simplicity::exec::BitMachine;

    #[test]
    fn left() {
        let mut context = Context::default();
        let program = TwoBeavers::left(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let b_q_left = vec![
            (Value::u1(0), Value::u1(0), Value::u1(0)),
            (Value::u1(0), Value::u1(1), Value::u1(1)),
            (Value::u1(1), Value::u1(0), Value::u1(1)),
        ];

        for (b, q, left) in b_q_left {
            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(b, q));
            let output = mac.exec(&program, &()).unwrap();
            assert_eq!(left, output);
        }
    }

    #[test]
    fn state() {
        let mut context = Context::default();
        let program = TwoBeavers::state(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let b_q_state = vec![
            (Value::Unit, Value::u1(0), Value::u1(1)),
            (Value::Unit, Value::u1(1), Value::u1(0)),
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
        let program = TwoBeavers::write(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        mac.input(&Value::prod(Value::Unit, Value::Unit));
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::u1(1), output);
    }
}
