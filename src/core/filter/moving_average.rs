//! Sensor signal filtering: standard circular MovingAverage and ExponentialMovingAverage.

/// Common interface for data smoothing and noise filters.
pub trait Filter: Send {
    /// Supplies a new sample to the filter.
    fn add_entry(&mut self, n: f64);
    /// Retrieves the current filtered value.
    fn get_value(&self) -> f64;
}

/// A standard circular moving average filter.
#[derive(Debug, Clone)]
pub struct MovingAverage {
    buffer: Vec<f64>,
    buffer_index: usize,
    current_avg: f64,
}

impl MovingAverage {
    /// Creates a moving average with 0.0 as default starting value.
    pub fn new(buffer_size: usize) -> Self {
        Self::with_starting_value(buffer_size, 0.0)
    }

    /// Creates a moving average with a specified default starting value.
    pub fn with_starting_value(buffer_size: usize, starting_value: f64) -> Self {
        let size = buffer_size.max(1);
        Self {
            buffer: vec![starting_value; size],
            buffer_index: 0,
            current_avg: starting_value,
        }
    }

    /// Returns the number of samples in the filter buffer.
    pub fn get_size(&self) -> usize {
        self.buffer.len()
    }
}

impl Filter for MovingAverage {
    fn add_entry(&mut self, n: f64) {
        let size = self.get_size() as f64;
        self.current_avg -= self.buffer[self.buffer_index] / size;
        self.current_avg += n / size;
        self.buffer[self.buffer_index] = n;

        self.buffer_index = (self.buffer_index + 1) % self.get_size();
    }

    fn get_value(&self) -> f64 {
        self.current_avg
    }
}

/// Exponential Moving Average filter weighting newer samples higher.
#[derive(Debug, Clone)]
pub struct ExponentialMovingAverage {
    buffer: Vec<f64>,
    buffer_index: usize,
    current_avg: f64,
}

impl ExponentialMovingAverage {
    /// Creates an exponential moving average with 0.0 as default starting value.
    pub fn new(buffer_size: usize) -> Self {
        Self::with_starting_value(buffer_size, 0.0)
    }

    /// Creates an exponential moving average with a specified default starting value.
    pub fn with_starting_value(buffer_size: usize, starting_value: f64) -> Self {
        let size = buffer_size.max(1);
        Self {
            buffer: vec![starting_value; size],
            buffer_index: 0,
            current_avg: starting_value,
        }
    }

    /// Returns the number of samples in the filter buffer.
    pub fn get_size(&self) -> usize {
        self.buffer.len()
    }
}

impl Filter for ExponentialMovingAverage {
    fn add_entry(&mut self, n: f64) {
        let size = self.get_size();
        let oldest_index = self.buffer_index;
        let oldest_sample = self.buffer[oldest_index];
        let oldest_weight = 1.0 / (2.0f64).powf((size - 1) as f64);

        let second_oldest_sample = self.buffer[(oldest_index + 1) % size];

        self.current_avg -= oldest_sample * oldest_weight;
        self.current_avg /= 2.0;
        self.current_avg += second_oldest_sample / (2.0f64).powf(size as f64);
        self.current_avg += n / 2.0;

        self.buffer[self.buffer_index] = n;

        self.buffer_index = (self.buffer_index + 1) % size;
    }

    fn get_value(&self) -> f64 {
        self.current_avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let mut ma = MovingAverage::new(4);
        ma.add_entry(4.0);
        ma.add_entry(4.0);
        ma.add_entry(4.0);
        ma.add_entry(4.0);
        assert_eq!(ma.get_value(), 4.0);

        ma.add_entry(8.0);
        assert_eq!(ma.get_value(), 5.0);
    }

    #[test]
    fn test_exponential_moving_average() {
        let mut ema = ExponentialMovingAverage::new(3);
        ema.add_entry(10.0);
        ema.add_entry(10.0);
        ema.add_entry(10.0);
        assert!((ema.get_value() - 10.0).abs() < 1.0);
    }
}
