pub struct MainMenu {
	start_button: StartButton,
	level_select_right: LevelSelectRight,
	level_select_left: LevelSelectLeft,
	mouse_flag: bool,
	last_selected_level: u32,
	pub level_icons: Vec<Image>,
	pub level_file_names: Vec<String>,
	pub level_backgrounds: Vec<Image>,
	pub level_names: Vec<String>,
	pub level_cores: Vec<Vec<[u32; 2]>>,
	pub level_bomb_star: Vec<u32>,
	pub level_star_cores: Vec<Vec<[u32; 2]>>,
	pub level_tips: Vec<Vec<String>>
}

pub static mut selected_level: u32 = 0;
pub static mut selected_level_name: Option<String> = None;
pub static mut level_count: u32 = 0;
pub static mut level_stars: Option<Vec<[bool; 3]>> = None;
pub static mut selected_level_cores: Option<Vec<[u32; 2]>> = None;
pub static mut selected_level_core_count: u32 = 0;
pub static mut selected_level_star_cores: Option<Vec<[u32; 2]>> = None;
pub static mut selected_level_star_core_count: u32 = 0;
pub static mut selected_level_tip: Option<Vec<String>> = None;

impl MainMenu {
	
	pub fn create() -> Self {
		unsafe{selected_level = 0};
	
		let mut level_icons = Vec::new();
		let mut level_file_names = Vec::new();
		let mut level_names = Vec::new();
		let mut level_cores = Vec::new();
		let mut level_backgrounds = Vec::new(); 
		let mut level_bomb_star = Vec::new();
		let mut level_star_cores = Vec::new();
		let mut level_tips = Vec::new();
		
		level_icons      .push(Image::load("Level_0\\icon.png"));
		level_backgrounds.push(Image::load("Level_0\\nothing.png lol"));
		level_file_names .push("Level_0\\map.png".to_owned());
		level_names      .push("<Not> Tutorial".to_owned());
		level_bomb_star  .push(1);
		level_cores      .push(vec![[148, 138]]);
		level_star_cores .push(vec![[40, 169]]);
		level_tips       .push(vec!["You cannot drop".to_owned(), 
									"below the line.".to_owned()]);
		
		level_icons      .push(Image::load("Level_1\\icon.png"));
		level_backgrounds.push(Image::load("Lellefowmeofpwefrfwergerg.png"));
		level_file_names .push("Level_1\\map.png".to_owned());
		level_names      .push("Through the Bunker".to_owned());
		level_bomb_star  .push(2);
		level_cores      .push(vec![[153, 167]]);
		level_star_cores .push(vec![[146, 66]]);
		level_tips       .push(vec!["Bouncy bombs might".to_owned(),
									"be useful."        .to_owned()]);
		
		level_icons      .push(Image::load("Level_2\\icon.png"));
		level_backgrounds.push(Image::load("Level_2\\nothing.png lol"));
		level_file_names .push("Level_2\\map.png".to_owned());
		level_names      .push("Bouncy Frustration".to_owned());
		level_bomb_star  .push(2);
		level_cores      .push(vec![[191, 302]]);
		level_star_cores .push(vec![[69, 248]]);
		level_tips       .push(vec!["Drag and drop to".to_owned(), 
									"shoot at angles.".to_owned()]);
									
		level_icons      .push(Image::load("Level_3\\icon.png"));
		level_backgrounds.push(Image::load("Level_3\\nothing.png lol"));
		level_file_names .push("Level_3\\map.png".to_owned());
		level_names      .push("The Caves".to_owned());
		level_bomb_star  .push(20);
		level_cores      .push(vec![[155, 301], [216, 322]]);
		level_star_cores .push(vec![[188, 262], [51, 330]]);
		level_tips       .push(vec!["You must destroy".to_owned(), "all cores.".to_owned()]); 
		
		
		let mut level_star = Vec::new();
		for _ in 0..level_icons.len() {
			level_star.push([false, false, false]);
		}
		unsafe{level_count = level_icons.len() as u32};
		unsafe{level_stars = Some(level_star)};
		unsafe{selected_level_name = Some(level_file_names[0].clone())};
		unsafe{selected_level_cores = Some(level_cores[0].clone())};		
		unsafe{selected_level_core_count = level_cores[0].len() as u32};
		unsafe{selected_level_star_cores = Some(level_star_cores[0].clone())};		
		unsafe{selected_level_star_core_count = level_star_cores[0].len() as u32};
		unsafe{selected_level_tip = Some(level_tips[0].clone())};
	
		Self {
			start_button: StartButton::new((WIDTH + DEBUG_WIDTH - 100) as i32/ 2, (HEIGHT - 50) as i32/ 2 + 150, 100, 30),
			level_select_right: LevelSelectRight::new((WIDTH + DEBUG_WIDTH - 25) as i32/ 2 + 100, (HEIGHT - 50) as i32/ 2, 25, 50),
			level_select_left: LevelSelectLeft::new((WIDTH + DEBUG_WIDTH - 25) as i32/ 2 - 100, (HEIGHT - 50) as i32/ 2, 25, 50),
			mouse_flag: false,
			level_icons: level_icons,
			level_file_names: level_file_names,
			level_names: level_names,
			level_cores: level_cores,
			level_backgrounds: level_backgrounds,
			level_bomb_star: level_bomb_star,
			last_selected_level: 1,
			level_star_cores: level_star_cores,
			level_tips: level_tips
		}
	}

