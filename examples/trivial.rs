use simple_turing::computation::Computation256;
use simple_turing::machine::Trivial;
use simple_turing::util;
use simple_turing::Verifier;
use simplicity::core::{Context, Value};
use simplicity::exec::BitMachine;

fn main() {
    let mut context = Context::default();
    let commit = Verifier::<Computation256, Trivial>::verify_computation(&mut context, 2);
    println!("{}", util::encode_base64(&commit));

    let states = [0, 1];
    let tapes = [
        [
            0b10000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
        [
            0b10000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    ];
    let indices = [
        [
            0b10000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
        [
            0b01000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    ];
    let computation =
        util::computation256_of(states.iter(), tapes.iter(), indices.iter(), Value::u1);

    let program = commit.finalize(computation).unwrap();
    println!("{}", program.ty);

    let mut mac = BitMachine::for_program(&program);
    mac.exec(&program, &()).unwrap();
}
