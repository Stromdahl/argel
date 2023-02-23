use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(self, other: Vec3) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        return Vec3 { x, y, z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return self.add(rhs);
    }
}
