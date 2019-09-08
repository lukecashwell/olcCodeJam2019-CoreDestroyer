pub struct StarCore {
	pub x: f32, 
	pub y: f32,
	is_dead: bool,
	animation_frame: u32
}

impl StarCore {
	pub fn new(x: f32, y: f32) -> Self {
		Self{x: x, y: y, is_dead: false, animation_frame: 0}
	}
}

impl Entity for StarCore {
	fn update(&mut self) -> bool {
		if self.is_dead {
			self.animation_frame += 1;
		}
		if self.animation_frame == 30 {
				create_explosion((self.x) as i32, (self.y) as i32, 20, unsafe{g_entity_handler.as_mut().unwrap()});
		}
		if self.animation_frame == 70 {
			unsafe { g_stat_star_cores_destroyed += 1 };
		}
		return self.is_dead && self.animation_frame >= 70;
	}
	
	fn kill(&mut self) {
		self.is_dead = true;
	}
	
	fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		let image_container = unsafe {g_image_container.as_ref().unwrap()};
		if self.is_dead {
			if self.animation_frame < 70 {
				screen.draw_image(&(image_container.star_explosion[(self.animation_frame / 10) as usize]), 
				(self.x - camera.x) as i32 - 25,
				 (self.y - camera.y) as i32 - 25,
				 50, 50);
			}
		} else {
			screen.draw_image(&(image_container.star_core), 
			(self.x - camera.x) as i32 - 7,
			 (self.y - camera.y) as i32 - 7,
			 15, 15);
		}
	}
}