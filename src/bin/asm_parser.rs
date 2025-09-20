use sm_presentation::Stack;

#[derive(Clone, Copy, Debug)]
enum Protocol {
    Http,
    Https,
}

struct Url {
    proto: Protocol,
    path: String,
}

impl Url {
    fn new() -> Url {
        Url {
            proto: Protocol::Http,
            path: "".into(),
        }
    }
}

enum State {
    Init,
    Proto,
    Slash,
    Domain,
    Path,
    Done,

    Error,
    Size,
}

impl State {
    const fn to_index(self: &Self) -> usize {
        match self {
            State::Init => 0,
            State::Proto => 1,
            State::Slash => 2,
            State::Domain => 3,
            State::Path => 4,
            State::Done => 5,
            State::Error => 6,
            State::Size => 7,
        }
    }
}

struct Parser {
    states: [StateFunc; State::Size.to_index()],
    state: State,

    stack: Stack,

    url: Url,
    error: Option<String>,
}

// TODO: if I pull the state functions out of Parser into its own object
type StateFunc = fn(parser: Parser, c: char) -> Parser;
impl Parser {
    fn init_state(mut self, c: char) -> Self {
        if c == 'h' {
            self.state = State::Proto;
            return self;
        }

        self.error = Some(String::from("bad protocol"));
        self.state = State::Error;
        self
    }

    fn proto_state(mut self, c: char) -> Self {
        if c == ':' {
            match self.stack.top() {
                Some('p') => {
                    self.stack.pop_all();
                    self.state = State::Slash;
                }
                Some('s') => {
                    self.stack.pop_all();
                    self.url.proto = Protocol::Https;
                    self.state = State::Slash;
                }
                _ => {
                    self.error = Some(String::from("bad protocol"));
                    self.state = State::Error;
                }
            }
        } else {
            match self.stack.push(c) {
                Ok(_) => {}
                Err(e) => {
                    self.error = Some(e);
                    self.state = State::Error;
                }
            };
        }

        self
    }

    fn slash_state(mut self, c: char) -> Self {
        if c == '/' {
            match self.stack.top() {
                Some('/') => {
                    self.stack.pop_all();
                    self.state = State::Domain;
                }
                None => match self.stack.push(c) {
                    Ok(_) => {}
                    Err(e) => {
                        self.error = Some(e);
                        self.state = State::Error;
                    }
                },
                _ => {
                    self.error = Some(String::from("invalid url"));
                    self.state = State::Error;
                }
            }
        }

        self
    }

    fn domain_state(mut self, c: char) -> Self {
        if c == '/' {
            self.state = State::Path;
        }

        self
    }

    fn path_state(mut self, c: char) -> Self {
        match c {
            '?' => {
                self.state = State::Done;
            }
            _ => {
                self.url.path.push(c);
            }
        }
        self
    }

    fn done_state(self, _: char) -> Parser {
        self
    }

    fn error_state(self, _c: char) -> Parser {
        self
    }

    fn new_states() -> [StateFunc; State::Size.to_index()] {
        [
            Self::init_state,
            Self::proto_state,
            Self::slash_state,
            Self::domain_state,
            Self::path_state,
            Self::done_state,
            Self::error_state,
        ]
    }

    fn new() -> Parser {
        Parser {
            states: Self::new_states(),
            state: State::Init,
            url: Url::new(),
            stack: Stack::new(),
            error: None,
        }
    }

    fn parse(mut self, buf: &String) -> Parser {
        for c in buf.chars() {
            self = self.states[self.state.to_index()](self, c);
        }

        self
    }
}

fn main() {
    let urls = vec![
        String::from("http://fast.parser.io/file/1"),
        String::from("https://fast.parser.io/file/2"),
        String::from("http://parser.io/another/path?query=abc"),
        String::from("ftp://parser.io/another/path"),
    ];

    for url in urls.iter() {
        let mut parser = Parser::new();
        parser = parser.parse(&url);

        match parser.error {
            Some(e) => println!("{e}"),
            _ => println!("{0:?}: {1}", parser.url.proto, parser.url.path),
        }
    }
}
