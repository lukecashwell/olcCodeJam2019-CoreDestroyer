pub struct ImageContainer {
	pub sky: Image,
	pub explosion: Vec<Image>,
	pub bomb: Image,
	pub bounce_bomb: Image,
	pub core: Image,
	pub core_explosion: Vec<Image>,
	pub plane: Image,
	pub star_filled: Image,
	pub star_empty: Image,
	pub restart: Image,
	pub next_level: Image,
	pub star_explosion: Vec<Image>,
	pub star_core: Image,
	pub star_filled_small: Image,
	pub star_empty_small: Image
}

impl ImageContainer {
	pub fn create() -> Self {
		let mut core_explosion = Vec::new();
		core_explosion.push(Image::load("core_explosion_1.png"));
		core_explosion.push(Image::load("core_explosion_2.png"));
		core_explosion.push(Image::load("core_explosion_3.png"));
		core_explosion.push(Image::load("core_explosion_4.png"));
		core_explosion.push(Image::load("core_explosion_5.png"));
		core_explosion.push(Image::load("core_explosion_6.png"));
		core_explosion.push(Image::load("core_explosion_7.png"));
		
		let mut star_explosion = Vec::new();
		star_explosion.push(Image::load("star_explosion_1.png"));
		star_explosion.push(Image::load("star_explosion_2.png"));
		star_explosion.push(Image::load("star_explosion_3.png"));
		star_explosion.push(Image::load("star_explosion_4.png"));
		star_explosion.push(Image::load("star_explosion_5.png"));
		star_explosion.push(Image::load("star_explosion_6.png"));
		star_explosion.push(Image::load("star_explosion_7.png"));
	
		
		let mut explosion = Vec::new();
		explosion.push(Image::load("explosion_1.png"));
		explosion.push(Image::load("explosion_2.png"));
		explosion.push(Image::load("explosion_3.png"));
		explosion.push(Image::load("explosion_4.png"));
		explosion.push(Image::load("explosion_5.png"));		
		
		Self{
			sky:         Image::load("sky.png"),
			explosion:   explosion,
			bomb:        Image::load("bomb.png"),
			bounce_bomb: Image::load("bounce_bomb.png"),
			core:        Image::load("core.png"),
			core_explosion: core_explosion,
			plane:       Image::load("plane.png"),
			star_filled: Image::load("star_filled.png"),
			star_empty:  Image::load("star_empty.png"),
			restart:     Image::load("restart.png"),
			next_level:  Image::load("next_level.png"),	
			star_explosion: star_explosion,
			star_core:   Image::load("star_core.png"),	
			star_filled_small:Image::load("star_filled_small.png"),
			star_empty_small:Image::load("star_empty_small.png")
		
		}
	}
}