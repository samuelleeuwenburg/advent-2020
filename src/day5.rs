#![allow(dead_code)]

fn find_seat(lines: Vec<String>) -> Option<usize> {
    let mut seats: Vec<(usize, usize)> = vec![];

    for line in lines.iter() {
        let row_binary_str = &line[..7].replace("F", "0").replace("B", "1");
        let row = usize::from_str_radix(&row_binary_str, 2).unwrap();

        let col_binary_str = &line[7..10].replace("L", "0").replace("R", "1");
        let col = usize::from_str_radix(&col_binary_str, 2).unwrap();

        seats.push((row, col));
    }

    let mut seat_ids: Vec<usize> = seats.iter().map(|(r, c)| r * 8 + c).collect();
    seat_ids.sort();

    let mut prev_seat = 59;
    let mut missing_seat = None;

    for &seat in seat_ids.iter() {
        if seat < 60 { continue; }

        if seat - 1 != prev_seat {
            missing_seat = Some(seat - 1);
            break;
        }

        prev_seat = seat;
    }

    missing_seat
}

#[cfg(test)]
mod tests {
    use crate::input::{read_lines};
    use super::*;

    #[test]
    fn test_find_seat() {
        let lines = read_lines("input/day5.txt");

        assert_eq!(find_seat(lines), Some(594));
    }
}
