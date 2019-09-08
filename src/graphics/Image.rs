pub struct Image {
	width: usize,
	height: usize,
	buffer: Vec<u32>	
}

impl Image {
	pub fn new(width: usize, height: usize, color: u32) -> Self {
		let mut buffer = Vec::new();
		for _ in 0..width*height { buffer.push(color); }
		Self{width: width, height: height, buffer: buffer}
	}
	
	pub fn load(name: &str) -> Self {
		let path: std::string::String = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "\\resources\\images\\" + name;
		let file = File::open(path);
		if file.is_err() {
			println!("[Image] unable to load: ~\\resources\\images\\{}", name);
		}else{
			let decoder = png::Decoder::new(file.unwrap());
			let (info, mut reader) = decoder.read_info().unwrap();
			let mut buf = vec![0; info.buffer_size()];
			reader.next_frame(&mut buf).unwrap();
			let mut new_buf: Vec<u32> = Vec::new();
			for i in 0..info.width*info.height {
				let r = (buf[i as usize*4 + 0] as u32) << 16;
				let g = (buf[i as usize*4 + 1] as u32) << 8;
				let b = buf[i as usize*4 + 2] as u32;
				let a = (buf[i as usize*4 + 3] as u32) << 24;
				new_buf.push(r | g | b | a);
			}
			return Self{width: info.width as usize, height: info.height as usize, buffer: new_buf};
		}
		Self{width: 2, height: 2, buffer: vec![0xff00, 0xfffff, 0xffffff, 0xff00]}
	}	
	
	pub fn load_offset(name: &str, xOff: u32, yOff: u32, width: u32, height: u32) -> Self {
		let path: std::string::String = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "\\resources\\images\\" + name;
		let file = File::open(path);
		if file.is_err() {
			println!("[Image] unable to load: ~\\resources\\images\\{}", name);
		}else{
			let decoder = png::Decoder::new(file.unwrap());
			let (info, mut reader) = decoder.read_info().unwrap();
			let mut buf = vec![0; info.buffer_size()];
			reader.next_frame(&mut buf).unwrap();
			let mut new_buf: Vec<u32> = Vec::new();
			
			for iy in yOff..(yOff + height) {
				let v = iy * info.width;
				for ix in xOff..(xOff + width) {
					let i = ix + v;
					let r = (buf[i as usize*4 + 0] as u32) << 16;
					let g = (buf[i as usize*4 + 1] as u32) << 8;
					let b = buf[i as usize*4 + 2] as u32;
					let a = (buf[i as usize*4 + 3] as u32) << 24;
					new_buf.push(a | r | g | b);
				}
			}
			return Self{width: width as usize, height: height as usize, buffer: new_buf};
		}
		Self{width: 2, height: 2, buffer: vec![0xff00, 0xfffff, 0xffffff, 0xff00]}
	}
	
	pub fn draw_line(&mut self, color: u32, x0: i32, y0: i32, x1: i32, y1: i32) {
		let slope = if y0 != y1 { (x0 - x1) as f32/(y0 - y1) as f32} else {1000000000.0};
		if absf32(slope) < 1.0 {
			let mut low_x = x1;
			let mut low_y = y1;
			let mut high_y = y0;
			if low_y > high_y {
				low_y = y0;
				high_y = y1;
				low_x = x0;
			}
			for y in 0..(high_y-low_y) {
				let x;
				if slope == 0.0 {
					x = 0;
				} else {
					x = (y as f32*slope) as i32;
				}
				self.set_pixel((x + low_x) as i32, (y + low_y) as i32, color | 0xff000000);
			}
		} else {
			let mut low_y = y1;
			let mut low_x = x1;
			let mut high_x = x0;
			if low_x > high_x {
				low_x = x0;
				high_x = x1;
				low_y = y0;
			}
			for x in 0..(high_x-low_x) {
				let y = (x as f32/slope) as i32;
				self.set_pixel((x + low_x) as i32, (y + low_y) as i32, color);
			}
		}
	}
	
	pub fn fill(&mut self, color: u32) {
		for i in 0..(self.width*self.height) - 1 {
			self.buffer[i] = color;
		}
	}
	
