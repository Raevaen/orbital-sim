fn main() {
    println!("Orbital simulation starting...");

    let speed = circular_speed(90_000.0, 0.0);
    match speed {
        Some(s) => println!("Circular orbital speed: {}", s),
        None => println!("Invalid input for circular speed calculation"),
    }
}

fn circular_speed(mu: f64, radius: f64) -> Option<f64> {
    if !mu.is_finite() || mu <= 0.0 || !radius.is_finite() || radius <= 0.0 {
        return None;
    }

    Some((mu / radius).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_circular_speed() {
        let speed = circular_speed(90_000.0, 160.0);

        assert_eq!(speed, Some(23.717082451262844));
    }

    #[test]
    fn rejects_zero_radius() {
        assert_eq!(circular_speed(90_000.0, 0.0), None);
    }

    #[test]
    fn rejects_negative_radius() {
        assert_eq!(circular_speed(90_000.0, -10.0), None);
    }
}
