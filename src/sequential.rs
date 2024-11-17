pub fn find_lucky_number(matrix: &[Vec<i32>]) -> Vec<i32> {
    let number_of_rows = matrix.len();
    let number_of_columns = matrix[0].len();
    let mut minimas_in_rows: Vec<(usize, usize)> = (0..number_of_rows).map(|i| (i, 0)).collect();
    let mut maximas_in_columns: Vec<(usize, usize)> =
        (0..number_of_columns).map(|j| (0, j)).collect();

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell < matrix[minimas_in_rows[i].0][minimas_in_rows[i].1] {
                minimas_in_rows[i] = (i, j);
            }

            if *cell > matrix[maximas_in_columns[j].0][maximas_in_columns[j].1] {
                maximas_in_columns[j] = (i, j);
            }
        }
    }

    minimas_in_rows
        .iter()
        .filter_map(|&positon_min| {
            if positon_min == maximas_in_columns[positon_min.1] {
                Some(matrix[positon_min.0][positon_min.1])
            } else {
                None
            }
        })
        .collect()
}
