pub struct Core {
	pub x: f32, 
	pub y: f32,
	is_dead: bool,
	animation_frame: u32
}

impl Core {
	pub fn new(x: f32, y: f32) -> Self {
		Self{x: x, y: y, is_dead: false, animation_frame: 0}
	}
}

impl Entity for Core {
	fn update(&mut self) -> bool {
		if self.is_dead {
			self.animation_frame += 1;
		}
		if self.animation_frame == 40 {
				create_explosion((self.x) as i32, (self.y) as i32, 50, unsafe{g_entity_handler.as_mut().unwrap()});
		}
		if self.animation_frame == 150 {
			unsafe {
				g_stat_cores_destroyed += 1;
			}
		}
		return self.is_dead && self.animation_frame >= 150;
	}
	
	fn kill(&mut self) {
		self.is_dead = true;
	}
	
	fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		let image_container = unsafe {g_image_container.as_ref().unwrap()};
		if self.is_dead {
			if self.animation_frame < 70 {
				screen.draw_image(&(image_container.core_explosion[(self.animation_frame / 10) as usize]), 
				(self.x - camera.x) as i32 - 50,
				 (self.y - camera.y) as i32 - 50,
				 100, 100);
			}
		} else {
			screen.draw_image(&(image_container.core), 
			(self.x - camera.x) as i32 - 10,
			 (self.y - camera.y) as i32 - 10,
			 20, 20);
		}
	}
}