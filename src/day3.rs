#![allow(dead_code)]
use std::str::FromStr;
use std::convert::TryFrom;
use std::convert::TryInto;

enum Object {
    Empty,
    Tree
}

struct Row { objects: Vec<Object> }

struct Map { rows: Vec<Row> }

struct Position { x: usize, y: usize }

impl TryFrom<char> for Object {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Object::Empty),
            '#' => Ok(Object::Tree),
            _ => Err("invalid object char".to_string()),
        }
    }
}

impl FromStr for Row {
    type Err = String;

    fn from_str(input: &str) -> Result<Row, String> {
        let mut objects = vec![];

        for c in input.chars() {
            let obj: Object = c.try_into()?;
            objects.push(obj);
        }

        Ok(Row { objects })
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = String;

    fn try_from(input: Vec<String>) -> Result<Map, String> {
        let mut rows: Vec<Row> = vec![];

        for line in input.iter() {
            let row = line.parse()?;
            rows.push(row);
        }

        Ok(Map { rows })
    }
}

impl Map {
    fn get_object_at_pos(&self, pos: &Position) -> Option<&Object> {
        let row = self.rows.get(pos.y)?;
        let row_length = row.objects.len();
        row.objects.get(pos.x % row_length)
    }

    fn number_of_trees_for_movement(&self, movement: &Position) -> usize {
        let mut pos = Position { x: 0, y: 0 };
        let mut num_trees = 0;

        loop {
            match self.get_object_at_pos(&pos) {
                Some(Object::Tree) => num_trees += 1,
                None => break,
                _ => (),
            }

            pos = Position {
                x: pos.x + movement.x,
                y: pos.y + movement.y
            };
        }

        num_trees
    }
}


fn part_one(input: Vec<String>) -> Result<usize, String> {
    let map: Map = input.try_into()?;
    let movement = Position { x: 3, y: 1 };
    let num_trees = map.number_of_trees_for_movement(&movement);
    Ok(num_trees)
}

fn part_two(input: Vec<String>) -> Result<usize, String> {
    let map: Map = input.try_into()?;

    let movements: Vec<Position> = vec![
        Position { x: 1, y: 1 },
        Position { x: 3, y: 1 },
        Position { x: 5, y: 1 },
        Position { x: 7, y: 1 },
        Position { x: 1, y: 2 },
    ];

    Ok(
        movements
            .iter()
            .map(|m| map.number_of_trees_for_movement(m))
            .fold(1, |sum, trees| sum * trees)
    )
}

#[cfg(test)]
mod tests {
    use crate::input::{read_lines};
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_lines("input/day3.txt");
        assert_eq!(part_one(input), Ok(299));
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("input/day3.txt");
        assert_eq!(part_two(input), Ok(3621285278));
    }
}
