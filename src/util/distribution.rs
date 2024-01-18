use rand::{distributions::WeightedIndex, thread_rng};

/// Create a `Lazy<AnyDistribution::Weighted>` from pairs of values and weights.
///
/// Each tuple in the macro arguments represents a value and its weight.
/// The macro constructs the `WeightedDistribution` by separating these into
/// corresponding arrays of values and weights.
///
/// # Example
/// ```
/// weighted_distribution!(
///     i32,
///     (10, 1),
///     (20, 2),
///     (30, 3)
/// );
/// ```
#[macro_export]
macro_rules! weighted_distribution {
    ($type:ty, $(($value:expr, $weight:expr)),*) => {{
        Lazy::new(|| {
            let mut values: Vec<$type> = Vec::new();
            let mut weights: Vec<u32> = Vec::new();
            $(
                values.push($value);
                weights.push($weight);
            )*

            AnyDistribution::<$type>::new_weighted(values, weights)
        })
    }};
}

/// Creates a `Lazy<AnyDistribution::Singleton>` with a single specified value.
///
/// This macro takes a single value and constructs a `SingletonDistribution` instance.
/// The `SingletonDistribution` always returns this same value when `get_random` is called.
///
/// # Example
/// ```
/// singleton_distribution!(42);
/// ```
#[macro_export]
macro_rules! singleton_distribution {
    ($value:expr) => {{
        Lazy::new(|| AnyDistribution::new_singleton($value))
    }};
}

/// Trait representing a general distribution of values.
pub trait Distribution<T> {
    /// Returns a random value from the distribution.
    fn get_random(&self) -> &T;
}

/// A distribution where values have associated weights.
///
/// This struct represents a weighted distribution where each value has a
/// corresponding weight. Values with higher weights are more likely to be chosen.
#[derive(Clone)]
pub struct WeightedDistribution<T> {
    values: Vec<T>,
    dist: WeightedIndex<u32>,
}

/// A distribution with a single value.
///
/// This struct represents a distribution that always returns the same value.
#[derive(Clone)]
pub struct SingletonDistribution<T> {
    value: T,
}

pub enum AnyDistribution<T> {
    Weighted(WeightedDistribution<T>),
    Singleton(SingletonDistribution<T>),
}

impl<T> WeightedDistribution<T> {
    pub fn new(values: Vec<T>, weights: Vec<u32>) -> Self {
        match WeightedIndex::new(&weights) {
            Ok(dist) => Self { values, dist },
            Err(_) => {
                panic!(
                    "Unable to create WeightedDistribution, values len: {:?}, weights: {:?}",
                    values.len(),
                    weights
                )
            }
        }
    }
}

impl<T> SingletonDistribution<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Distribution<T> for WeightedDistribution<T> {
    fn get_random(&self) -> &T {
        let mut rng = thread_rng();
        &self.values[rand::prelude::Distribution::sample(&self.dist, &mut rng)]
    }
}

impl<T> Distribution<T> for SingletonDistribution<T> {
    fn get_random(&self) -> &T {
        &self.value
    }
}

impl<T> AnyDistribution<T> {
    pub fn new_weighted(values: Vec<T>, weights: Vec<u32>) -> Self {
        AnyDistribution::Weighted(WeightedDistribution::new(values, weights))
    }

    pub fn new_singleton(value: T) -> Self {
        AnyDistribution::Singleton(SingletonDistribution::new(value))
    }

    pub fn get_random(&self) -> &T {
        match self {
            AnyDistribution::Weighted(d) => d.get_random(),
            AnyDistribution::Singleton(d) => d.get_random(),
        }
    }
}
