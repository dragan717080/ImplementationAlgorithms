use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<usize>().unwrap());
    let [m, _, r] = [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut matrix: Vec<Vec<usize>> = Vec::new();
    
    for _ in 0..m {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        matrix.push(line.split_whitespace().map(|c| c.parse().unwrap()).collect());
    }
    
    for row in rotate(matrix, r) {
        println!("{}", row.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "));
    }
}

struct Matrix {
    m: Vec<Vec<usize>>,
}

impl Matrix {
    fn from_vec(v: Vec<Vec<usize>>) -> Matrix {
        Matrix{m: v}
    }

    fn to_vec(self) -> Vec<Vec<usize>> {
        self.m
    }

    fn width(&self) -> usize {
        self.m[0].len()
    }

    fn height(&self) -> usize {
        self.m.len()
    }

    fn num_of_layers(&self) -> usize {
       [self.height(), self.width()].iter().min().unwrap() / 2
    }

    fn layer_top_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (layer, layer)
    }

    fn layer_right_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (layer, self.width() - 1 - layer)
    }

    fn layer_bottom_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (self.height() - 1 - layer, self.width() - 1 - layer)
    }
    
    fn layer_left_start_coord(&self, layer: usize) -> (usize, usize) {
        assert!(layer < self.num_of_layers());
        (self.height() - 1 - layer, layer)
    }
    
    fn read_layer(&self, layer: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let (top_layer_i, top_layer_j) = self.layer_top_start_coord(layer);
        let (right_layer_i, right_layer_j) = self.layer_right_start_coord(layer);
        let (bottom_layer_i, bottom_layer_j) = self.layer_bottom_start_coord(layer);
        let (left_layer_i, left_layer_j) = self.layer_left_start_coord(layer);

        for j in top_layer_j..right_layer_j {
            result.push(self.m[top_layer_i][j]);
        }

        for i in right_layer_i..bottom_layer_i {
            result.push(self.m[i][right_layer_j])
        }

        for nth_item in 0..bottom_layer_j - left_layer_j {
            let j_idx = bottom_layer_j - nth_item;
            result.push(self.m[bottom_layer_i][j_idx]);
        }

        for nth_item in 0..left_layer_i - top_layer_i {
            let i_idx = bottom_layer_i - nth_item;
            result.push(self.m[i_idx][left_layer_j]);
        }

        result
    }

    fn write_layer(&mut self, layer: usize, layer_data: Vec<usize>) {
        let (top_layer_i, top_layer_j) = self.layer_top_start_coord(layer);
        let (right_layer_i, right_layer_j) = self.layer_right_start_coord(layer);
        let (bottom_layer_i, bottom_layer_j) = self.layer_bottom_start_coord(layer);
        let (left_layer_i, left_layer_j) = self.layer_left_start_coord(layer);

        let mut data_idx = 0;
        for j in top_layer_j..right_layer_j {
            self.m[top_layer_i][j] = layer_data[data_idx];
            data_idx += 1;
        }

        for i in right_layer_i..bottom_layer_i {
            self.m[i][right_layer_j] = layer_data[data_idx];
            data_idx += 1;
        }

        for nth_item in 0..bottom_layer_j - left_layer_j {
            let j_idx = bottom_layer_j - nth_item;
            self.m[bottom_layer_i][j_idx] = layer_data[data_idx];
            data_idx += 1;
        }

        for nth_item in 0..left_layer_i - top_layer_i {
            let i_idx = bottom_layer_i - nth_item;
            self.m[i_idx][left_layer_j] = layer_data[data_idx];
            data_idx += 1;
        }
    }

    fn rotate(&mut self, r: usize) {
        for i in 0..self.num_of_layers() {
            let mut layer = self.read_layer(i);
            let rotations = r % layer.len();
            layer.rotate_left(rotations);
            self.write_layer(i, layer);
        }
    }
}

fn rotate(v: Vec<Vec<usize>>, r: usize) -> Vec<Vec<usize>> {
    let mut matrix = Matrix::from_vec(v);
    matrix.rotate(r);
    matrix.to_vec()
}
