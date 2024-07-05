pub use ::bb_derive::{Color, State};

pub trait Color: Default + PartialEq + Eq {}

pub trait State: Default {
    fn is_halt(&self) -> bool;
}

/// The machine context
pub struct Ctx<'a, S, C> {
    pub head: &'a mut usize,
    pub state: &'a mut S,
    pub tape: &'a mut Vec<C>,
}

pub struct TuringMachine<S, C> {
    steps: usize,
    head: usize,
    tape: Vec<C>,
    current_state: S,
    transition_fn: Box<dyn FnMut(Ctx<'_, S, C>)>,
}

impl<S, C> TuringMachine<S, C>
where
    S: State,
    C: Color,
{
    pub fn new(tape: Vec<C>, transition_fn: Box<dyn FnMut(Ctx<'_, S, C>)>) -> Self {
        TuringMachine {
            tape,
            head: 0,
            steps: 0,
            transition_fn,
            current_state: S::default(),
        }
    }

    pub fn run(&mut self) {
        while !self.current_state.is_halt() {
            self.step()
        }
    }

    fn step(&mut self) {
        if self.head == 0 {
            self.tape.insert(0, C::default());

            if !self.tape.is_empty() {
                self.head += 1;
            }
        }

        if self.head == self.tape.len() {
            self.tape.push(C::default());
        }

        let ctx = Ctx {
            head: &mut self.head,
            state: &mut self.current_state,
            tape: &mut self.tape,
        };

        (self.transition_fn)(ctx);
        self.steps += 1;
    }

    pub fn tape(&self) -> &[C] {
        &self.tape
    }

    pub fn total_steps(&self) -> usize {
        self.steps
    }
}
