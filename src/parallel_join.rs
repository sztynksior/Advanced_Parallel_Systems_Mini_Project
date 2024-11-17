use rayon::join;
use std::time::Instant;

pub fn find_lucky_number(matrix: &[Vec<i32>]) -> Vec<i32> {
    let number_of_rows = matrix.len();
    let number_of_columns = matrix[0].len();
    let mut rows_min_positions: Vec<(usize, usize)> = vec![(0, 0); number_of_rows];
    let mut cols_max_positions: Vec<(usize, usize)> = vec![(0, 0); number_of_columns];

    find_minimas_in_rows(matrix, &mut rows_min_positions, &(number_of_rows - 1));
    find_maximas_in_cols(matrix, &mut cols_max_positions, number_of_columns - 1);

    let lucky_number = search_for_lucky_number(&rows_min_positions, &cols_max_positions);
    match lucky_number {
        Some(position) => vec![matrix[position.0][position.1]],
        None => Vec::new(),
    }
}

fn find_minimas_in_rows(
    matrix: &[Vec<i32>],
    rows_min_positions: &mut [(usize, usize)],
    current_row: &usize,
) {
    if rows_min_positions.len() == 1 {
        rows_min_positions[0] = (
            *current_row,
            find_minimum(
                matrix,
                matrix[*current_row].len(),
                *current_row,
                matrix[*current_row].len() - 1,
            ),
        );
    } else {
        let (rows_min_positions_left, rows_min_positions_right) =
            rows_min_positions.split_at_mut(rows_min_positions.len() / 2);
        let left_split_last_row_number = current_row - rows_min_positions_right.len();
        diam::join(
            || find_minimas_in_rows(matrix, rows_min_positions_left, &left_split_last_row_number),
            || find_minimas_in_rows(matrix, rows_min_positions_right, current_row),
        );
    }
}

pub fn find_minimum(
    matrix: &[Vec<i32>],
    split_lenght: usize,
    min_row: usize,
    min_column: usize,
) -> usize {
    if split_lenght == 1 {
        min_column
    } else if split_lenght == 2 {
        if matrix[min_row][min_column - 1] < matrix[min_row][min_column] {
            min_column - 1
        } else {
            min_column
        }
    } else {
        let left_split_length = split_lenght / 2;
        let right_split_length = split_lenght - left_split_length;
        let left_split_min_position = min_column - right_split_length;

        let (min_position_left, min_position_right) = diam::join(
            || find_minimum(matrix, left_split_length, min_row, left_split_min_position),
            || find_minimum(matrix, right_split_length, min_row, min_column),
        );

        if matrix[min_row][min_position_left] < matrix[min_row][min_position_right] {
            min_position_left
        } else {
            min_position_right
        }
    }
}

fn find_maximas_in_cols(
    matrix: &[Vec<i32>],
    cols_max_positions: &mut [(usize, usize)],
    current_col: usize,
) {
    if cols_max_positions.len() == 1 {
        cols_max_positions[0] = (
            find_maximum(matrix, matrix.len(), matrix.len() - 1, current_col),
            current_col,
        );
    } else {
        let (cols_max_positions_top, cols_max_positions_bottom) =
            cols_max_positions.split_at_mut(cols_max_positions.len() / 2);
        let left_split_last_col_number = current_col - cols_max_positions_bottom.len();
        diam::join(
            || find_maximas_in_cols(matrix, cols_max_positions_top, left_split_last_col_number),
            || find_maximas_in_cols(matrix, cols_max_positions_bottom, current_col),
        );
    }
}

fn find_maximum(
    matrix: &[Vec<i32>],
    split_length: usize,
    max_row: usize,
    max_column: usize,
) -> usize {
    if split_length == 1 {
        max_row
    } else if split_length == 2 {
        if matrix[max_row - 1][max_column] > matrix[max_row][max_column] {
            max_row - 1
        } else {
            max_row
        }
    } else {
        let top_split_length = split_length / 2;
        let bottom_split_length = split_length - top_split_length;
        let top_split_max_position = max_row - bottom_split_length;

        let (max_position_top, max_position_bottom) = diam::join(
            || find_maximum(matrix, top_split_length, top_split_max_position, max_column),
            || find_maximum(matrix, bottom_split_length, max_row, max_column),
        );

        if matrix[max_position_top][max_column] > matrix[max_position_bottom][max_column] {
            max_position_top
        } else {
            max_position_bottom
        }
    }
}

fn search_for_lucky_number(
    rows_min_positions: &[(usize, usize)],
    cols_max_positions: &[(usize, usize)],
) -> Option<(usize, usize)> {
    if rows_min_positions.len() == 1 {
        if rows_min_positions[0] == cols_max_positions[rows_min_positions[0].1] {
            Some(rows_min_positions[0])
        } else {
            None
        }
    } else {
        let (rows_min_positions_left, rows_min_positions_right) =
            rows_min_positions.split_at(rows_min_positions.len() / 2);

        let (left_search_result, right_search_result) = diam::join(
            || search_for_lucky_number(rows_min_positions_left, cols_max_positions),
            || search_for_lucky_number(rows_min_positions_right, cols_max_positions),
        );

        match (left_search_result, right_search_result) {
            (Some(position), None) => Some(position),
            (None, Some(position)) => Some(position),
            _ => None,
        }
    }
}
