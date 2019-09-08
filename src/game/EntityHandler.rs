pub struct EntityHandler<'a> {
	entities: Vec<&'a mut dyn Entity>,
	particles: Vec<Particle>,
	bombs: Vec<Bomb>,
	bounce_bombs: Vec<BounceBomb>,
	pub cores: Vec<Core>,
	pub star_cores: Vec<StarCore>,
	last_time: u128
}

impl<'a> EntityHandler<'a> {
	pub fn create() -> Self {
		let stamp = SystemTime::now();
		Self{entities: Vec::new(), 
			 particles: Vec::new(), 
			 bombs: Vec::new(), 
			 bounce_bombs: Vec::new(), 
			 cores: Vec::new(),
			 star_cores: Vec::new(), 
			 last_time: stamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 100000000}
	}
	
	pub fn refresh_time(&mut self) {
		let stamp = SystemTime::now();
		self.last_time = stamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 1000000000;
	}
	
	pub fn update(&mut self) {
		let stamp = SystemTime::now();
		let now = stamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 1000000000;
		while self.last_time % 1000000000 < now {
			{
				let mut i = 0 ;
				let mut l = self.entities.len() ;
				while i < l {
					let result = self.entities[i].update();
					if result {
						self.entities.remove(i);
					} else {
						i += 1;	
					}
					l = self.entities.len();
				}	
			}
			{
				let mut i = 0 ;
				let mut l = self.particles.len() ;
				while i < l {
					let result = self.particles[i].update();
					if result {
						self.particles.remove(i);
					} else {
						i += 1;	
					}
					l = self.particles.len();
				}	
			}
			{
				let mut i = 0 ;
				let mut l = self.bombs.len() ;
				while i < l {
					let result = self.bombs[i].update();
					if result {
						self.bombs.remove(i);
					} else {
						i += 1;	
					}
					l = self.bombs.len();
				}	
			}
			{
				let mut i = 0 ;
				let mut l = self.bounce_bombs.len() ;
				while i < l {
					let result = self.bounce_bombs[i].update();
					if result {
						self.bounce_bombs.remove(i);
					} else {
						i += 1;	
					}
					l = self.bounce_bombs.len();
				}	
			}
			{
				let mut i = 0 ;
				let mut l = self.cores.len() ;
				while i < l {
					let result = self.cores[i].update();
					if result {
						self.cores.remove(i);
					} else {
						i += 1;	
					}
					l = self.cores.len();
				}	
			}
			{
				let mut i = 0 ;
				let mut l = self.star_cores.len() ;
				while i < l {
					let result = self.star_cores[i].update();
					if result {
						self.star_cores.remove(i);
					} else {
						i += 1;	
					}
					l = self.star_cores.len();
				}	
			}
			self.last_time += 10;
			self.last_time %= 1000000000;
		}				
	} 
	
	pub fn render(&mut self, screen: &mut Image, camera: &mut Camera) {
		for entity in self.entities.iter_mut() {
			entity.render(screen, camera);
		}
		for particles in self.particles.iter_mut() {
			particles.render(screen, camera);
		}
		for bombs in self.bombs.iter_mut() {
			bombs.render(screen, camera);
		}
		for bounce_bombs in self.bounce_bombs.iter_mut() {
			bounce_bombs.render(screen, camera);
		}
		for cores in self.cores.iter_mut() {
			cores.render(screen, camera);
		}
		for star_cores in self.star_cores.iter_mut() {
			star_cores.render(screen, camera);
		}
	}
	
	pub fn add_entity(&mut self, entity: &'a mut dyn Entity) {
		self.entities.push(entity);
	}
	
	pub fn add_particle(&mut self, particle: Particle) {
		self.particles.push(particle);
	}
	
	pub fn add_bomb(&mut self, bomb: Bomb) {
		self.bombs.push(bomb);
	}
	
	pub fn add_bounce_bomb(&mut self, bounce_bomb: BounceBomb) {
		self.bounce_bombs.push(bounce_bomb);
	}
	
	pub fn add_core(&mut self, core: Core) {
		self.cores.push(core);
	}
	
	pub fn add_star_core(&mut self, star_core: StarCore) {
		self.star_cores.push(star_core);
	}
	
	pub fn get_entity_count(&self) -> usize {
		self.bounce_bombs.len() + self.bombs.len() + self.particles.len() + self.entities.len()
	}  
}