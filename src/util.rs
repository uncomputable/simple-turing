use crate::jet::application::Turing;
use simplicity::bitwriter::BitWriter;
use simplicity::core::{Context, Value};
use simplicity::CommitNode;
use std::rc::Rc;

/// Check equality of two bits.
///
/// `eq_2: 2 × 2 → 2`
pub fn eq_2(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
    // A → 2
    let bit_false = CommitNode::bit_false(context).unwrap();
    // B → 2
    let bit_true = CommitNode::bit_true(context).unwrap();
    // 2 → 2
    let iden2 = CommitNode::iden(context).unwrap();
    // 2 → 1
    let unit2 = CommitNode::unit(context).unwrap();
    // 2 → 2 × 1
    let pair_iden2_unit2 = CommitNode::pair(context, iden2, unit2).unwrap();
    // 2 × 1 → 2
    let case_true_false = CommitNode::case(context, bit_true.clone(), bit_false.clone()).unwrap();
    // 2 → 2
    let adapted_case_true_false =
        CommitNode::comp(context, pair_iden2_unit2.clone(), case_true_false).unwrap();
    // 2 × 1 → 2
    let case_false_true = CommitNode::case(context, bit_false, bit_true).unwrap();
    // 2 → 2
    let adapted_case_false_true =
        CommitNode::comp(context, pair_iden2_unit2, case_false_true).unwrap();
    // 2 × 2 → 2
    CommitNode::cond(context, adapted_case_false_true, adapted_case_true_false).unwrap()
}

/// Create a computation from states, tapes and indices, as witness for a program commitment.
pub fn computation256_of<'a, S, T, I>(
    states: S,
    tapes: T,
    indices: I,
) -> impl Iterator<Item = Value> + 'a
where
    S: Iterator<Item = &'a u8> + 'a,
    T: Iterator<Item = &'a [u8; 32]> + 'a,
    I: Iterator<Item = &'a [u8; 32]> + 'a,
{
    states
        .zip(tapes.zip(indices))
        .map(|(state, (tape, index))| {
            Value::prod(
                Value::u1(*state),
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

        let a_b_eq = vec![
            (Value::u1(0), Value::u1(0), Value::u1(1)),
            (Value::u1(0), Value::u1(1), Value::u1(0)),
            (Value::u1(1), Value::u1(0), Value::u1(0)),
            (Value::u1(1), Value::u1(1), Value::u1(1)),
        ];

        for (a, b, eq) in a_b_eq {
            let mut mac = BitMachine::for_program(&program);
            mac.input(&Value::prod(a, b));
            let output = mac.exec(&program, &()).unwrap();
            assert_eq!(eq, output);
        }
    }
}
