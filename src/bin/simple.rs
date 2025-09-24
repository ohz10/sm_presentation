enum Event {
    E1,
    E2,
}

struct S1;
struct S2;
struct S3;

enum State {
    S1(S1),
    S2(S2),
    S3(S3),
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
        StateMachine{ state: State::S1(S1{}) }
    }

    fn print(self: &Self) {
        match self.state {
            State::S1(_) => println!("S1"),
            State::S2(_) => println!("S2"),
            State::S3(_) => println!("S3"),
        }
    }

    fn on_event(self: &mut Self, event: Event) {
        let state: State = match (&self.state, event) {
            (State::S1(s), Event::E1) => State::S2(s.into()),
            (State::S1(s), Event::E2) => State::S1(s.into()),
            (State::S2(s), Event::E1) => State::S3(s.into()),
            (State::S2(s), Event::E2) => State::S2(s.into()),
            (State::S3(s), Event::E1) => State::S1(s.into()),
            (State::S3(s), Event::E2) => State::S3(s.into()),
        };

        self.state = state;
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
