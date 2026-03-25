#[derive(Clone)]
pub struct Vector3 {
    pub x : f64, 
    pub y : f64,
    pub z : f64
}
impl Vector3 {
    pub fn new (x : f64, y : f64, z : f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        return self.clone().div(length);
    }

    pub fn div(self, rhs: f64) -> Self {
        Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
    pub fn mul(self, rhs: f64) -> Self {
        Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }

    pub fn distanse(&self, other : &Self) -> f64{
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    fn rotate_x(&self, angle : f64) -> Self {
        let np: (f64, f64) = rotate_2d(self.y, self.z, angle);
        Self { x: self.x, y: np.0, z: np.1 }
    }
    fn rotate_y(&self, angle : f64) -> Self {
        let np: (f64, f64) = rotate_2d(self.x, self.z, angle);
        Self { x: np.0, y: self.y, z: np.1 }
    }
    fn rotate_z(&self, angle : f64) -> Self {
        let np: (f64, f64) = rotate_2d(self.x, self.y, angle);
        Self { x: np.0, y: np.1, z: self.z }
    }

    pub fn rotate(&self, angle : &Vector3) -> Self {
        self.clone().rotate_x(angle.x).rotate_y(angle.y).rotate_z(angle.z)
    }
}
impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

fn rotate_2d(x : f64, y : f64, angle : f64) -> (f64, f64) {
    (x*angle.cos()-y*angle.sin(), x*angle.sin()+y*angle.cos())
}