use malachite_q::Rational;
use std::{cmp::Ordering, collections::BinaryHeap, ops::AddAssign};

pub struct DelayedAction<'a, S, O = Rational> {
    action: Box<dyn FnOnce(S) -> S + 'a>,
    delay: O,
}

impl<S, O: PartialEq> PartialEq for DelayedAction<'_, S, O> {
    fn eq(&self, other: &Self) -> bool {
        self.delay == other.delay
    }
}

impl<S, O: PartialOrd> PartialOrd for DelayedAction<'_, S, O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.delay.partial_cmp(&other.delay).map(Ordering::reverse)
    }
}

impl<S, O: Ord> Ord for DelayedAction<'_, S, O> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.delay.cmp(&other.delay).reverse()
    }
}

impl<S, O: Eq> Eq for DelayedAction<'_, S, O> {}

impl<'a, S, O> DelayedAction<'a, S, O> {
    pub fn new<F, N>(action: F, delay: N) -> Self
    where
        F: FnOnce(S) -> S + 'a,
        N: Into<O>,
    {
        Self {
            action: Box::new(action),
            delay: delay.into(),
        }
    }
}

//Note: DelayedAction implements a reversed Ord, so this BinaryHeap will be a min-heap
type EventQueue<'a, S, O> = BinaryHeap<DelayedAction<'a, S, O>>;
pub struct StateMachine<'a, S, O = Rational>
where
    O: Ord,
{
    trigger: fn(&S) -> Vec<DelayedAction<'a, S, O>>,
    queue: EventQueue<'a, S, O>,
    state: S,
    time: O,
}

//probably overkill to have this in its own impl block with an anonymous lifetime? could probably be in the 'a impl without sacrificing any flexibility?
//idk enough about rust monomorphization to know whether this is good or bad practice
impl<S, O> StateMachine<'_, S, O>
where
    O: Ord + AddAssign + Clone,
{
    //Runs the state machine until there are no pending events in the queue, and return the end state. is not guaranteed to terminate.
    //I want this to be async/multithreaded compatible, so you can pass functions dependant on user input into the queue, or w/e. we shall see
    pub fn run(mut self) {
        while let Some((next_state, elapsed)) =
            self.queue.pop().map(|f| ((f.action)(self.state), f.delay))
        {
            self.state = next_state;
            self.time += elapsed;
            self.queue
                .extend((self.trigger)(&self.state).into_iter().map(|mut x| {
                    x.delay += self.time.clone();
                    x
                }));
        }
    }
}

impl<'a, S, O> StateMachine<'a, S, O>
where
    O: Ord,
{
    pub fn new<N>(state: S, trigger: fn(&S) -> Vec<DelayedAction<'a, S, O>>, time: N) -> Self
    where
        N: Into<O>,
    {
        Self {
            trigger,
            queue: (trigger)(&state).into(),
            state,
            time: time.into(),
        }
    }
}
