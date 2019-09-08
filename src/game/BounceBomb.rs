pub struct BounceBomb {
	x: f32,
	y: f32,
	xv: f32,
	yv: f32,
	life_time: u32,
	bounce_count: u32,
	is_dead: bool,
	animation_frame: u32
}

impl BounceBomb {
	pub fn new(x: f32, y: f32, xv: f32, yv: f32) -> Self {
		Self{x: x, y: y, xv: xv, yv: yv, life_time: 1000, is_dead: false, animation_frame: 0, bounce_count: 0}
	}
}

impl Entity for BounceBomb {
	fn update(&mut self) -> bool {
		let map = unsafe {g_map.as_ref().unwrap()};
		if self.is_dead {
			self.animation_frame += 1;
		} else {
	
			self.x += self.xv;
			self.y += self.yv;
		
			
			let mut collision_points = Vec::new(); 
			for a in 0..36 {
				let cos = cosd(a as f32*10.0);
				let sin = sind(a as f32*10.0);
				for l in 1..5 {
					let nx = self.x + cos*l as f32;
					let ny = self.y + sin*l as f32;
					let color = (map.get_pixel(nx as i32, ny as i32) & 0xff000000) >> 24;
					if color != 0 {
						collision_points.push([nx, ny]);
						break;
					} 
				}
			}
			if collision_points.len() > 0 {
				self.x -= self.xv;
				self.y -= self.yv;
				let mut sx = 0.0;
				let mut sy = 0.0;
				for i in 0..collision_points.len() {
					sx += collision_points[i][0];
					sy += collision_points[i][1];
				}
				sx /= collision_points.len() as f32;
				sy /= collision_points.len() as f32;
				sx -= self.x;
				sy -= self.y;
				let smagnitude = sqrt((sx*sx + sy*sy) as f64).unwrap() as f32;
				let magnitude = sqrt((self.xv*self.xv + self.yv*self.yv) as f64).unwrap() as f32;
				if smagnitude > 0.01 {
					sx /= -smagnitude;
					sy /= -smagnitude;
					let rx = sx*sx*self.xv + 2.0*self.yv*sy*sx - self.xv*sy*sy;
			 		let ry = sy*sy*self.yv + 2.0*self.xv*sy*sx - self.yv*sx*sx;
					let sqrdmag = rx as f64*rx as f64 + ry as f64*ry as f64;
					let rmagnitude = sqrt(sqrdmag).unwrap() as f32;
					if rmagnitude > 0.01 {
						self.xv = -rx/rmagnitude*magnitude*0.8;
						self.yv = -ry/rmagnitude*magnitude*0.8;
					}		
				}
				self.bounce_count+=1;
			}
			if self.bounce_count >= 10 {
				self.kill();
			}
		
			self.yv += 0.02;
			self.xv *= 0.9999;
			self.yv *= 0.9999;
		}
		if self.animation_frame == 12 {
			create_explosion((self.x) as i32, (self.y + 5.0) as i32, 25, unsafe{g_entity_handler.as_mut().unwrap()});
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
			self.kill();
			screen.draw_image(&(image_container.explosion[self.animation_frame as usize/4]), (self.x - camera.x) as i32 - 25, (self.y - camera.y) as i32 - 20, 50, 50);
		} else {
			screen.draw_image(&(image_container.bounce_bomb), 
			(self.x - camera.x) as i32 - 5,
			 (self.y - camera.y) as i32 - 5,
			 10, 10);
		}
	}
}