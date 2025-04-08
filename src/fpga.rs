
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

    fn set_truth_value(&mut self, index: usize, value: bool) {
        self.truth_table[index] = value;
    }

    fn toggle_mem(&mut self) {
        self.use_mem = !self.use_mem;
    }

    fn compute_value(&self, inputs: &Vec<bool>) -> bool {
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

    fn clock(&mut self, inputs: &Vec<bool>) {
        self.state = self.compute_value(inputs);
    }

    fn get_output(&self, inputs: &Vec<bool>) -> bool {
        if (self.use_mem) {
            self.state
        }
        else {
            self.compute_value(inputs)
        }
    }
}

pub struct LMUInterface {
    id: u32,
    x: i32,
    y: i32,
    face_north: bool,
    interface_size: usize,
    bus_size: usize,
    connections: Vec<bool>,
    bus_values: Vec<bool>,
}

impl LMUInterface {
    fn new(id: u32, x: i32, y: i32, face_north: bool, interface_size: usize, bus_size: usize) -> Self {
        Self {
            id: id,
            x: x,
            y: y,
            face_north: face_north,
            interface_size: interface_size,
            bus_size: bus_size,
            connections: vec![false; bus_size * (interface_size + 1)],
            bus_values: vec![false; bus_size]
        }
    }

    fn configure(&mut self, bus_index: usize, interface_index: usize, value: bool) {
        let index = bus_index * (self.interface_size + 1) + interface_index;
        self.connections[index] = value;
    }

    fn update_values(&mut self, inputs: &Vec<bool>, logic_memory_unit: &LMUnit) {
        let mut interface_values = vec![false; self.interface_size + 1];
        for i in 0..self.bus_size {
            if !inputs[i] {continue;}

            for j in 0..self.interface_size {
                let index = i * (self.interface_size + 1) + j;

                if self.connections[index] {
                    interface_values[j] = true;
                }
            }
        }

        let logic_output = logic_memory_unit.get_output(&interface_values);

        for i in 0..self.bus_size {
            self.bus_values[i] = inputs[i] || (logic_output && self.connections[(i + 1) * (self.interface_size + 1) - 1]);
        }
    }
}

pub struct MatrixCommutator {
    id: u32,
    x: i32,
    y: i32,
    face_north: bool,
    bus_size: usize,
    cross_size: usize,
    connections: Vec<bool>,
    bus_values: Vec<bool>,
    left_cross_values: Vec<bool>,
    right_cross_values: Vec<bool>
}

impl MatrixCommutator {
    fn new(id: u32, x: i32, y: i32, face_north: bool, bus_size: usize, cross_size: usize) -> Self {
        Self {
            id: id,
            x: x,
            y: y,
            face_north: face_north,
            bus_size: bus_size,
            cross_size: cross_size,
            connections: vec![false; bus_size * cross_size],
            bus_values: vec![false; bus_size],
            left_cross_values: vec![false; cross_size / 2],
            right_cross_values: vec![false; cross_size / 2]
        }
    }

    fn configure(&mut self, bus_index: usize, cross_index: usize, value: bool) {
        let index = bus_index * self.cross_size + cross_index;
        self.connections[index] = value;
    }

    fn update_values(&mut self, bus_inputs: &Vec<bool>, left_cross_inputs: &Vec<bool>, right_cross_inputs: &Vec<bool>) {
        self.bus_values = bus_inputs.clone();
        for i in 0..self.cross_size {
            
        }
    }
}
