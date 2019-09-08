pub struct Camera {
	pub x: f32,
	pub y: f32
}

impl Camera {
	pub fn new() -> Self {
	 	Self{x: 0.0, y: -40.0}
	}
}