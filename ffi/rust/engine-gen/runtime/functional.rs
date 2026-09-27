pub struct Functional;

impl Functional {
    pub fn for_each<T, F>(arr: &Vec<T>, mut f: F)
    where
        F: FnMut(&T),
    {
        for item in arr {
            f(item);
        }
    }

    pub fn sum_of_float<T, F>(arr: &Vec<T>, mut f: F) -> f64
    where
        F: FnMut(&T) -> f64,
    {
        let mut total = 0.0f64;
        for item in arr {
            total += f(item) as f64;
        }
        total as f64
    }
}
