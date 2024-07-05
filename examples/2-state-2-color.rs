//! 2-state 2-color busy beaver machine:
//!
//! - 2-state: The machine has two operational states (A and B) plus a Halt state
//! - 2-color: The tape uses two symbols (0 and 1, represented by ZERO and ONE)
//!
//! This is the simplest non-trivial Busy Beaver machine configuration.
//! It aims to write the maximum number of 1s on an initially blank tape
//! before halting, using only these limited states and symbols.
//!
//! The transition function for this busy beaver machine is:
//!
//! State A, read 0: write 1, move right, go to state B
//! State A, read 1: write 1, move left, go to state B
//! State B, read 0: write 1, move left, go to state A
//! State B, read 1: write 0, move left, go to Halt state

use bb_machine::TuringMachine;

#[derive(Debug, Clone, Copy, bb_machine::State)]
enum State {
    #[default]
    A,
    B,
    #[halt]
    Halt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, bb_machine::Color)]
enum Symbol {
    #[default]
    ZERO,
    ONE,
}

fn main() {
    let mut machine = TuringMachine::<State, Symbol>::new(
        Vec::new(),
        Box::new(|ctx| {
            let current_state = *ctx.state;
            let current_symbol = ctx.tape[*ctx.head];

            match (current_state, current_symbol) {
                (State::A, Symbol::ZERO) => {
                    ctx.tape[*ctx.head] = Symbol::ONE;
                    *ctx.head += 1;
                    *ctx.state = State::B;
                }

                (State::A, Symbol::ONE) => {
                    ctx.tape[*ctx.head] = Symbol::ONE;
                    *ctx.head -= 1;
                    *ctx.state = State::B;
                }

                (State::B, Symbol::ZERO) => {
                    ctx.tape[*ctx.head] = Symbol::ONE;
                    *ctx.head -= 1;
                    *ctx.state = State::A;
                }

                (State::B, Symbol::ONE) => {
                    ctx.tape[*ctx.head] = Symbol::ZERO;
                    *ctx.head -= 1;
                    *ctx.state = State::Halt;
                }

                (State::Halt, _) => {}
            }
        }),
    );

    machine.run();

    println!("Steps: {}", machine.total_steps());
}