	pub fn main_loop(&mut self, screen: &mut Image, window: &mut minifb::Window) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		let mouse_pos = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap();
		let mousex = mouse_pos.0;
		let mousey = mouse_pos.1;
			
		unsafe {selected_level_name = Some(self.level_file_names[selected_level as usize].clone())};
		unsafe {selected_level_cores = Some(self.level_cores[selected_level as usize].clone())};		
		unsafe {selected_level_core_count = self.level_cores[selected_level as usize].len() as u32};
		unsafe {selected_level_star_cores = Some(self.level_star_cores[selected_level as usize].clone())};		
		unsafe {selected_level_star_core_count = self.level_star_cores[selected_level as usize].len() as u32};
		
		if self.last_selected_level != unsafe{selected_level} {
			unsafe{ g_map_background = Some(self.level_backgrounds[selected_level as usize].clone())};
			unsafe{ g_start_limit_bomb_count = self.level_bomb_star[selected_level as usize]};
			unsafe{ selected_level_tip = Some(self.level_tips[selected_level as usize].clone())};
		}
		self.last_selected_level = unsafe{selected_level};	
		
		let text = "Core Destoryer";
		let size = 5;
		screen.draw_font(font, !0, text, ((WIDTH + DEBUG_WIDTH) as i32 - font.get_text_width(text, size) as i32)/2, 50, size);
		
		let text = self.level_names[unsafe{selected_level} as usize].as_str();
		let size = 1;
		screen.draw_font(font, !0, text, ((WIDTH + DEBUG_WIDTH) as i32 - font.get_text_width(text, size) as i32)/2, (HEIGHT - font.get_height()*size) as i32/2 + 75, size);
	
		let text = format!("Level :{}", unsafe{selected_level} as usize);
		let size = 1;
		screen.draw_font(font, !0, text.as_str(), ((WIDTH + DEBUG_WIDTH) as i32 - font.get_text_width(text.as_str(), size) as i32)/2, (HEIGHT - font.get_height()*size) as i32/2 + 64, size);
	
	
		screen.fill_rect(!0, ((WIDTH + DEBUG_WIDTH) as i32 - 104)/2, (HEIGHT - 104) as i32/2, 104, 104);
		screen.draw_image(&(self.level_icons[unsafe{selected_level} as usize]), ((WIDTH + DEBUG_WIDTH) as i32 - 100)/2, (HEIGHT - 100) as i32/2, 100, 100);
	
