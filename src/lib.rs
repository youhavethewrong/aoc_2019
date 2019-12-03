pub fn fuel_requirements(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

#[cfg(test)]
mod tests {
    use super::fuel_requirements;

    #[test]
    fn it_works() {
        assert_eq!(2.0, fuel_requirements(12.0));
        assert_eq!(2.0, fuel_requirements(14.0));
        assert_eq!(654.0, fuel_requirements(1969.0));
        assert_eq!(33583.0, fuel_requirements(100756.0));
    }
}
