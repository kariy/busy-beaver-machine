use std::collections::VecDeque;

pub use ::bb_derive::{Color, State};

pub trait Color: Default + PartialEq + Eq {}

pub trait State: Default {
    fn is_halt(&self) -> bool;
}

/// The machine context
pub struct Ctx<'a, S, C> {
    pub head: &'a mut usize,
    pub state: &'a mut S,
    pub tape: &'a mut VecDeque<C>,
}

impl<'a, S, C> Ctx<'a, S, C>
where
    S: State,
    C: Color,
{
    pub fn read_at(&mut self, index: usize) -> &C {
        self.expand_tape_till_offset(index as isize);
        if let Some(value) = self.tape.get(index) {
            value
        } else {
            &self.tape[index]
        }
    }

    /// Move the head relative to its current position
    pub fn move_head(&mut self, offset: isize) {
        let new_position = dbg!((*self.head as isize) + dbg!(offset));
        dbg!(self.expand_tape_till_offset(offset));

        if new_position.is_positive() {
            *self.head = new_position as usize;
        } else {
            *self.head = 0;
        }
    }

    /// Expand tape until offset from zero.
    /// This is a no-op if the offset is 0 <= offset < tape len, unless the tape is empty.
    fn expand_tape_till_offset(&mut self, offset: isize) -> usize {
        if offset < 0 {
            let expand_by = (-offset) as usize + 1;
            (0..expand_by).for_each(|_| self.tape.push_front(C::default()));
            return expand_by;
        }

        if offset >= self.tape.len() as isize {
            let expand_by = (offset as usize - self.tape.len()) + 1;
            (0..expand_by).for_each(|_| self.tape.push_back(C::default()));
            return expand_by;
        }

        0
    }
}

pub struct TuringMachine<S, C> {
    steps: usize,
    head: usize,
    tape: VecDeque<C>,
    current_state: S,
    transition_fn: Box<dyn FnMut(Ctx<'_, S, C>)>,
}

impl<S, C> TuringMachine<S, C>
where
    S: State,
    C: Color,
{
    pub fn new<I, F>(tape: I, transition_fn: F) -> Self
    where
        I: IntoIterator<Item = C>,
        F: FnMut(Ctx<'_, S, C>) + 'static,
    {
        TuringMachine {
            head: 0,
            steps: 0,
            current_state: S::default(),
            tape: VecDeque::from_iter(tape),
            transition_fn: Box::new(transition_fn),
        }
    }

    pub fn run(&mut self) {
        while !self.current_state.is_halt() {
            self.step()
        }
    }

    pub fn steps(&self) -> usize {
        self.steps
    }

    pub fn tape(&self) -> impl Iterator<Item = &C> {
        self.tape.iter()
    }

    fn step(&mut self) {
        self.run_transition();
        self.steps += 1;
    }

    fn run_transition(&mut self) {
        (self.transition_fn)(Ctx {
            head: &mut self.head,
            tape: &mut self.tape,
            state: &mut self.current_state,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    struct TestColor;
    impl Color for TestColor {}

    #[derive(Default)]
    struct TestState;
    impl State for TestState {
        fn is_halt(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_expand_tape_empty() {
        let mut ctx = Ctx {
            head: &mut 0,
            state: &mut TestState::default(),
            tape: &mut VecDeque::<TestColor>::new(),
        };

        assert_eq!(ctx.tape.len(), 0);

        ctx.expand_tape_till_offset(3);

        assert_eq!(ctx.tape.len(), 4);
        assert!(ctx.tape.iter().all(|&c| c == TestColor::default()));
    }

    #[test]
    fn test_expand_tape_negative() {
        let mut ctx = Ctx {
            head: &mut 0,
            state: &mut TestState::default(),
            tape: &mut VecDeque::<TestColor>::new(),
        };

        ctx.expand_tape_till_offset(-2);
        assert_eq!(ctx.tape.len(), 3);
        assert!(ctx.tape.iter().all(|&c| c == TestColor::default()));
    }

    #[test]
    fn test_expand_tape_existing() {
        let mut ctx = Ctx {
            head: &mut 0,
            state: &mut TestState::default(),
            tape: &mut VecDeque::from(vec![TestColor::default(); 2]),
        };

        ctx.expand_tape_till_offset(5);
        assert_eq!(ctx.tape.len(), 6);
        assert!(ctx.tape.iter().all(|&c| c == TestColor::default()));
    }

    #[test]
    fn test_expand_tape_no_change() {
        let mut ctx = Ctx {
            head: &mut 0,
            state: &mut TestState::default(),
            tape: &mut VecDeque::from(vec![TestColor::default(); 3]),
        };

        ctx.expand_tape_till_offset(2);
        assert_eq!(ctx.tape.len(), 3);
        assert!(ctx.tape.iter().all(|&c| c == TestColor::default()));
    }

    #[test]
    fn test_expand_tape_empty_at_zero() {
        let mut ctx = Ctx {
            head: &mut 0,
            state: &mut TestState::default(),
            tape: &mut VecDeque::<TestColor>::new(),
        };

        ctx.expand_tape_till_offset(0);
        assert_eq!(ctx.tape.len(), 1);
        assert!(ctx.tape.iter().all(|&c| c == TestColor::default()));
    }
}
