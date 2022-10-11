use crate::jet::application::Turing;
use crate::machine::Machine;
use crate::util;
use simplicity::core::Context;
use simplicity::merkle::cmr::Cmr;
use simplicity::CommitNode;
use std::rc::Rc;

/// Turing machine with two states that writes the most symbols upon the empty input.
/// It realizes the behaviour of `BusyBeaver(2)`.
///
/// States are encoded as `m = 1`-bit strings.
pub struct TwoBeavers {}

impl Machine for TwoBeavers {
    /// `left: 2 × 2 → 2`
    fn left(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // A → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // B → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // C → D
        let fail = CommitNode::fail(context, Cmr::from([0; 32]), Cmr::from([0; 32])).unwrap();
        // 2 → 2
        let iden2 = CommitNode::iden(context).unwrap();
        // 2 → 2
        let unit2 = CommitNode::unit(context).unwrap();
        // 2 → 2 × 1
        let pair_iden2_unit2 = CommitNode::pair(context, iden2, unit2).unwrap();
        // 2 × 1 → 2
        let case_right_left = CommitNode::case(context, bit_false, bit_true.clone()).unwrap();
        // 2 → 2
        let adapted_case_right_left =
            CommitNode::comp(context, pair_iden2_unit2.clone(), case_right_left).unwrap();
        // 2 × 1 → 2
        let case_left_fail = CommitNode::case(context, bit_true, fail).unwrap();
        // 2 → 2
        let adapted_case_left_fail =
            CommitNode::comp(context, pair_iden2_unit2, case_left_fail).unwrap();
        // 2 × 2 → 2
        CommitNode::cond(context, adapted_case_left_fail, adapted_case_right_left).unwrap()
    }

    /// `state: 2 × 2 → 2`
    fn state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // A → 2
        let bit_false = CommitNode::bit_false(context).unwrap();
        // B → 2
        let bit_true = CommitNode::bit_true(context).unwrap();
        // 2 × C → 2
        CommitNode::case(context, bit_true, bit_false).unwrap()
    }

    /// `write: 2 × 2 → 2`
    fn write(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        // A → 2
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

        let q_b_left = vec![
            (Value::u1(0), Value::u1(0), Value::u1(0)),
            (Value::u1(0), Value::u1(1), Value::u1(1)),
            (Value::u1(1), Value::u1(0), Value::u1(1)),
        ];

        for (q, b, left) in q_b_left {
            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(q, b));
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

        let q_b_state = vec![
            (Value::u1(0), Value::Unit, Value::u1(1)),
            (Value::u1(1), Value::Unit, Value::u1(0)),
        ];

        for (q, b, state) in q_b_state {
            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(q, b));
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
