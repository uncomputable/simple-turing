use crate::computation::Computation;
use crate::jet::application::Turing;
use crate::machine::Machine;
use bitcoin_hashes::sha256::Midstate;
use simplicity::core::Context;
use simplicity::merkle::cmr::Cmr;
use simplicity::CommitNode;
use std::marker::PhantomData;
use std::rc::Rc;

/// Produces Simplicity programs that verify
/// that a given Turing machine computes a given computation.
///
/// The machine is hard-coded by `M`.
/// The computation is provided to the program as witness and must be of the format given by `C`.
pub struct Verifier<C: Computation, M: Machine> {
    _computation: PhantomData<C>,
    _machine: PhantomData<M>,
}

impl<C: Computation, M: Machine> Verifier<C, M> {
    fn pair_q_b(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let iden_m = CommitNode::iden(context).unwrap();
        let take_m_lk = CommitNode::take(context, iden_m).unwrap();
        let q = CommitNode::take(context, take_m_lk).unwrap();

        let iden_lk = CommitNode::iden(context).unwrap();
        let drop_m_lk = CommitNode::drop(context, iden_lk).unwrap();
        let pair_w_i = CommitNode::take(context, drop_m_lk).unwrap();
        let get = C::get(context);
        let b = CommitNode::comp(context, pair_w_i, get).unwrap();

        CommitNode::pair(context, q, b).unwrap()
    }

    fn check_state(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let pair_q_b = Self::pair_q_b(context);
        let state = M::state(context);
        let computed_q_prime = CommitNode::comp(context, pair_q_b, state).unwrap();

        let iden_m = CommitNode::iden(context).unwrap();
        let take_m_lk = CommitNode::take(context, iden_m).unwrap();
        let q_prime = CommitNode::drop(context, take_m_lk).unwrap();

        let pair_computed_q_prime_q_prime =
            CommitNode::pair(context, computed_q_prime, q_prime).unwrap();
        let eq_state = M::eq_state(context);

        CommitNode::comp(context, pair_computed_q_prime_q_prime, eq_state).unwrap()
    }

    fn check_index(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let pair_q_b = Self::pair_q_b(context);
        let left_predicate = M::left(context);
        let left = CommitNode::comp(context, pair_q_b, left_predicate).unwrap();

        let iden_k = CommitNode::iden(context).unwrap();
        let drop_l_k = CommitNode::drop(context, iden_k).unwrap();
        let drop_m_lk = CommitNode::drop(context, drop_l_k).unwrap();
        let i = CommitNode::take(context, drop_m_lk.clone()).unwrap();
        let pair_left_i = CommitNode::pair(context, left, i).unwrap();

        let dec_index = C::dec_index(context);
        let inc_index = C::inc_index(context);
        let cond_dec_inc = CommitNode::cond(context, dec_index, inc_index).unwrap();
        let computed_i_prime = CommitNode::comp(context, pair_left_i, cond_dec_inc).unwrap();

        let i_prime = CommitNode::drop(context, drop_m_lk).unwrap();
        let pair_computed_i_prime_i_prime =
            CommitNode::pair(context, computed_i_prime, i_prime).unwrap();
        let eq_index = C::eq_index(context);

        CommitNode::comp(context, pair_computed_i_prime_i_prime, eq_index).unwrap()
    }

    fn check_tape(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let pair_q_b = Self::pair_q_b(context);
        let write = M::write(context);
        let b = CommitNode::comp(context, pair_q_b, write).unwrap();

        let iden_lk = CommitNode::iden(context).unwrap();
        let drop_m_lk = CommitNode::drop(context, iden_lk).unwrap();
        let pair_w_i = CommitNode::take(context, drop_m_lk).unwrap();
        let pair_b_pair_w_i = CommitNode::pair(context, b, pair_w_i).unwrap();

        let set = C::set(context);
        let computed_w_prime = CommitNode::comp(context, pair_b_pair_w_i, set).unwrap();

        let iden_l = CommitNode::iden(context).unwrap();
        let take_l_k = CommitNode::take(context, iden_l).unwrap();
        let drop_m_lk = CommitNode::drop(context, take_l_k).unwrap();
        let w_prime = CommitNode::drop(context, drop_m_lk).unwrap();

        let pair_computed_w_prime_w_prime =
            CommitNode::pair(context, computed_w_prime, w_prime).unwrap();
        let eq_tape = C::eq_tape(context);

        CommitNode::comp(context, pair_computed_w_prime_w_prime, eq_tape).unwrap()
    }

