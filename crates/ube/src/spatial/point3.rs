#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Point3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point3 {
    pub fn distance(a: &Self, b: &Self) -> f64 {
        let xs = (a.x - b.x) * (a.x - b.x);
        let ys = (a.y - b.y) * (a.y - b.y);
        let zs = (a.z - b.z) * (a.z - b.z);
        ((xs + ys + zs) as f64).sqrt()
    }
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