		let selected_level_stars = unsafe{level_stars.as_ref().unwrap()[selected_level as usize]};
		let image_container = unsafe{g_image_container.as_ref().unwrap()};
		screen.draw_image({
			if selected_level_stars[0] {
				&(image_container.star_filled_small)
			} else {
				&(image_container.star_empty_small)
			}
		}, ((WIDTH + DEBUG_WIDTH) as i32)/2 - 50, (HEIGHT) as i32/2 - 87, 32, 32);
		screen.draw_image({
			if selected_level_stars[1] {
				&(image_container.star_filled_small)
			} else {
				&(image_container.star_empty_small)
			}
		}, ((WIDTH + DEBUG_WIDTH) as i32)/2 - 16, (HEIGHT) as i32/2 - 87, 32, 32);
		screen.draw_image({
			if selected_level_stars[2] {
				&(image_container.star_filled_small)
			} else {
				&(image_container.star_empty_small)
			}
		}, ((WIDTH + DEBUG_WIDTH) as i32)/2 + 50 - 32, (HEIGHT) as i32/2 - 87, 32, 32);
		
		let mut star_count = 0;
		for i in 0..unsafe{level_count} as usize {
			if unsafe{level_stars.as_ref().unwrap()}[i][0] { star_count += 1 }
			if unsafe{level_stars.as_ref().unwrap()}[i][1] { star_count += 1 }
			if unsafe{level_stars.as_ref().unwrap()}[i][2] { star_count += 1 }
		}
		let text = format!("{}/{}", star_count, unsafe{level_count}*3);
		screen.draw_image(&(image_container.star_filled_small), ((WIDTH + DEBUG_WIDTH) as i32) - font.get_text_width(text.as_str(), 1) as i32 - 28, 4, 16, 16);
		screen.draw_font(font, !0, text.as_str(), ((WIDTH + DEBUG_WIDTH) as i32) - font.get_text_width(text.as_str(), 1) as i32 - 10, 10, 1);
		
		self.start_button.render(screen, mousex, mousey);
		self.level_select_right.render(screen, mousex, mousey);
		self.level_select_left.render(screen, mousex, mousey);
		
		if window.get_mouse_down(minifb::MouseButton::Left) {
			if !self.mouse_flag {
				self.mouse_flag = true;
			
				self.start_button.on_mouse_left_press(mousex, mousey, window);
				self.level_select_right.on_mouse_left_press(mousex, mousey, window);
				self.level_select_left.on_mouse_left_press(mousex, mousey, window);
			}
		} else {
			if self.mouse_flag {
				self.mouse_flag = false;
				
				self.start_button.on_mouse_left_release(mousex, mousey, window);
				self.level_select_right.on_mouse_left_release(mousex, mousey, window);
				self.level_select_left.on_mouse_left_release(mousex, mousey, window);
			}
		}
	}
}

