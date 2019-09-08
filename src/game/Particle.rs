pub struct Particle {
	x: f32,
	y: f32,
	xv: f32,
	yv: f32,
	color: u32,
	life_time: u32,
	is_dead: bool
}

impl Particle {
	pub fn new(x: f32, y: f32, xv: f32, yv: f32, color: u32) -> Self {
		Self{x: x, y: y, xv: xv, yv: yv, color: color, life_time: 1000, is_dead: false}
	}
}

impl Entity for Particle {
	fn update(&mut self) -> bool {
		let map = unsafe {g_map.as_ref().unwrap()};
		self.x += self.xv;
		
		//X collisions
		if self.xv > 0.0 {
			let color1 = (map.get_pixel((self.x + 0.5) as i32, self.y as i32) & 0xff000000) >> 24;
			let color2 = (map.get_pixel((self.x + 0.5) as i32, (self.y + 1.0) as i32) & 0xff000000) >> 24;
			if color1 != 0 || color2 != 0 {
				self.x -= self.xv;
				self.xv *= -0.5;
			} 
		} else 
		if self.xv < 0.0 {
			let color1 = (map.get_pixel((self.x) as i32, self.y as i32) & 0xff000000) >> 24;
			let color2 = (map.get_pixel((self.x) as i32, (self.y + 1.0) as i32) & 0xff000000) >> 24;
			if color1 != 0 || color2 != 0 {
				self.x -= self.xv;
				self.xv *= -0.5;
			}
		}
		
		self.y += self.yv;
		
		//Y collisions
		if self.yv > 0.0 {
			let color1 = (map.get_pixel((self.x) as i32, (self.y + 0.5)as i32) & 0xff000000) >> 24;
			let color2 = (map.get_pixel((self.x + 0.9) as i32, (self.y + 0.5) as i32) & 0xff000000) >> 24;
			if color1 != 0 || color2 != 0 {
				self.y -= self.yv;
				self.yv *= -0.5;
			}
		} else 
		if self.yv < 0.0 {
			let color1 = (map.get_pixel((self.x) as i32, (self.y) as i32) & 0xff000000) >> 24;
			let color2 = (map.get_pixel((self.x + 0.9) as i32, (self.y) as i32) & 0xff000000) >> 24;
			if color1 != 0 || color2 != 0 {
				self.y -= self.yv;
				self.yv *= -0.5;
			}
		}
		
		self.yv += 0.01;
		self.xv *= 0.999;
		self.yv *= 0.999;
		
		self.life_time -= 1;
		if self.life_time == 1 {
			if absf32(self.xv) < 0.01 && absf32(self.yv) < 0.01 {
				unsafe{g_map.as_mut().unwrap()}.set_pixel(self.x as i32, self.y as i32, self.color);
			}
		}
	
		return self.life_time == 0;
	}
	
	fn kill(&mut self) {
		self.is_dead = true;
	}
	
	fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		screen.set_pixel((self.x - camera.x) as i32, (self.y - camera.y) as i32, self.color | 0xff000000);
	}
}