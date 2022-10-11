use crate::jet::application::Turing;
use simplicity::bitwriter::BitWriter;
use simplicity::core::{Context, Value};
use simplicity::CommitNode;
use std::rc::Rc;

/// Check equality of bits.
///
/// `eq_2: 2 × 2 → 2`
pub fn eq_2(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
    // 2 → 2
    let iden_2 = CommitNode::iden(context).unwrap();
    // 2 → 2
    let not_2 = CommitNode::not(context, iden_2.clone()).unwrap();
    // 2 × 2 → 2
    CommitNode::cond(context, iden_2, not_2).unwrap()
}

/// Check equality of two-bit strings.
///
/// `eq_2: 2^2 × 2^2 → 2`
pub fn eq_22(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
    // 2 → 2
    let iden_2 = CommitNode::iden(context).unwrap();
    // 2 × 2 → 2
    let take_2_2 = CommitNode::take(context, iden_2.clone()).unwrap();
    // 2 × 2 → 2
    let drop_2_2 = CommitNode::drop(context, iden_2).unwrap();
    // 2^2 × 2^2 → 2
    let first_first = CommitNode::take(context, take_2_2.clone()).unwrap();
    // 2^2 × 2^2 → 2
    let second_first = CommitNode::drop(context, take_2_2).unwrap();
    // 2^2 × 2^2 → 2
    let first_second = CommitNode::take(context, drop_2_2.clone()).unwrap();
    // 2^2 × 2^2 → 2
    let second_second = CommitNode::drop(context, drop_2_2).unwrap();

    // 2^2 × 2^2 → 2 × 2
    let first_bit = CommitNode::pair(context, first_first, second_first).unwrap();
    // 2 × 2 → 2
    let eq_2 = eq_2(context);
    // 2^2 × 2^2 → 2
    let first_bit_equal = CommitNode::comp(context, first_bit, eq_2.clone()).unwrap();
    // 2^2 × 2^2 → 2 × 2
    let second_bit = CommitNode::pair(context, first_second, second_second).unwrap();
    // 2^2 × 2^2 → 2 × 2^2
    let first_equal_and_second = CommitNode::pair(context, first_bit_equal, second_bit).unwrap();

    // 2^2 → 2
    let bit_false = CommitNode::bit_false(context).unwrap();
    // 2 × 2^2 → 2
    let cond_second_equal_or_false = CommitNode::cond(context, eq_2, bit_false).unwrap();

    // 2^2 × 2^2 → 2
    CommitNode::comp(context, first_equal_and_second, cond_second_equal_or_false).unwrap()
}

/// Create a computation from states, tapes and indices, as witness for a program commitment.
pub fn computation256_of<'a, S, T, I, F>(
    states: S,
    tapes: T,
    indices: I,
    state_to_value: F,
) -> impl Iterator<Item = Value> + 'a
where
    S: Iterator<Item = &'a u8> + 'a,
    T: Iterator<Item = &'a [u8; 32]> + 'a,
    I: Iterator<Item = &'a [u8; 32]> + 'a,
    F: (Fn(u8) -> Value) + 'a,
{
    states
        .zip(tapes.zip(indices))
        .map(move |(state, (tape, index))| {
            Value::prod(
                state_to_value(*state),
                Value::prod(Value::u256_from_slice(tape), Value::u256_from_slice(index)),
            )
        })
}

/// Encode the given program commitment as base64 string.
pub fn encode_base64(commit: &CommitNode<Turing>) -> String {
    let mut program_bytes = Vec::new();
    let mut w = BitWriter::new(&mut program_bytes);
    commit
        .encode(&mut w)
        .map(|_| w.flush_all())
        .expect("encode base64")
        .expect("flushing");
    base64::encode(&program_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::core::Value;
    use simplicity::exec::BitMachine;

    #[test]
    fn eq_2() {
        let mut context = Context::default();
        let program = super::eq_2(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        for a in 0..2 {
            for b in 0..2 {
                let input = Value::prod(Value::u1(a), Value::u1(b));
                let eq = if a == b { Value::u1(1) } else { Value::u1(0) };

                let mut mac = BitMachine::for_program(&program);
                mac.input(&input);
                let output = mac.exec(&program, &()).unwrap();
                assert_eq!(eq, output);
            }
        }
    }

    #[test]
    fn eq_22() {
        let mut context = Context::default();
        let program = super::eq_22(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        for a in 0..4 {
            for b in 0..4 {
                let input = Value::prod(Value::u2(a), Value::u2(b));
                let eq = if a == b { Value::u1(1) } else { Value::u1(0) };

                let mut mac = BitMachine::for_program(&program);
                mac.input(&input);
                let output = mac.exec(&program, &()).unwrap();
                assert_eq!(eq, output);
            }
        }
    }
}
