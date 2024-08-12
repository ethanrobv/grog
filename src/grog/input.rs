use std::collections::HashMap;
use std::io;
use console::Key;
use console::Term;

struct Stack<T: Clone> {
    data: Vec<T>,
    max_capacity: usize,
    bottom: i32,
    top: i32,
}

impl<T: Clone> Stack<T> {
    pub fn new(max_capacity: usize) -> Stack<T> {
        Stack {
            data: Vec::with_capacity(max_capacity),
            max_capacity,
            bottom: 0,
            top: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        if self.is_full() {
            self.bottom = (self.bottom + 1) % self.max_capacity as i32;
        }
        self.data.push(element);
        self.top = (self.top + 1) % self.max_capacity as i32;
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        self.data.last().cloned()
    }

    fn is_empty(&self) -> bool {
        self.top == self.bottom
    }

    fn is_full(&self) -> bool {
        (self.top + 1) % self.max_capacity as i32 == self.bottom
    }
}

pub struct Input {
    // keyboard input
    keys: Stack<Key>,
    input_map: HashMap<Key, Box<dyn Fn(&Input) -> io::Result<()>>>,
}

impl Input {
    pub fn new() -> Input {
        let mut input_map: HashMap<Key, Box<dyn Fn(&Input) -> io::Result<()>>> = HashMap::new();
        input_map.insert(Key::ArrowRight, Box::new(Input::arrow_key_right));
        input_map.insert(Key::ArrowLeft, Box::new(Input::arrow_key_left));
        input_map.insert(Key::ArrowUp, Box::new(Input::arrow_key_up));
        input_map.insert(Key::ArrowDown, Box::new(Input::arrow_key_down));

        Input {
            keys: Stack::new(8),
            input_map,
        }
    }

    pub fn get_key(&mut self) {
        if let Ok(key) = Term::buffered_stdout().read_key() {
            self.keys.push(key);
        }
    }

    fn last_key(&self) -> Option<Key> {
        self.keys.peek()
    }

    pub fn process_events(&self) -> Option<io::Result<()>> {
        match self.last_key() {
            Some(key) => self.input_map.get(&key).map(|action| action(self)),
            None => None,
        }
    }

    pub fn arrow_key_right(&self) -> io::Result<()> {
        Term::stdout().move_cursor_right(1)
    }

    pub fn arrow_key_left(&self) -> io::Result<()> {
        Term::stdout().move_cursor_left(1)
    }

    pub fn arrow_key_up(&self) -> io::Result<()> {
        Term::stdout().move_cursor_up(1)
    }

    pub fn arrow_key_down(&self) -> io::Result<()> {
        Term::stdout().move_cursor_down(1)
    }
}