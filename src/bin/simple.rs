// uses type system to prevent invalid state transitions
// however, does not account for event type

enum Event {
    E1,
    E2,
}

trait Transitions {
    fn event_1(&self) -> State;
    fn event_2(&self) -> State;
}

struct S1;
struct S2;
struct S3;

enum State {
    S1(S1),
    S2(S2),
    S3(S3),
}

impl Transitions for S1 {
    fn event_1(&self) -> State {
        State::S2(self.into())
    }

    fn event_2(&self) -> State {
        State::S1(self.into())
    }
}

impl Transitions for S2 {
    fn event_1(&self) -> State {
        State::S3(self.into())
    }

    fn event_2(&self) -> State {
        State::S2(self.into())
    }
}

impl Transitions for S3 {
    fn event_1(&self) -> State {
        State::S1(self.into())
    }

    fn event_2(&self) -> State {
        State::S3(self.into())
    }
}

impl From<&S1> for S1 {
    fn from(_: &S1) -> S1 {
        S1
    }
}

impl From<&S1> for S2 {
    fn from(_: &S1) -> S2 {
        S2
    }
}

impl From<&S2> for S2 {
    fn from(_: &S2) -> S2 {
        S2
    }
}

impl From<&S2> for S3 {
    fn from(_: &S2) -> S3 {
        S3
    }
}

impl From<&S3> for S3 {
    fn from(_: &S3) -> S3 {
        S3
    }
}

impl From<&S3> for S1 {
    fn from(_: &S3) -> S1 {
        S1
    }
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: State::S1(S1 {}),
        }
    }

    fn print(&self) {
        match self.state {
            State::S1(_) => println!("S1"),
            State::S2(_) => println!("S2"),
            State::S3(_) => println!("S3"),
        }
    }

    fn on_event(&mut self, event: Event) {
        self.state = match (&self.state, event) {
            (State::S1(s), Event::E1) => s.event_1(),
            (State::S1(s), Event::E2) => s.event_2(),
            (State::S2(s), Event::E1) => s.event_1(),
            (State::S2(s), Event::E2) => s.event_2(),
            (State::S3(s), Event::E1) => s.event_1(),
            (State::S3(s), Event::E2) => s.event_2(),
        };
    }
}

fn main() {
    let mut sm = StateMachine::new();
    sm.on_event(Event::E1);
    sm.print();

    sm.on_event(Event::E1);
    sm.print();

    sm.on_event(Event::E1);
    sm.print();

    sm.on_event(Event::E2);
    sm.print();
}
