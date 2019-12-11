pub fn fuel_requirements(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

pub fn fuel_for_fuel(mass: f64) -> f64 {
    let mut sum = 0.0;
    let initial = fuel_requirements(mass);
    sum += initial;
    let mut n = fuel_requirements(initial);
    while n > 0.0 {
        sum += n;
        n = fuel_requirements(n);
    }

    sum
}

pub fn parse_intcode_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

pub fn intcode_program(codes: Vec<usize>) -> Vec<usize> {
    let mut pos = 0;
    let mut program_results = codes.clone();
    while program_results[pos] != 99 {
        let current = &program_results[pos..pos + 3];
        match current[0] {
            1 => {
                let result_location = program_results[pos + 3];
                let first_term_location = program_results[pos + 1];
                let second_term_location = program_results[pos + 2];
                program_results[result_location] =
                    program_results[first_term_location] + program_results[second_term_location];
            }
            2 => {
                let result_location = program_results[pos + 3];
                let first_factor_location = program_results[pos + 1];
                let second_factor_location = program_results[pos + 2];
                program_results[result_location] = program_results[first_factor_location]
                    * program_results[second_factor_location];
            }
            _ => (),
        }
        pos += 4
    }

    program_results
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Vector {
    U { m: usize },
    D { m: usize },
    L { m: usize },
    R { m: usize },
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub fn parse_vector(input: String) -> Vector {
    let d = &input[..1];
    let m = &input[1..];
    match d {
        "U" => Vector::U {
            m: usize::from_str_radix(m, 10).unwrap(),
        },
        "D" => Vector::D {
            m: usize::from_str_radix(m, 10).unwrap(),
        },
        "L" => Vector::L {
            m: usize::from_str_radix(m, 10).unwrap(),
        },
        "R" => Vector::R {
            m: usize::from_str_radix(m, 10).unwrap(),
        },
        _ => panic!(),
    }
}

pub fn points_visited(origin: Point, vector: Vector) -> Vec<Point> {
    match vector {
        Vector::D { m } => (origin.y - m..origin.y)
            .map(|i| Point { x: origin.x, y: i })
            .collect(),
        Vector::U { m } => (origin.y + 1..origin.y + m + 1)
            .map(|i| Point { x: origin.x, y: i })
            .collect(),
        Vector::L { m } => (origin.x - m..origin.x)
            .map(|i| Point { x: i, y: origin.y })
            .collect(),
        Vector::R { m } => (origin.x + 1..origin.x + m + 1)
            .map(|i| Point { x: i, y: origin.y })
            .collect(),
    }
}

pub fn generate_points(origin: Point, vector_sequence: Vec<Vector>) -> Vec<Point> {
    let points: Vec<Point> = vec![origin];
    let visited = vector_sequence.iter().fold(points, |mut acc, v| {
        let last_visited = acc.last().unwrap().clone();
        let mut visited = points_visited(last_visited, *v);
        acc.append(&mut visited);
        acc
    });
    visited[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2.0, fuel_requirements(12.0));
        assert_eq!(2.0, fuel_requirements(14.0));
        assert_eq!(654.0, fuel_requirements(1969.0));
        assert_eq!(33583.0, fuel_requirements(100756.0));
    }

    #[test]
    fn how_much_fuel_for_fuel() {
        assert_eq!(966.0, fuel_for_fuel(1969.0));
        assert_eq!(50346.0, fuel_for_fuel(100756.0));
    }

    #[test]
    fn test_intcode_program() {
        let input_1: Vec<usize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected_1: Vec<usize> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(expected_1, intcode_program(input_1));

        let input_2: Vec<usize> = vec![1, 0, 0, 0, 99];
        let expected_2: Vec<usize> = vec![2, 0, 0, 0, 99];
        assert_eq!(expected_2, intcode_program(input_2));

        let input_3: Vec<usize> = vec![2, 3, 0, 3, 99];
        let expected_3: Vec<usize> = vec![2, 3, 0, 6, 99];
        assert_eq!(expected_3, intcode_program(input_3));

        let input_4: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        let expected_4: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(expected_4, intcode_program(input_4));

        let input_5: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected_5: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(expected_5, intcode_program(input_5));
    }

    #[test]
    fn test_parse_input() {
        let input = "0,1,2,4".to_string();
        let expected: Vec<usize> = vec![0, 1, 2, 4];
        assert_eq!(expected, parse_intcode_input(input));
    }

    #[test]
    fn test_vector_parse() {
        let input = "D999".to_string();
        let expected = Vector::D { m: 999 };
        assert_eq!(expected, parse_vector(input));
    }

    #[test]
    fn test_points_visited() {
        let origin_1 = Point { x: 0, y: 0 };
        let vector_1 = Vector::R { m: 2 };
        let expected_1 = vec![Point { x: 1, y: 0 }, Point { x: 2, y: 0 }];
        assert_eq!(expected_1, points_visited(origin_1, vector_1));

        let origin_2 = Point { x: 2, y: 2 };
        let vector_2 = Vector::L { m: 2 };
        let expected_2 = vec![Point { x: 0, y: 2 }, Point { x: 1, y: 2 }];
        assert_eq!(expected_2, points_visited(origin_2, vector_2));

        let origin_3 = Point { x: 1, y: 1 };
        let vector_3 = Vector::U { m: 2 };
        let expected_3 = vec![Point { x: 1, y: 2 }, Point { x: 1, y: 3 }];
        assert_eq!(expected_3, points_visited(origin_3, vector_3));

        let origin_4 = Point { x: 1, y: 4 };
        let vector_4 = Vector::D { m: 2 };
        let expected_4 = vec![Point { x: 1, y: 2 }, Point { x: 1, y: 3 }];
        assert_eq!(expected_4, points_visited(origin_4, vector_4));
    }

    #[test]
    fn test_generate_points() {
        let origin = Point { x: 1, y: 1 };
        let sequence = vec![
            Vector::U { m: 2 },
            Vector::R { m: 2 },
            Vector::D { m: 1 },
            Vector::L { m: 1 },
        ];
        let expected = vec![
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
            Point { x: 2, y: 3 },
            Point { x: 3, y: 3 },
            Point { x: 3, y: 2 },
            Point { x: 2, y: 2 },
        ];
        assert_eq!(expected, generate_points(origin, sequence));
    }
}
