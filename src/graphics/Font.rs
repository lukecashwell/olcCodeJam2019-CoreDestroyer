pub struct Font {
	glyphs: HashMap<u8, Image>,
	height: u8, 
	width: u8
}

impl Font {
	pub fn new(name: &str, chars: &str, width: u8, height: u8) -> Self {
		let owned = chars.to_owned();
		let ca = owned.as_bytes();
		let mut ret = HashMap::new();
		for i in 0..ca.len() as u32 {
			ret.insert(ca[i as usize], Image::load_offset(name, 0, i*height as u32, width as u32, height as u32));
		}
		Self{glyphs: ret, height: height, width: width}
	}
	
	pub fn get_text_width(&self, text: &str, scale: usize) -> usize {
		text.len()*scale*self.get_width()
	}
	
	pub fn get_glyph(&self, c: char) -> Option<&Image> {
		self.glyphs.get(&(c as u8))
	}
	
	pub fn get_height(&self) -> usize {
		self.height as usize
	}
	
	pub fn get_width(&self) -> usize {
		self.width as usize
	}
	
}