	pub fn tint(&mut self, tint: u32) {
		let tr = (tint & 0xff0000) >> 16;
		let tg = (tint & 0x00ff00) >> 8 ;
		let tb = (tint & 0x0000ff) >> 0 ;
		
		for i in 0..(self.width*self.height) - 1 {
			let color1 = self.buffer[i];
			let r = (color1 & 0xff0000) >> 16;
			let g = (color1 & 0x00ff00) >> 8 ;
			let b = color1 & 0x0000ff;
			let color2 = (clampi32(((tr*r)/255) as i32, 0, 255) << 16) as u32 | 
						(clampi32(((tg*g)/255) as i32, 0, 255) << 8) as u32 |
						 clampi32(((tb*b)/255) as i32, 0, 255) as u32 |
						0xff000000;
			self.buffer[i] = color2;
		}
	}
	
	//THIS FUNCTION IS NOT EFFICIENT
	pub fn draw_rect(&mut self, color: u32, x: i32, y: i32, w: i32, h: i32) {
		for iy in y..y+h {
			self.set_pixel(x, iy, color);
		}
		for ix in x..x+w {
			self.set_pixel(ix, y, color);
		}
		for iy in y..y+h {
			self.set_pixel(x + w - 1, iy, color);
		}
		for ix in x..x+w {
			self.set_pixel(ix, y + h - 1, color);
		}
	}
	
	pub fn fill_rect(&mut self, color: u32, x: i32, y: i32, w: i32, h: i32) {
		let nx = clampi32(x, 0, self.width as i32);
		let ny = clampi32(y, 0, self.height as i32);
		let nwp = clampi32(w + x, 0, self.width as i32);
		let nhp = clampi32(h + y, 0, self.height as i32);
		for y in ny..nhp {
			let v = y*self.width as i32;
			for x in nx..nwp {
				self.buffer[(x + v) as usize] = color | 0xff000000; 
			}
		} 
	}
	
	pub fn draw_circle(&mut self, color: u32, x: i32, y: i32, r: u32) {
		if r == 0 { self.set_pixel(x, y, color); return; }
		let nx = clampi32(x - r as i32, 0, self.width as i32);
		let ny = clampi32(y - r as i32, 0, self.height as i32);
		let nwp = clampi32(x + r  as i32 + 1, 0, self.width as i32);
		let nhp = clampi32(y + r as i32 + 1, 0, self.height as i32);
		for iy in ny..nhp {
			let v = iy*self.width as i32;
			for ix in nx..nwp {
				let lx = ix - x;
				let ly = iy - y;
				if (lx*lx + ly*ly)/(2*r) as i32 == r as i32/2 as i32 { 
					self.buffer[(ix + v) as usize] = color; 
				}
			}
		} 
	}
	
	pub fn fill_circle(&mut self, color: u32, x: i32, y: i32, r: u32) {
		if r == 0 { self.set_pixel(x, y, color); return; }
		let nx = clampi32(x - r as i32, 0, self.width as i32);
		let ny = clampi32(y - r as i32, 0, self.height as i32);
		let nwp = clampi32(x + r as i32 + 1, 0, self.width as i32);
		let nhp = clampi32(y + r as i32 + 1, 0, self.height as i32);
		let ds = r as i32*r as i32;
		for iy in ny..nhp {
			let v = iy*self.width as i32;
			for ix in nx..nwp {
				let lx = ix - x;
				let ly = iy - y;
				if lx*lx + ly*ly < ds { 
					self.buffer[(ix + v) as usize] = color; 
				}
			}
		} 
	}
	
	pub fn draw_font(&mut self, font: &Font, tint: u32, text: &str, x: i32, y: i32, s: usize) {
		let owned = text.to_owned();
		let ca = owned.as_bytes();
		for i in 0..ca.len() {
			let o = font.get_glyph(ca[i] as char);
			if o.is_some() {
				self.draw_image_tint(o.unwrap(), tint, x + (i*font.get_width()*s) as i32, y, (font.get_width()*s) as i32, (font.get_height()*s) as i32);
			}
		}
	}

	pub fn draw_image(&mut self, image: &Image, x: i32, y: i32, w: i32, h: i32) {
		let nx = clampi32(x, 0, self.width as i32);
		let ny = clampi32(y, 0, self.height as i32);
		let nwp = clampi32(w + x, 0, self.width as i32);
		let nhp = clampi32(h + y, 0, self.height as i32);
		for py in ny..nhp {
			let v = py*self.width as i32;
			for px in nx..nwp {
				let imgx = (px - x) as f32/ w as f32;
				let imgy = (py - y) as f32/ h as f32;
				let color = image.get_sample(imgx, imgy);
				let a = (color & 0xff000000) >> 24;
				if a != 0 {
					self.buffer[(px + v) as usize] = color;
				}
			}
		} 
	}
	
