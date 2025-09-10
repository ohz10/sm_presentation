use std::path::PathBuf;

enum State {
    Init,
    Proto,
    Domain,
    Path,
    EndRequest,
    Done,

    Error,
    Size,
}

impl State {
    const fn to_index(self: &Self) -> usize {
        match self {
            State::Init => 0,
            State::Proto => 1,
            State::Domain => 2,
            State::Path => 3,
            State::EndRequest => 4,
            State::Done => 5,
            State::Error => 6,
            State::Size => 7,
        }
    }
}

#[derive(Clone,Copy)]
enum Protocol {
    Http,
    Https,
}

struct Url {
    proto: Protocol,
    path: PathBuf,
}

struct Parser {
    states: [StateFunc; State::Size.to_index()],
    state: State,
    url: Url,
    error_message: &'static str,
}

type StateFunc = fn(parser: &mut Parser, c: u8) -> State;

fn init_state(parser: &mut Parser, c: u8) -> State {
    if c == b'h' {
        return State::Proto;
    }

    parser.error_message = "Bad Protocol";
    State::Error
}

fn proto_state(_parser: &mut Parser, _c: u8) -> State {
    State::Proto
}

fn domain_state(_parser: &mut Parser, _c: u8) -> State {
    State::Domain
}

fn path_state(_parser: &mut Parser, _c: u8) -> State {
    State::Path
}

fn endreq_state(_parser: &mut Parser, _c: u8) -> State {
    State::EndRequest
}

fn done_state(_parser: &mut Parser, _c: u8) -> State {
    State::Done
}

fn error_state(parser: &mut Parser, _c: u8) -> State {
    State::Error
}

impl Parser {
    fn new_states() -> [StateFunc; State::Size.to_index()] {
        [
            init_state,
            proto_state,
            domain_state,
            path_state,
            endreq_state,
            done_state,
            error_state,
        ]
    }

    fn new() -> Parser {
        Parser {
            states: Self::new_states(),
            url: Url {
                proto: Protocol::Http,
                path: "".into(),
            },
            state: State::Init,
            error_message: "",
        }
    }

    fn reset(self: &mut Self) {
        self.url = Url {
            proto: Protocol::Http,
            path: "".into(),
        };

        self.state = State::Init;
        self.error_message = "";
    }


    fn parse(self: &mut Self, buf: &[u8]) {
        for c in buf.iter() {
            self.state = self.states[self.state.to_index()](self, *c);
        }
    }

    fn url(self: &Self) -> Url {
        Url{ proto: self.url.proto, path: self.url.path.clone() }
    }
}

fn main() {
    let mut parser = Parser::new();
    parser.parse(b"http://good.boy.com/file/1");

    let url = parser.url();
    parser.reset();
}