    fn verify_step(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let check_state = Self::check_state(context);
        let verify_state =
            CommitNode::assert(context, check_state, Cmr(Midstate([0; 32]))).unwrap();

        let check_index = Self::check_index(context);
        let verify_index =
            CommitNode::assert(context, check_index, Cmr(Midstate([1; 32]))).unwrap();
        let verify_state_index = CommitNode::pair(context, verify_state, verify_index).unwrap();

        let check_tape = Self::check_tape(context);
        let verify_tape = CommitNode::assert(context, check_tape, Cmr(Midstate([2; 32]))).unwrap();

        CommitNode::pair(context, verify_state_index, verify_tape).unwrap()
    }

    fn verify_first(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let iden_m = CommitNode::iden(context).unwrap();
        let take_m_lk = CommitNode::take(context, iden_m).unwrap();
        let q = CommitNode::take(context, take_m_lk).unwrap();

        let initial = M::initial(context);
        let check_first = CommitNode::comp(context, q, initial).unwrap();

        CommitNode::assert(context, check_first, Cmr(Midstate([3; 32]))).unwrap()
    }

    fn verify_last(context: &mut Context<Turing>) -> Rc<CommitNode<Turing>> {
        let iden_m = CommitNode::iden(context).unwrap();
        let take_m_lk = CommitNode::take(context, iden_m).unwrap();
        let q_prime = CommitNode::drop(context, take_m_lk).unwrap();

        let accepting = M::accepting(context);
        let check_last = CommitNode::comp(context, q_prime, accepting).unwrap();

        CommitNode::assert(context, check_last, Cmr(Midstate([4; 32]))).unwrap()
    }

    /// Return a program that verifies that the Turing machine computes a computation of `n_steps`.
    ///
    /// The program fails if the validation fails and does nothing otherwise.
    pub fn verify_computation(
        context: &mut Context<Turing>,
        n_steps: usize,
    ) -> Rc<CommitNode<Turing>> {
        assert!(
            n_steps > 1,
            "There must be at least two steps (initial and accepting configuration)"
        );

        let witness_0 = CommitNode::witness(context).unwrap();
        let witness_1 = CommitNode::witness(context).unwrap();
        let first_witnesses = CommitNode::pair(context, witness_0, witness_1).unwrap();

        let verify_step = Self::verify_step(context);
        let verify_first = Self::verify_first(context);
        let verify_last = Self::verify_last(context);

        let verify_first_step =
            CommitNode::pair(context, verify_first, verify_step.clone()).unwrap();
        let verify_last_step =
            CommitNode::pair(context, verify_last.clone(), verify_step.clone()).unwrap();

        let iden_c = CommitNode::iden(context).unwrap();
        let drop_c_c = CommitNode::drop(context, iden_c).unwrap();

        if n_steps == 2 {
            let verify = CommitNode::pair(context, verify_first_step, verify_last).unwrap();
            return CommitNode::comp(context, first_witnesses, verify).unwrap();
        }

        let mut verify_prefix = first_witnesses;

        for i in 2..n_steps {
            let next_witness = CommitNode::witness(context).unwrap();
            let verify = if i == 2 {
                verify_first_step.clone()
            } else {
                verify_step.clone()
            };
            let verify_next_witness = CommitNode::comp(context, verify, next_witness).unwrap();
            let drop_and_verify =
                CommitNode::pair(context, drop_c_c.clone(), verify_next_witness).unwrap();

            verify_prefix = CommitNode::comp(context, verify_prefix, drop_and_verify).unwrap();
        }

        CommitNode::comp(context, verify_prefix, verify_last_step).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::computation::Computation256;
    use crate::machine::TwoBeavers;

    #[test]
    fn type_check() {
        let mut context = Context::default();

        let program = Verifier::<Computation256, TwoBeavers>::pair_q_b(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::check_state(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::check_index(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::check_tape(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::verify_step(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::verify_first(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);

        let program = Verifier::<Computation256, TwoBeavers>::verify_last(&mut context)
            .finalize(std::iter::empty())
            .unwrap();
        println!("{}", program.ty);
    }
}
