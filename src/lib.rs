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
}
