pub mod vec2;
pub use vec2::Vec2;

trait Vector {
    fn len(&self) -> f64;
    fn normalized(&self) -> Self;
}

macro_rules! new {
    () => {

    }
}

struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector for Vector2 {
    fn len(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    fn normalized(&self) -> Self {
        todo!()
    }
}
