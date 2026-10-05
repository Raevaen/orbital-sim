// Vector2D
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    // scalar multiplication
    pub fn mul_scalar(&self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    // vector addition
    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    // magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[cfg(test)] // compile module only for tests
mod tests {
    use super::*;

    #[test]
    fn test_mul_scalar() {
        let v = Vec2 { x: 2.0, y: 3.0 };
        let result = v.mul_scalar(2.0);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec2 { x: 1.0, y: 2.0 };
        let v2 = Vec2 { x: 3.0, y: 4.0 };
        let result = v1.add(&v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vec2 { x: 3.0, y: 4.0 };
        let result = v.magnitude();
        assert_eq!(result, 5.0);
    }
}
