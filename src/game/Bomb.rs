pub struct Bomb {
	x: f32,
	y: f32,
	xv: f32,
	yv: f32,
	life_time: u32,
	is_dead: bool,
	animation_frame: u32
}

impl Bomb {
	pub fn new(x: f32, y: f32, xv: f32, yv: f32) -> Self {
		Self{x: x, y: y, xv: xv, yv: yv, life_time: 1000, is_dead: false, animation_frame: 0}
	}
}

impl Entity for Bomb {
	fn update(&mut self) -> bool {
		let entity_handler = unsafe {g_entity_handler.as_mut().unwrap()};
		if self.is_dead {
			self.animation_frame += 1;
		} else {
			let map = unsafe {g_map.as_ref().unwrap()};
			self.x += self.xv;
		
			//X collisions
			if self.xv > 0.0 {
				let color1 = (map.get_pixel((self.x + 4.5) as i32, (self.y - 5.0)as i32) & 0xff000000) >> 24;
				let color2 = (map.get_pixel((self.x + 4.5) as i32, (self.y + 5.0) as i32) & 0xff000000) >> 24;
				if color1 != 0 || color2 != 0 {
					self.x -= self.xv;
					self.xv *= -0.5;
				} 
			} else 
			if self.xv < 0.0 {
				let color1 = (map.get_pixel((self.x - 5.0) as i32, (self.y - 5.0) as i32) & 0xff000000) >> 24;
				let color2 = (map.get_pixel((self.x - 5.0) as i32, (self.y + 4.5) as i32) & 0xff000000) >> 24;
				if color1 != 0 || color2 != 0 {
					self.x -= self.xv;
					self.xv *= -0.5;
				}
			}
			self.y += self.yv;
			
			//Y collisions
			if self.yv > 0.0 {
				let color1 = (map.get_pixel((self.x - 5.0) as i32, (self.y + 4.5)as i32) & 0xff000000) >> 24;
				let color2 = (map.get_pixel((self.x + 5.0) as i32, (self.y + 4.5) as i32) & 0xff000000) >> 24;
				if color1 != 0 || color2 != 0 {
					self.kill();
					self.y -= self.yv;
					self.yv *= -0.5;
				}
			} else 
			if self.yv < 0.0 {
				let color1 = (map.get_pixel((self.x - 5.0) as i32, (self.y - 5.0) as i32) & 0xff000000) >> 24;
				let color2 = (map.get_pixel((self.x + 5.0) as i32, (self.y - 5.0) as i32) & 0xff000000) >> 24;
				if color1 != 0 || color2 != 0 {
					self.kill();
					self.y -= self.yv;
					self.yv *= -0.5;
				}
			}
			
			self.yv += 0.01;
			self.xv *= 0.999;
			self.yv *= 0.999;
			
			self.life_time -= 1;
		}
		if self.animation_frame == 12 {
			create_explosion((self.x) as i32, (self.y + 5.0) as i32, 30, entity_handler);
		}
		
		return self.life_time == 0 ||  (self.is_dead && self.animation_frame >= 20);
	}
	
	fn kill(&mut self) {
		self.is_dead = true;
	}
	
	fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		unsafe {
			if self.y < 600.0 && self.x > -5.0 && self.x < GAME_WIDTH as f32 + 5.0 {
				g_y_sum += self.y;
				g_y_sum_count += 1;	
			}
		}
		let image_container = unsafe {g_image_container.as_ref().unwrap()};
		if self.is_dead {
			screen.draw_image(&(image_container.explosion[self.animation_frame as usize/4]), (self.x - camera.x) as i32 - 25, (self.y - camera.y) as i32 - 20, 50, 50);
		} else {
			screen.draw_image(&(image_container.bomb), (self.x - camera.x) as i32 - 5, (self.y - camera.y) as i32 - 5, 10, 10);
		}
	}
}