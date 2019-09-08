pub struct Plane {
	x: f32,
	y: f32,
	is_dead: bool
}

impl Plane {
	pub fn new(x: f32, y: f32) -> Self {
		Self{x: x, y: y, is_dead: false}
	}
}

impl Entity for Plane {
	fn update(&mut self) -> bool {

		return self.is_dead;
	}
	
	fn kill(&mut self) {
		self.is_dead = true;
	}
	
	fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		screen.draw_image(&(unsafe{g_image_container.as_ref().unwrap()}.plane), self.x as i32, self.y as i32, 30, 10);
	}
}