	pub fn draw_image_tint(&mut self, image: &Image, tint: u32, x: i32, y: i32, w: i32, h: i32) {
		let nx = clampi32(x, 0, self.width as i32);
		let ny = clampi32(y, 0, self.height as i32);
		let nwp = clampi32(w + x, 0, self.width as i32);
		let nhp = clampi32(h + y, 0, self.height as i32);
		let tr = (tint & 0xff0000) >> 16;
		let tg = (tint & 0x00ff00) >> 8 ;
		let tb = (tint & 0x0000ff) >> 0 ;
		
		for py in ny..nhp {
			let v = py*self.width as i32;
			for px in nx..nwp {
				let imgx = (px - x) as f32/ w as f32;
				let imgy = (py - y) as f32/ h as f32;
				let color = image.get_sample(imgx, imgy);
				let r = (color & 0xff0000) >> 16;
				let g = (color & 0x00ff00) >> 8 ;
				let b = color & 0x0000ff;
				let a = (color & 0xff000000) >> 24;
				let color = (clampi32(((tr*r)/255) as i32, 0, 255) << 16) as u32 | 
							(clampi32(((tg*g)/255) as i32, 0, 255) << 8)  as u32 |
							 clampi32(((tb*b)/255) as i32, 0, 255) as u32 | (a << 24) as u32;
				if a != 0 {
					self.buffer[(px + v) as usize] = color;
				}
			}
		} 
	}
	
	pub fn copy_dithered(&mut self, image: &Image, allowed_colors: &Vec<u32>) {
		if image.width != self.width && image.height != self.height {
			panic!("The images must be same size.");
		}
		let mut a_c: Vec<[u8; 3]> = Vec::new();
		for col in allowed_colors.iter() {
			let r = (col & 0xff0000) >> 16;
			let g = (col & 0xff00) >> 8; 
			let b = col & 0xff;
			a_c.push([r as u8, g as u8, b as u8]);
		}
		let mut buffer = image.get_buffer().clone();
		for py in 0..self.height as u32 {
			let v = py*self.width as u32;
			for px in 0..self.width as u32{
				let color = buffer[(px + v) as usize];
				let r = (color & 0xff0000) >> 16;
				let g = (color & 0xff00) >> 8; 
				let b = color & 0xff;
				let mut best_index = 0;
				let mut best_diff = 100000;
				let mut count = 0;
				for a in a_c.iter() {
					let mut avrg = absi32(r as i32 - a[0] as i32);
					avrg += absi32(g as i32 - a[1] as i32);
					avrg += absi32(b as i32 - a[2] as i32);
					avrg /= 3;
					if best_diff >= avrg {
						best_diff = avrg;		
						best_index = count;
					}		
					count += 1;	
				}
				let best_color = a_c[best_index];
				let best_color_r = best_color[0] as u32;
				let best_color_g = best_color[1] as u32;
				let best_color_b = best_color[2] as u32;
				let best_color_32 = ((best_color_r) << 16) | ((best_color_g) << 8) | (best_color_b);
				self.buffer[(px + v) as usize] = best_color_32;
				
				let rerror = r as i32 - best_color_r as i32;
				let gerror = g as i32 - best_color_g as i32;
				let berror = b as i32 - best_color_b as i32;
						
				//---------------
				
				if px < self.width as u32  - 1  {
					let ar = ((buffer[(px + v) as usize + 1]	& 0xff0000) >> 16) as i32;
					let ag = ((buffer[(px + v) as usize + 1]	& 0xff00) >> 8   ) as i32;
					let ab = (buffer[(px + v) as usize + 1]	& 0xff)           	   as i32;
					
					let mut nr = clampi32((ar + (rerror * 7 / 16)) as i32, 0, 0xff) as u32;
					let mut ng = clampi32((ag + (gerror * 7 / 16)) as i32, 0, 0xff) as u32;			
					let nb = clampi32((ab + (berror * 7 / 16)) as i32, 0, 0xff) as u32;

					nr <<= 16;
					ng <<= 8;
					buffer[(px + v) as usize + 1] = nr | ng | nb;
				}
				if py < self.height as u32  - 1 && px > 0  {
					let ar = ((buffer[(px + v) as usize - 1 + self.width]	& 0xff0000) >> 16) as i32;
					let ag = ((buffer[(px + v) as usize - 1 + self.width]	& 0xff00) >> 8   ) as i32;
					let ab = (buffer[(px + v) as usize - 1 + self.width]	& 0xff)           	   as i32;
					
					let mut nr = clampi32((ar + (rerror * 3 / 16)) as i32, 0, 0xff) as u32;
					let mut ng = clampi32((ag + (gerror * 3 / 16)) as i32, 0, 0xff) as u32;			
					let nb = clampi32((ab + (berror * 3 / 16)) as i32, 0, 0xff) as u32;

					
					nr <<= 16;
					ng <<= 8;
					buffer[(px + v) as usize - 1 + self.width] = nr | ng | nb | 0xff000000;
				}
				
				if py < self.height as u32  - 1 {
					let ar = ((buffer[(px + v) as usize + self.width]	& 0xff0000) >> 16) as i32;
					let ag = ((buffer[(px + v) as usize + self.width]	& 0xff00) >> 8   ) as i32;
					let ab = (buffer[(px + v) as usize + self.width]	& 0xff)           	   as i32;
					
					let mut nr = clampi32((ar + (rerror * 5 / 16)) as i32, 0, 0xff) as u32;
					let mut ng = clampi32((ag + (gerror * 5 / 16)) as i32, 0, 0xff) as u32;			
					let nb = clampi32((ab + (berror * 5 / 16)) as i32, 0, 0xff) as u32;

					
					nr <<= 16;
					ng <<= 8;
					buffer[(px + v) as usize + self.width] = nr | ng | nb | 0xff000000;
				}
			
				if py < self.height as u32  - 1 && px < self.width as u32  - 1 {
					let ar = ((buffer[(px + v) as usize + self.width + 1]	& 0xff0000) >> 16) as i32;
					let ag = ((buffer[(px + v) as usize + self.width + 1]	& 0xff00) >> 8   ) as i32;
					let ab = (buffer[(px + v) as usize + self.width  + 1]	& 0xff)           	   as i32;
					
					let mut nr = clampi32((ar + (rerror * 1 / 16)) as i32, 0, 0xff) as u32;
					let mut ng = clampi32((ag + (gerror * 1 / 16)) as i32, 0, 0xff) as u32;			
					let nb = clampi32((ab + (berror * 1 / 16)) as i32, 0, 0xff) as u32;

					
					nr <<= 16;
					ng <<= 8;
					buffer[(px + v) as usize + self.width + 1] = nr | ng | nb | 0xff000000;
				}	
				
				//-------------
				
			}
		} 
	}
	