pub struct ExitButton {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl ExitButton {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe{
				g_game_state = STATE_LOADING;
				g_loading_mode = LOADING_MENU;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		let text = "Exit";
		let size = 1;
		screen.draw_font(font, !0, text, self.x + (self.width - 2 - font.get_text_width(text, size) as i32)/2, self.y + (self.height - 2 - (font.get_height()*size) as i32)/2, size);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

struct StartButton {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl StartButton {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe {
				g_game_state = STATE_LOADING;
				g_loading_mode = LOADING_MAP;	
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		let text = "Start Level";
		let size = 1;
		screen.draw_font(font, !0, text, self.x + (self.width - 2 - font.get_text_width(text, size) as i32)/2, self.y + (self.height - 2 - (font.get_height()*size) as i32)/2, size);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}



struct LevelSelectRight {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl LevelSelectRight {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe{
				selected_level += 1;
				selected_level %= level_count;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		
		screen.draw_line(!0, self.x + 5, self.y + 4, self.x + self.width - 7, self.height/2 + self.y);
		screen.draw_line(!0, self.x + 5, self.y - 5 + self.height, self.x + self.width - 7, self.height/2 + self.y);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

struct LevelSelectLeft {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl LevelSelectLeft {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe{
				if selected_level == 0 {
					selected_level = level_count;
				}
				selected_level -= 1;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		
		screen.draw_line(!0, self.x + self.width - 7, self.y + 4, self.x + 5, self.height/2 + self.y);
		screen.draw_line(!0, self.x + self.width - 7, self.y - 5 + self.height, self.x + 5, self.height/2 + self.y);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

pub struct BombButton {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	image: Image,
	bomb_type: u32,
	is_pressed: bool
}

impl BombButton {
	pub fn new(x: i32, y: i32, width: i32, height: i32, image: Image, bomb_type: u32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false, image: image, bomb_type: bomb_type}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) -> bool {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
		return self.is_pressed;
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe {
				g_stat_selected_bomb = self.bomb_type;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let font = unsafe{ g_font.as_ref().unwrap()};
		screen.draw_rect(!0, self.x, self.y,         self.width,     self.height);
		if unsafe{g_stat_selected_bomb} == self.bomb_type {
			screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
		}
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			let text = match self.bomb_type {
				BOMB_BOUNCE => { "Bounce Bomb"},
				BOMB_DROP => { "Drop Bomb"},
				_ => { "unknown" }
			};
			if self.is_pressed {
				screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				if unsafe{g_stat_selected_bomb} != self.bomb_type {
					screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
				}	
			}
			screen.draw_image(&(self.image), self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			screen.draw_font(font, !0, text, mousex as i32, (mousey - (font.get_height()/2) as f32) as i32, 1);
		} else {
			screen.draw_image(&(self.image), self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

pub struct RestartButton {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl RestartButton {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			unsafe{
				g_game_state = STATE_LOADING;
				g_loading_mode = LOADING_MAP;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let image_container = unsafe{ g_image_container.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		screen.draw_image(&(image_container.restart), self.x + 1, self.y + 1, self.width - 2, self.height -2);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

pub struct NextButton {
 	x: i32, 
	y: i32,
	width: i32, 
	height: i32,
	is_pressed: bool
}

impl NextButton {
	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		Self{x: x, y: y, width: width, height: height, is_pressed: false}
	}
	
	pub fn on_mouse_left_press(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {
			self.is_pressed = true;	
		}
	}
	
	pub fn on_mouse_left_release(&mut self, mousex: f32, mousey: f32, window: &mut minifb::Window) {
		self.is_pressed = false;
		if self.mouse_is_in_button(mousex as i32, mousey as i32) {				
			unsafe {
				selected_level += 1;
				let main_menu = g_main_menu.as_ref().unwrap();
				selected_level_name = Some(main_menu.level_file_names[selected_level as usize].clone());
				selected_level_cores = Some(main_menu.level_cores[selected_level as usize].clone());
				selected_level_core_count = main_menu.level_cores[selected_level as usize].len() as u32;
				g_map_background = Some(main_menu.level_backgrounds[selected_level as usize].clone());
				g_start_limit_bomb_count = main_menu.level_bomb_star[selected_level as usize];
			    selected_level_star_cores = Some(main_menu.level_star_cores[selected_level as usize].clone());	
		        selected_level_star_core_count = main_menu.level_star_cores[selected_level as usize].len() as u32;
				selected_level_tip = Some(main_menu.level_tips[selected_level as usize].clone());
	
				
				g_game_state = STATE_LOADING;
				g_loading_mode = LOADING_MAP;
			}
		}
	}
	
	pub fn render(&mut self, screen: &mut Image, mousex: f32, mousey: f32) {
		let image_container = unsafe{ g_image_container.as_ref().unwrap()};
		screen.fill_rect(!0, self.x, self.y,         self.width,     self.height);
		if !self.is_pressed {
			if self.mouse_is_in_button(mousex as i32, mousey as i32) {
				screen.fill_rect(0xff00ff00, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
			} else {
				screen.fill_rect(0xff << 24, self.x + 1, self.y + 1, self.width - 2, self.height - 2);	
			}
		} else {
			screen.fill_rect(0xff0000ff, self.x + 1, self.y + 1, self.width - 2, self.height - 2);
		}
		screen.draw_image(&(image_container.next_level), self.x + 1, self.y + 1, self.width - 2, self.height -2);
	}
	
	fn mouse_is_in_button(&mut self,  mousex: i32, mousey: i32) -> bool {
		self.x <= mousex  && self.y <= mousey && mousex < self.x + self.width && mousey < self.y + self.height
	}
}

