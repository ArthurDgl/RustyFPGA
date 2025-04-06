
pub struct LMUnit {
    id: u32,
    x: i32,
    y: i32,
    size: usize,
    truth_table: Vec<bool>,
    use_mem: bool,
    state: bool
}

impl LMUnit {
    fn new(id: u32, x: i32, y: i32, size: usize) -> Self {
        Self {
            id: id,
            x: x,
            y: y,
            size: size,
            truth_table: vec![false; 2usize.pow(size as u32)],
            use_mem: false,
            state: false
        }
    }

    fn toggle_mem(&mut self) {
        self.use_mem = !self.use_mem;
    }

    fn compute_value(&self, inputs: Vec<bool>) -> bool {
        if inputs.len() != self.size {
            panic!("Input count cannot be different from LM Unit size !");
        }

        let mut index: usize = 0;
        for i in 0..self.size {
            if inputs[i] {
                index += 2usize.pow(i as u32);
            }
        }

        self.truth_table[index]
    }

    fn clock(&mut self, inputs: Vec<bool>) {
        self.state = self.compute_value(inputs);
    }

    fn get_output(&self, inputs: Vec<bool>) -> bool {
        if (self.use_mem) {
            self.state
        }
        else {
            self.compute_value(inputs)
        }
    }
}