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
}
