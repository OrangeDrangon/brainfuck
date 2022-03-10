#[derive(Debug, Default, Clone, Copy)]
struct CellPointer(isize);

impl CellPointer {
    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.0 = self.0.wrapping_sub(1);
    }

    fn val(&self) -> isize {
        self.0
    }

    fn index(&self) -> usize {
        if self.val() >= 0 {
            self.val() as usize
        } else {
            self.val().abs() as usize - 1
        }
    }
}

#[derive(Debug)]
pub struct Cells {
    pointer: CellPointer,
    negative: Vec<u8>,
    positive: Vec<u8>,
}

impl Cells {
    pub fn get_cell(&self) -> u8 {
        if self.pointer.val() >= 0 {
            self.positive[self.pointer.index()]
        } else {
            self.negative[self.pointer.index()]
        }
    }

    pub fn set_cell(&mut self, val: u8) {
        if self.pointer.val() >= 0 {
            self.positive[self.pointer.index()] = val;
        } else {
            self.negative[self.pointer.index()] = val;
        }
    }

    pub fn increment_pointer(&mut self) {
        self.pointer.increment();
        self.gaurentee_cell_pointer();
    }

    pub fn decrement_pointer(&mut self) {
        self.pointer.decrement();
        self.gaurentee_cell_pointer();
    }

    pub fn increment_cell(&mut self) {
        self.set_cell(self.get_cell().wrapping_add(1));
    }

    pub fn decrement_cell(&mut self) {
        self.set_cell(self.get_cell().wrapping_sub(1));
    }

    fn gaurentee_cell_pointer(&mut self) {
        let needed_size = self.pointer.index() + 1;

        if self.pointer.val() >= 0 && self.positive.len() < needed_size {
            self.positive.push(0);
        } else if self.pointer.val() < 0 && self.negative.len() < needed_size {
            self.negative.push(0);
        }
    }
}

impl Default for Cells {
    fn default() -> Self {
        Self {
            pointer: Default::default(),
            negative: Default::default(),
            positive: vec![0; 1],
        }
    }
}
