pub(crate) fn circular_speed(mu: f64, radius: f64) -> Option<f64> {
    if !mu.is_finite() || mu <= 0.0 || !radius.is_finite() || radius <= 0.0 {
        return None;
    }

    Some((mu / radius).sqrt())
}

#[cfg(test)] // compile module only for tests
mod tests {
    use super::*; // use the parent module names in here too

    #[test] // valid input
    fn calculates_circular_speed() {
        let speed = circular_speed(90_000.0, 160.0);
        assert_eq!(speed, Some(23.717082451262844)); // rounding errors may occur?
    }

    #[test] // zero radius should be rejected
    fn rejects_zero_radius() {
        assert_eq!(circular_speed(90_000.0, 0.0), None);
    }

    #[test] // negative radius should be rejected
    fn rejects_negative_radius() {
        assert_eq!(circular_speed(90_000.0, -10.0), None);
    }
}
