#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 { x, y, z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        };
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        };
    }
}

impl std::ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        return Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn test_new() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(a, b);
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(3, 3, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(4,5,6), a + b);

    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(3, 3, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(2,1,0), a - b);
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(3, 3, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(3,6,9), a * b);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(3,6,9), a * 3);
    }
}
