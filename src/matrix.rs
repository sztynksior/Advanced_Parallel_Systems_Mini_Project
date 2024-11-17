pub struct Matrix {
    pub cells: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new_lucky_matrix(m: usize, n: usize) -> Matrix {
        Matrix {
            cells: Self::generate_matrix(m, n),
        }
    }

    fn generate_matrix(m: usize, n: usize) -> Vec<Vec<i32>> {
        vec![vec![0; n]; m]
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .zip(i * n + 1..i * n + n + 1)
                    .map(|(_, j)| j as i32)
                    .collect()
            })
            .collect()
    }

    pub fn print(&self) {
        self.cells.iter().for_each(|row| {
            row.iter().for_each(|cell| print!("{}", cell));
            println!();
        });
    }
}
