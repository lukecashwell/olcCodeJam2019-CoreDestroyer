pub fn create_explosion(x: i32, y: i32, r: i32, entity_handler: &mut EntityHandler) {
	for i in 0..entity_handler.cores.len() {
		let dx = entity_handler.cores[i].x - x as f32;
		let dy = entity_handler.cores[i].y - y as f32;
		if dx*dx + dy*dy <= r as f32*r as f32 {
			entity_handler.cores[i].kill();
		}
	}
	for i in 0..entity_handler.star_cores.len() {
		let dx = entity_handler.star_cores[i].x - x as f32;
		let dy = entity_handler.star_cores[i].y - y as f32;
		if dx*dx + dy*dy <= r as f32*r as f32 {
			entity_handler.star_cores[i].kill();
		}
	}
	
	let mut rng = rand::thread_rng();
	let map = unsafe{g_map.as_mut().unwrap()};
	let width = map.get_width();
	let height = map.get_height();
	
	if r == 0 { map.set_pixel(x, y, 0); return; }
	let nx = clampi32(x - r, 0, width as i32);
	let ny = clampi32(y - r, 0, height as i32);
	let nwp = clampi32(x + r + 1, 0, width as i32);
	let nhp = clampi32(y + r + 1, 0, height as i32);
	let ds = r as i32*r as i32;
	for iy in ny..nhp {
		for ix in nx..nwp {
			let lx = ix - x;
			let ly = iy - y;
			if lx*lx + ly*ly <= ds { 
				let mut r1: f32 = rng.gen();
				r1 *= (lx*lx + ly*ly) as f32;
				if r1 < (r*r) as f32/2.0 {
					let vx: f32 = rng.gen();
					let vy: f32 = rng.gen();					
					let p = map.get_pixel(ix, iy);
					if p & 0xff000000 == 0xff000000{
						unsafe{ g_score += 1 };
						entity_handler.add_particle(Particle::new(ix as f32, iy as f32, vx*0.2 - 0.1 + 1.0/(lx*lx) as f32, vy*0.2 - 0.1 + 1.0/(ly*ly) as f32, p));
						map.set_pixel(ix, iy, 0); 	
					}
				}
			}
		}
	} 
}