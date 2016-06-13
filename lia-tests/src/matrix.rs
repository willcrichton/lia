use lia::runtime::*;

lia! {
    function multiply_matrices() {
        var x = @Matrix::new(1, 1);
        var y = @Matrix::new(1, 1);
        @Matrix::set(x, 0, 0, 5);
        @Matrix::set(y, 0, 0, 10);
        var z = @Matrix::multiply(x, y);
        return @Matrix::get(z, 0, 0);
    }
}

#[derive(Clone)]
struct Matrix {
    data: Vec<i32>,
    rows: i32,
    cols: i32,
}

#[allow(unused_mut, dead_code)]
#[lia_impl_glue]
impl Matrix {
    pub fn new(rows: i32, cols: i32) -> Matrix {
        let mut data = Vec::with_capacity((rows * cols) as usize);
        for _ in 0..(rows * cols) {
            data.push(0);
        }

        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn get(&self, row: i32, col: i32) -> i32 {
        self.data[(row * self.cols + col) as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, val: i32) {
        self.data[(row * self.cols + col) as usize] = val;
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);

        let mut new_mat = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut dot = 0;
                for k in 0..self.cols {
                    dot += self.data[(i * self.cols + k) as usize] *
                        other.data[(k * other.cols + j) as usize];
                }
                new_mat.data[(i * new_mat.cols + j) as usize] = dot;
            }
        }

        return new_mat;
    }
}

#[test]
fn matrix_test() {
    let result: LiaAny = call!(multiply_matrices());
    cast!(let num: i32 = result);
    assert!(num == 50);
}