	pub fn get_sample(&self, x: f32, y: f32) -> u32 {
		if x > 1.0 || x < 0.0 {
			panic!("x out of range 0-1: {}", x);
		} 
		if y > 1.0 || y < 0.0 {
			panic!("y out of range 0-1: {}", y);
		}
		self.buffer[clampf32(x*(self.width as f32) + (floor32(y*(self.height as f32))*(self.width as f32)), 0.0, (self.width*self.height) as f32 - 1.0) as i32 as usize]
	} 
	
	pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
		if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
			return 0;
		}
		self.buffer[(x + y*self.width as i32) as usize]
	}
	
	pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
		if x >= 0 && x < self.width as i32{
			if y >= 0 && y < self.height as i32 {
				self.buffer[x as usize + self.width*y as usize] = color;
			}
		}
	}
	
	pub fn get_rgb(&self, x: u32, y: u32) -> [u8; 3] {
		let color = self.buffer[(x + y*self.height as u32) as usize];
		let r = color & 0xff0000 >> 16;
		let g = color & 0xff00 >> 8; 
		let b = color & 0xff;
		[r as u8, g as u8, b as u8]
	}
	
	pub fn set_rgb(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
		self.buffer[(x + y*self.height as u32) as usize] = ((r as u32) << 16) | ((g as u32) << 8) | b as u32 | 0xff000000;
	}
	
	pub fn get_buffer(&self) -> &Vec<u32> {
		&self.buffer
	}
	
	pub fn get_width(&self) -> usize {
		self.width
	}
	
	pub fn get_height(&self) -> usize {
		self.height
	}
	 
	pub fn clone(&self) -> Self {
		Self{width: self.width, height: self.height, buffer: self.buffer.clone()}
	} 
	
}