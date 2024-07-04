use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
enum State {
    #[default]
    A,
    B,
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
enum Symbol {
    #[default]
    ZERO,
    ONE,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
struct TransitionFx {
    write: Symbol,
    new_state: State,
    direction: Direction,
}

#[derive(Debug, Default)]
struct TransitionFxTable {
    table: HashMap<(State, Symbol), TransitionFx>,
}

impl TransitionFxTable {
    fn get(&self, state: State, symbol: Symbol) -> Option<&TransitionFx> {
        self.table.get(&(state, symbol))
    }
}

impl<I> From<I> for TransitionFxTable
where
    I: Iterator,
    I::Item: Into<((State, Symbol), TransitionFx)>,
{
    fn from(values: I) -> Self {
        Self {
            table: HashMap::from_iter(values.map(|i| i.into())),
        }
    }
}

struct TuringMachine {
    head: usize,
    tape: Vec<Symbol>,
    current_state: State,
    transitions: TransitionFxTable,
    steps: usize,
}

///
/// The transition function for this busy beaver machine is:
///
/// State A, read 0: write 1, move right, go to state B
/// State A, read 1: write 1, move left, go to state B
/// State B, read 0: write 1, move left, go to state A
/// State B, read 1: write 0, move left, go to Halt state
///
impl TuringMachine {
    fn new(tape: Vec<Symbol>) -> Self {
        TuringMachine {
            tape,
            head: 0,
            steps: 0,
            current_state: State::default(),
            transitions: Self::transition_functions().into(),
        }
    }

    fn run(&mut self) {
        while self.current_state != State::Halt {
            self.step()
        }
    }

    fn step(&mut self) {
        if self.head == 0 {
            self.tape.insert(0, Symbol::default());
            self.head += 1;
        } else if self.head == self.tape.len() {
            self.tape.push(Symbol::default());
        }

        // get the symbol at HEAD
        let symbol = &mut self.tape[self.head];
        let state = &mut self.current_state;
        let fx = self.transitions.get(*state, *symbol).unwrap();

        match fx.direction {
            Direction::LEFT => self.head -= 1,
            Direction::RIGHT => self.head += 1,
        }

        *symbol = fx.write;
        *state = fx.new_state;
        self.steps += 1;
    }

    fn transition_functions() -> impl Iterator<Item = ((State, Symbol), TransitionFx)> {
        macro_rules! fx {
            ($write:expr, $new_state:expr, $direction:expr) => {
                TransitionFx {
                    write: $write,
                    new_state: $new_state,
                    direction: $direction,
                }
            };
        }

        [
            (
                (State::A, Symbol::ZERO),
                fx!(Symbol::ONE, State::B, Direction::RIGHT),
            ),
            (
                (State::A, Symbol::ONE),
                fx!(Symbol::ONE, State::B, Direction::LEFT),
            ),
            (
                (State::B, Symbol::ZERO),
                fx!(Symbol::ONE, State::A, Direction::LEFT),
            ),
            (
                (State::B, Symbol::ONE),
                fx!(Symbol::ZERO, State::Halt, Direction::LEFT),
            ),
        ]
        .into_iter()
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let tape_length = rng.gen_range(10..=20);

    let tape: Vec<Symbol> = (0..tape_length)
        .map(|_| {
            if rng.gen_bool(0.5) {
                Symbol::ONE
            } else {
                Symbol::ZERO
            }
        })
        .collect();

    let mut machine = TuringMachine::new(tape);
    machine.run();

    assert!(
        machine.steps <= 6,
        "this BB machine should run in maximum only 6 steps"
    );

    println!("Steps: {}", machine.steps);
}
