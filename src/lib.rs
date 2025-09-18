pub struct Stack {
    stack: [char; 4],
    index: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: ['\0'; 4],
            index: usize::MAX,
        }
    }

    // returns None on success
    pub fn push(self: &mut Self, c: char) -> Option<String> {
        match self.index {
            0..3 => {
                self.index += 1;
                self.stack[self.index] = c;
                None
            }
            usize::MAX => {
                self.index = 0;
                self.stack[self.index] = c;
                None
            }
            _ => Some(String::from("stack overflow")),
        }
    }

    pub fn pop_all(self: &mut Self) {
        self.index = usize::MAX
    }

    pub fn top(self: &Self) -> Option<char> {
        match self.index {
            0..4 => Some(self.stack[self.index as usize]),
            _ => None,
        }
    }

    pub fn empty(self: &Self) -> bool {
        self.index == usize::MAX
    }

    pub fn size(self: &Self) -> usize {
        self.index
    }
}
