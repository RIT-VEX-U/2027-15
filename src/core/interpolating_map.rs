//! Interpolating map implementation.
//!
//! Provides a sorted map that linearly interpolates between adjacent key-value pairs
//! when querying keys that do not have exact matches.

use std::ops::{Add, Mul, Sub};

/// A key-value map that performs linear interpolation when an exact key match is not found.
///
/// Keys are stored in sorted order. When queried:
/// - If the map is empty, querying returns `None`.
/// - If the queried key is smaller than or equal to the minimum key, the minimum key's value is returned.
/// - If the queried key is larger than or equal to the maximum key, the maximum key's value is returned.
/// - Otherwise, the value is linearly interpolated between the immediate predecessor and successor:
///   $$\text{value} = \delta \cdot v_{\text{upper}} + (1.0 - \delta) \cdot v_{\text{lower}}$$
///   where $\delta = \frac{k - k_{\text{lower}}}{k_{\text{upper}} - k_{\text{lower}}}$.
#[derive(Debug, Clone)]
pub struct InterpolatingMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> Default for InterpolatingMap<K, V>
where
    K: PartialOrd + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> InterpolatingMap<K, V>
where
    K: PartialOrd + Copy,
{
    /// Creates an empty interpolating map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts a key-value pair into the map in sorted order.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(pos) = self
            .entries
            .iter()
            .position(|(k, _)| k.partial_cmp(&key) == Some(std::cmp::Ordering::Equal))
        {
            self.entries[pos].1 = value;
        } else {
            let pos = self
                .entries
                .iter()
                .position(|(k, _)| k.partial_cmp(&key) == Some(std::cmp::Ordering::Greater))
                .unwrap_or(self.entries.len());
            self.entries.insert(pos, (key, value));
        }
    }

    /// Clears all entries from the map.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl InterpolatingMap<f64, f64> {
    /// Queries the value at the given key with linear interpolation.
    ///
    /// # Returns
    /// - `None` if the map is empty.
    /// - The interpolated or clamped `f64` value otherwise.
    pub fn get(&self, key: f64) -> Option<f64> {
        if self.entries.is_empty() {
            return None;
        }

        if key <= self.entries[0].0 {
            return Some(self.entries[0].1);
        }
        let last = self.entries.len() - 1;
        if key >= self.entries[last].0 {
            return Some(self.entries[last].1);
        }

        for i in 0..last {
            let (k_low, v_low) = self.entries[i];
            let (k_up, v_up) = self.entries[i + 1];

            if (k_low - key).abs() < 1e-12 {
                return Some(v_low);
            }
            if (k_up - key).abs() < 1e-12 {
                return Some(v_up);
            }

            if key >= k_low && key <= k_up {
                let range = k_up - k_low;
                if range.abs() < 1e-12 {
                    return Some(v_low);
                }
                let delta = (key - k_low) / range;
                return Some(delta * v_up + (1.0 - delta) * v_low);
            }
        }

        Some(self.entries[last].1)
    }
}

/// Generic linear interpolation helper for any types implementing arithmetic operations.
impl<K, V> InterpolatingMap<K, V>
where
    K: Copy + PartialOrd + Sub<Output = K> + Into<f64>,
    V: Copy + Add<Output = V> + Mul<f64, Output = V>,
{
    /// Evaluates the map at the given key using linear interpolation for generic types.
    pub fn interpolate(&self, key: K) -> Option<V> {
        if self.entries.is_empty() {
            return None;
        }

        if key <= self.entries[0].0 {
            return Some(self.entries[0].1);
        }
        let last = self.entries.len() - 1;
        if key >= self.entries[last].0 {
            return Some(self.entries[last].1);
        }

        for i in 0..last {
            let (k_low, v_low) = self.entries[i];
            let (k_up, v_up) = self.entries[i + 1];

            if key >= k_low && key <= k_up {
                let k_low_f: f64 = k_low.into();
                let k_up_f: f64 = k_up.into();
                let key_f: f64 = key.into();
                let delta = (key_f - k_low_f) / (k_up_f - k_low_f);
                return Some(v_up * delta + v_low * (1.0 - delta));
            }
        }

        Some(self.entries[last].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolating_map() {
        let mut map = InterpolatingMap::new();
        map.insert(0.0, 0.0);
        map.insert(10.0, 100.0);
        map.insert(20.0, 300.0);

        // Exact hits
        assert_eq!(map.get(0.0), Some(0.0));
        assert_eq!(map.get(10.0), Some(100.0));
        assert_eq!(map.get(20.0), Some(300.0));

        // Clamping bounds
        assert_eq!(map.get(-5.0), Some(0.0));
        assert_eq!(map.get(25.0), Some(300.0));

        // Midpoint interpolation
        assert!((map.get(5.0).unwrap() - 50.0).abs() < 1e-6);
        assert!((map.get(15.0).unwrap() - 200.0).abs() < 1e-6);
    }
}
