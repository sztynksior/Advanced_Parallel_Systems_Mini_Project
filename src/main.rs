mod matrix;
mod parallel_join;
mod sequential;

use std::time::Instant;

fn main() {
    let matrix = matrix::Matrix::new_lucky_matrix(16, 24);

    // performence_check(&matrix);

    diam::svg("../lucky_number.svg", || {
        let _ = parallel_join::find_lucky_number(&matrix.cells);
    })
    .expect("failed saving svg file!");
}

fn performence_check(matrix: &matrix::Matrix) {
    let mut start = Instant::now();
    let mut lucky_numbers = sequential::find_lucky_number(&matrix.cells);
    let mut duration = start.elapsed();

    println!("Lucky numbers are: {lucky_numbers:?} Sequential time: {duration:?}");

    start = Instant::now();
    lucky_numbers = parallel_join::find_lucky_number(&matrix.cells);
    duration = start.elapsed();

    start = Instant::now();
    parallel_join::find_minimum(
        &matrix.cells,
        matrix.cells[0].len(),
        0,
        matrix.cells[0].len() - 1,
    );
    println!("Minimum search duration {:?}", start.elapsed());

    println!("Lucky numbers are: {lucky_numbers:?} Parallel time: {duration:?}");
}
