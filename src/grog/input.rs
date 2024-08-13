use std::collections::HashMap;
use console::Key;
use console::Term;

struct Stack<T: Clone> {
    data: Vec<T>,
    max_capacity: usize,
    bottom: usize,
    top: usize,
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
            self.bottom = (self.bottom + 1) % self.max_capacity;
        }
        self.data.push(element);
        self.top = (self.top + 1) % self.max_capacity;
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
        (self.top + 1) % self.max_capacity == self.bottom
    }
}

pub struct Input {
    // cursor pos
    pub cursor_x_pos: usize,
    pub cursor_y_pos: usize,
    // keyboard input
    keys: Stack<Key>,
    input_map: HashMap<Key,fn(&mut Input) -> ()>,
}

impl Input {
    pub fn new() -> Input {
        let mut input_map: HashMap<Key, fn(&mut Input) -> ()> = HashMap::new();
        input_map.insert(Key::ArrowRight, Input::arrow_key_right);
        input_map.insert(Key::ArrowLeft, Input::arrow_key_left);
        input_map.insert(Key::ArrowUp, Input::arrow_key_up);
        input_map.insert(Key::ArrowDown, Input::arrow_key_down);

        Input {
            keys: Stack::new(8),
            input_map,
            cursor_x_pos: 1,
            cursor_y_pos: 1,
        }
    }

    pub fn process_key_event(&mut self) -> Option<()> {
        let key = self.last_key()?;
        let action = self.input_map.get(&key)?;
        Some(action(self))
    }

    pub fn get_key(&mut self) {
        if let Ok(key) = Term::buffered_stdout().read_key() {
            self.keys.push(key);
        }
    }

    fn last_key(&self) -> Option<Key> {
        self.keys.peek()
    }

    pub fn arrow_key_right(&mut self) -> () {
        let terminal_width: usize = Term::stdout().size().1 as usize;
        if self.cursor_x_pos >= terminal_width - 2 {
            self.cursor_x_pos = terminal_width - 2
        } else {
            self.cursor_x_pos += 1
        }
    }

    pub fn arrow_key_left(&mut self) -> () {
        if self.cursor_x_pos > 1 {
            self.cursor_x_pos -= 1
        } else {
            self.cursor_x_pos = 1
        }
    }

    pub fn arrow_key_up(&mut self) -> () {
        if self.cursor_y_pos > 1 {
            self.cursor_y_pos -= 1
        } else {
            self.cursor_y_pos = 1
        }
    }

    pub fn arrow_key_down(&mut self) -> () {
        let terminal_height: usize = Term::stdout().size().0 as usize;
        if self.cursor_y_pos >= terminal_height - 2 {
            self.cursor_y_pos = terminal_height - 2
        } else {
            self.cursor_y_pos += 1
        }
    }
}