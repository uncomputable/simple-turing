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
