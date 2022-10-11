use crate::computation::Computation;
use crate::jet;
use crate::jet::application::Turing;
use simplicity::core::Context;
use simplicity::CommitNode;
use std::rc::Rc;

/// Computation _(of arbitrary length)_ with 256-bit tapes and 256-bit one-hot indices.
///
/// Tapes are encoded as `l = 256`-bit strings.
/// Indices are encoded as `k = 256`-bit strings (one-hot encoding).
pub struct Computation256 {}

impl Computation for Computation256 {
    /// `get: 2^256 × 2^256 → 2`
    fn get(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let w_and_i = CommitNode::jet(context, &jet::turing::AND_256).unwrap();
        let is_zero256 = CommitNode::jet(context, &jet::turing::IS_ZERO256).unwrap();
        let w_and_i_is_zero = CommitNode::comp(context, w_and_i, is_zero256).unwrap();

        CommitNode::not(context, w_and_i_is_zero).unwrap()
    }

    /// `set: 2 × (2^256 × 2^256) → 2^256`
    fn set(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let iden_256 = CommitNode::iden(context).unwrap();
        let w = CommitNode::take(context, iden_256.clone()).unwrap();
        let i = CommitNode::drop(context, iden_256).unwrap();

        let pair_w_i = CommitNode::pair(context, w.clone(), i.clone()).unwrap();
        let or_256 = CommitNode::jet(context, &jet::turing::OR_256).unwrap();
        let w_or_i = CommitNode::comp(context, pair_w_i, or_256).unwrap();

        let complement256 = CommitNode::jet(context, &jet::turing::COMPLEMENT_256).unwrap();
        let complement_i = CommitNode::comp(context, i, complement256).unwrap();
        let pair_w_complement_i = CommitNode::pair(context, w, complement_i).unwrap();
        let and_256 = CommitNode::jet(context, &jet::turing::AND_256).unwrap();
        let w_and_complement_i = CommitNode::comp(context, pair_w_complement_i, and_256).unwrap();

        CommitNode::cond(context, w_or_i, w_and_complement_i).unwrap()
    }

    /// `eq_tape: 2^256 × 2^256 → 2`
    fn eq_tape(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        CommitNode::jet(context, &jet::turing::EQ256).unwrap()
    }

    /// `eq_tape: 2^256 × 2^256 → 2`
    fn eq_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        CommitNode::jet(context, &jet::turing::EQ256).unwrap()
    }

    /// `inc_index: 2^256 → 2^256`
    fn inc_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        CommitNode::jet(context, &jet::turing::RIGHT_SHIFT_256).unwrap()
    }

    /// `dec_index: 2^256 → 2^256`
    fn dec_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        CommitNode::jet(context, &jet::turing::LEFT_SHIFT_256).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Computation256;
    use crate::computation::Computation;
    use simplicity::core::{Context, Value};
    use simplicity::exec::BitMachine;

    #[test]
    fn get() {
        let mut context = Context::default();
        let program = Computation256::get(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let mut mac = BitMachine::for_program(&program);
        let w = Value::u256_from_slice(&[
            0b1000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]);
        let i = Value::u256_from_slice(&[
            0b1000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]);

        mac.input(&Value::prod(w, i));
        let output = mac.exec(&program, &()).unwrap();
        assert_eq!(Value::u1(1), output);
    }

    #[test]
    fn set() {
        let mut context = Context::default();
        let program = Computation256::set(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let base = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let symbol_input_output = vec![
            (0, 0b00000000, 0b00000000),
            (0, 0b10000000, 0b00000000),
            (1, 0b00000000, 0b10000000),
            (1, 0b10000000, 0b10000000),
        ];
        let index = 0b10000000;

        for (symbol, input_byte, output_byte) in symbol_input_output {
            let mut input_tape = base.clone();
            input_tape[0] = input_byte;
            let mut unary_index = base.clone();
            unary_index[0] = index;
            let mut output_tape = base.clone();
            output_tape[0] = output_byte;

            let b = Value::u1(symbol);
            let w = Value::u256_from_slice(&input_tape);
            let i = Value::u256_from_slice(&unary_index);
            let expected = Value::u256_from_slice(&output_tape);

            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(b, Value::prod(w, i)));
            let output = mac.exec(&program, &()).unwrap();

            assert_eq!(expected, output);
        }
    }
}
