//! Numerical ODE integration methods (Euler, Explicit Midpoint RK2, Classic Runge-Kutta RK4).

use nalgebra::SVector;

/// First order Euler integration for `dx/dt = f(x, u)`.
pub fn euler_with_input<const X: usize, const U: usize, F>(
    f: &F,
    x: &SVector<f64, X>,
    u: &SVector<f64, U>,
    h: f64,
) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>, &SVector<f64, U>) -> SVector<f64, X>,
{
    let k1 = f(x, u);
    x + k1 * h
}

/// First order Euler integration for `dx/dt = f(x)`.
pub fn euler_without_input<const X: usize, F>(f: &F, x: &SVector<f64, X>, h: f64) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>) -> SVector<f64, X>,
{
    let k1 = f(x);
    x + k1 * h
}

/// First order Euler integration for time-variant `dy/dt = f(t, y)`.
pub fn euler_time_variant<const Y: usize, F>(
    f: &F,
    t: f64,
    y: &SVector<f64, Y>,
    h: f64,
) -> SVector<f64, Y>
where
    F: Fn(f64, &SVector<f64, Y>) -> SVector<f64, Y>,
{
    let k1 = f(t, y);
    y + k1 * h
}

/// Second order explicit midpoint Runge-Kutta (RK2) for `dx/dt = f(x, u)`.
pub fn rk2_with_input<const X: usize, const U: usize, F>(
    f: &F,
    x: &SVector<f64, X>,
    u: &SVector<f64, U>,
    h: f64,
) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>, &SVector<f64, U>) -> SVector<f64, X>,
{
    let k1 = f(x, u);
    let x_mid = x + k1 * (0.5 * h);
    let k2 = f(&x_mid, u);
    x + k2 * h
}

/// Second order explicit midpoint Runge-Kutta (RK2) for `dx/dt = f(x)`.
pub fn rk2_without_input<const X: usize, F>(f: &F, x: &SVector<f64, X>, h: f64) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>) -> SVector<f64, X>,
{
    let k1 = f(x);
    let x_mid = x + k1 * (0.5 * h);
    let k2 = f(&x_mid);
    x + k2 * h
}

/// Second order explicit midpoint Runge-Kutta (RK2) for time-variant `dy/dt = f(t, y)`.
pub fn rk2_time_variant<const Y: usize, F>(
    f: &F,
    t: f64,
    y: &SVector<f64, Y>,
    h: f64,
) -> SVector<f64, Y>
where
    F: Fn(f64, &SVector<f64, Y>) -> SVector<f64, Y>,
{
    let k1 = f(t, y);
    let y_mid = y + k1 * (0.5 * h);
    let k2 = f(t + 0.5 * h, &y_mid);
    y + k2 * h
}

/// Classic 4th-order Runge-Kutta (RK4) for `dx/dt = f(x, u)`.
pub fn rk4_with_input<const X: usize, const U: usize, F>(
    f: &F,
    x: &SVector<f64, X>,
    u: &SVector<f64, U>,
    h: f64,
) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>, &SVector<f64, U>) -> SVector<f64, X>,
{
    let k1 = f(x, u);
    let x_k2 = x + k1 * (0.5 * h);
    let k2 = f(&x_k2, u);
    let x_k3 = x + k2 * (0.5 * h);
    let k3 = f(&x_k3, u);
    let x_k4 = x + k3 * h;
    let k4 = f(&x_k4, u);

    x + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
}

/// Classic 4th-order Runge-Kutta (RK4) for `dx/dt = f(x)`.
pub fn rk4_without_input<const X: usize, F>(f: &F, x: &SVector<f64, X>, h: f64) -> SVector<f64, X>
where
    F: Fn(&SVector<f64, X>) -> SVector<f64, X>,
{
    let k1 = f(x);
    let x_k2 = x + k1 * (0.5 * h);
    let k2 = f(&x_k2);
    let x_k3 = x + k2 * (0.5 * h);
    let k3 = f(&x_k3);
    let x_k4 = x + k3 * h;
    let k4 = f(&x_k4);

    x + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
}

/// Classic 4th-order Runge-Kutta (RK4) for time-variant `dy/dt = f(t, y)`.
pub fn rk4_time_variant<const Y: usize, F>(
    f: &F,
    t: f64,
    y: &SVector<f64, Y>,
    h: f64,
) -> SVector<f64, Y>
where
    F: Fn(f64, &SVector<f64, Y>) -> SVector<f64, Y>,
{
    let k1 = f(t, y);
    let y_k2 = y + k1 * (0.5 * h);
    let k2 = f(t + 0.5 * h, &y_k2);
    let y_k3 = y + k2 * (0.5 * h);
    let k3 = f(t + 0.5 * h, &y_k3);
    let y_k4 = y + k3 * h;
    let k4 = f(t + h, &y_k4);

    y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_integration() {
        // dx/dt = -x => analytical solution x(t) = x0 * exp(-t)
        let f = |x: &SVector<f64, 1>| -*x;
        let x0 = SVector::<f64, 1>::new(1.0);
        let h = 0.1;
        let x_rk4 = rk4_without_input(&f, &x0, h);

        let analytical = (-h).exp();
        assert!((x_rk4[0] - analytical).abs() < 1e-5);
    }
}
