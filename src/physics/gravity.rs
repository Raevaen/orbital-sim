// gravity module
use crate::vec2::Vec2;
pub const G: f64 = 6.67430e-11; // gravitational constant

// mu is G times M (very big mass compared to the orbiting body)
pub fn gravity(mu: f64, position: &Vec2) -> Option<Vec2> {
    let r = position.magnitude();
    if !mu.is_finite() || mu <= 0.0 || !r.is_finite() || r <= 0.0 {
        return None;
    }
    let factor = -mu / (r * r * r);
    Some(position.mul_scalar(factor))
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn test_gravity() {
        let mu = 100.0;
        let position = Vec2 { x: 10.0, y: 0.0 };
        let result = gravity(mu, &position);
        assert!(result.is_some());
        let gravity_acc = result.unwrap();
        assert_eq!(gravity_acc.x, -1.0);
        assert_eq!(gravity_acc.y, 0.0);
    }
}
