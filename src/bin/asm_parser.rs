use sm_presentation::Stack;

#[derive(Clone, Copy, Debug, PartialEq)]
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

type StateFunc = fn(parser: Parser, c: char) -> Parser;
impl Parser {
    fn init_state(mut self, c: char) -> Self {
        if c == 'h' {
            self.state = State::Proto;
            return self;
        }

        self.error = Some("bad protocol".into());
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
                    self.url.proto = Protocol::Https;
                    self.stack.pop_all();
                    self.state = State::Slash;
                }
                _ => {
                    self.error = Some("bad protocol".into());
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
                    self.error = Some("invalid url".into());
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
    // process the URL as it comes asynchronously from the network
    let url: Vec<String> = vec![
        "htt".into(),
        "ps:".into(),
        "://fast.parser.io".into(),
        "/path/".into(),
        "file1".into(),
    ];

    let mut parser = Parser::new();
    for part in url.iter() {
        parser = parser.parse(&part);
    }

    match parser.error {
        Some(e) => println!("{e}"),
        _ => println!("{0:?}: {1}", parser.url.proto, parser.url.path),
    }
}

#[cfg(test)]
mod tests {

    use crate::Parser;
    use crate::Protocol;

    #[test]
    fn test_urls() {
        let mut parser = Parser::new();
        parser = parser.parse(&"http://fast.parser.io/file/1".into());

        assert!(parser.error.is_none());
        assert_eq!(Protocol::Http, parser.url.proto);
        assert_eq!(String::from("file/1"), parser.url.path);

        let mut parser = Parser::new();
        parser = parser.parse(&"https://fast.parser.io/file/2".into());

        assert!(parser.error.is_none());
        assert_eq!(Protocol::Https, parser.url.proto);
        assert_eq!(String::from("file/2"), parser.url.path);

        let mut parser = Parser::new();
        parser = parser.parse(&"http://parser.io/another/path?query=abc".into());

        assert!(parser.error.is_none());
        assert_eq!(Protocol::Http, parser.url.proto);
        assert_eq!(String::from("another/path"), parser.url.path);

        let mut parser = Parser::new();
        parser = parser.parse(&"ftp://parser.io/another/path".into());

        assert!(parser.error.is_some());
        assert_eq!(String::from("bad protocol"), parser.error.unwrap());
    }
}
