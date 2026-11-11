use crate::runtime::functional::Functional;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct AccurateSum;

impl AccurateSum {
    pub fn accurate_sum_of(values: &Vec<f64>) -> f64 {
        let accumulate: Arc<dyn Fn() -> f64 + Send + Sync + 'static> = { let values = (values).to_vec(); Arc::new(move || {
        return Functional::sum_of_float(&values, move |value: &f64| {
        return *value;
});
}) };
        return accumulate();
    }
}
