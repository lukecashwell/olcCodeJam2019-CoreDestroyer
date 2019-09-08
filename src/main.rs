#![windows_subsystem = "windows"]
mod consts {
	
	pub const WIDTH: usize = 500;
	pub const HEIGHT: usize = 500;
	pub const DEBUG_WIDTH: usize = 200;
	pub const GAME_WIDTH: usize = 250;
	pub const GAME_HEIGHT: usize = 250;
	
	pub const TITLE: &'static str = "OLC Rust Game";
	
	
	pub const STATE_MAIN_MENU: u32 = 0;
	pub const STATE_GAME: u32 = 1;
	pub const STATE_LOADING: u32 = 2;
	pub const STATE_END: u32 = 3;
	
	pub const LOADING_MAP: u32 = 1;
	pub const LOADING_MENU: u32 = 2;
	
	pub const BOMB_DROP: u32 = 1;
	pub const BOMB_BOUNCE: u32 = 2;
}
mod globals {
	use crate::graphics::*;
	use crate::game::*;
	use crate::consts::*;
	
	pub static mut g_entity_handler: Option<EntityHandler> = None;
	pub static mut g_image_container: Option<ImageContainer> = None;
	pub static mut g_map: Option<Image> = None;
	pub static mut g_map_background: Option<Image> = None;
	pub static mut g_game_image: Option<Image> = None;
	pub static mut g_debug_image: Option<Image> = None;
	pub static mut g_font: Option<Font> = None;
	pub static mut g_camera: Option<Camera> = None;
	pub static mut g_set_camera: Option<Camera> = None;
	pub static mut g_score: u32 = 0;
	pub static mut g_game_state: u32 = STATE_MAIN_MENU;
	pub static mut g_loading_mode: u32 = 0;
	pub static mut g_start_limit_bomb_count: u32 = 0;
	
	pub static mut g_main_menu: Option<MainMenu> = None;
	
	pub static mut g_stat_selected_bomb: u32 = 0;
	pub static mut g_stat_bombs_released: u32 = 0;
	pub static mut g_stat_cores_destroyed: u32 = 0;
	pub static mut g_stat_star_cores_destroyed: u32 = 0;
	
	pub static mut g_y_sum: f32 = 0.0;
	pub static mut g_y_sum_count: u32 = 0;
}

mod math;
mod graphics;
mod game;

use consts::*;
use math::*;
use graphics::*;
use globals::*;
use game::*;
use rand::*;

use minifb::{Window, WindowOptions, Scale, MouseMode, MouseButton, Key, KeyRepeat};

fn main() {
  
	let mut window = match Window::new(TITLE, WIDTH + DEBUG_WIDTH, HEIGHT, WindowOptions{
		borderless: true, title: true, resize: false, scale: Scale::X2}) {
		Ok(win) => win,
			Err(err) => {
			println!("Unable to create window {}", err);
			return;
		}
	};	
	
	//This code just makes the mouse cursor not look like the window is loading all the time.
	window.set_cursor_style(minifb::CursorStyle::ClosedHand);
	window.set_cursor_style(minifb::CursorStyle::Arrow);	
	
	//Create image buffers
	let mut window_image = Image::new(WIDTH + DEBUG_WIDTH, HEIGHT, 0xffffff);
	let game_image = Image::new(GAME_WIDTH, GAME_HEIGHT, 0xffffff);
	let debug_image = Image::new(DEBUG_WIDTH, HEIGHT, 0);

	//Initialize globals
	unsafe { g_game_image      = Some(game_image)            																				   }; 
	unsafe { g_debug_image     = Some(debug_image)     		 																				   };
	unsafe { g_camera          = Some(Camera::new())   		 																				   };
	unsafe { g_set_camera      = Some(Camera::new())   		 																				   };
	unsafe { g_map             = Some(Image::new(0, 0, 0)) 																			  	   };
	unsafe { g_map_background  = Some(Image::new(0, 0, 0)) 																			  	   };
	unsafe { g_image_container = Some(ImageContainer::create()) 																			   };
	unsafe { g_entity_handler  = Some(EntityHandler::create())                                                                                 };	
	unsafe { g_font            = Some(Font::new("glyphs.png", "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890-:.,;+!></", 8, 10)) };
	unsafe { g_main_menu	   = Some(MainMenu::create())																						};
	let font            = unsafe{ g_font.as_ref().unwrap()            };
	let game_image      = unsafe{ g_game_image.as_mut().unwrap()      };
	let debug_image     = unsafe{ g_debug_image.as_mut().unwrap()     };
	let image_container = unsafe{ g_image_container.as_ref().unwrap() };
	let camera 			= unsafe{ g_camera.as_mut().unwrap()          };	
	let entity_handler  = unsafe{ g_entity_handler.as_mut().unwrap()  };
	let map             = unsafe{ g_map.as_ref().unwrap()             };
	let map_background  = unsafe{ g_map_background.as_ref().unwrap()  };
		
	//Create main menu
	let main_menu = unsafe{g_main_menu.as_mut().unwrap()};
	let mut exit_button = ExitButton::new(4, HEIGHT as i32 - 34, DEBUG_WIDTH as i32 - 8, 30);
	let mut restart_button = RestartButton::new(4, HEIGHT as i32 - 68, 32, 32);
	let mut restart_button_win = RestartButton::new((WIDTH + DEBUG_WIDTH) as i32/2 - 16, HEIGHT as i32*4/5 - 35, 32, 32);
	let mut next_button_win = NextButton::new((WIDTH + DEBUG_WIDTH) as i32/2 + 32, HEIGHT as i32*4/5 - 35, 32, 32);
	let mut exit_button_win = ExitButton::new((WIDTH + DEBUG_WIDTH) as i32/2 - 74, HEIGHT as i32*4/5 - 35, 42, 32);
	
	let mut bounce_bomb_button = BombButton::new(4, 4, 36, 36, image_container.bounce_bomb.clone(), BOMB_BOUNCE);
	let mut drop_bomb_button = BombButton::new(4, 44, 36, 36, image_container.bomb.clone(), BOMB_DROP);
	
		
	//Declare in-loop variables
	let mut b = false;
	let mut click_position = [0.0, 0.0];
	while window.is_open() {
		unsafe{g_y_sum = 0.0; g_y_sum_count = 0;}
			
		if unsafe{g_game_state} == STATE_MAIN_MENU {
			window_image.fill(0);
			main_menu.main_loop(&mut window_image, &mut window);
		} else if unsafe {g_game_state} == STATE_GAME {
			let camera = unsafe{ g_camera.as_mut().unwrap()};	
			unsafe {
			let s_camera = g_set_camera.as_ref().unwrap();
			camera.x = s_camera.x;
			camera.y = s_camera.y};
			
			let mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap();
			let mousex = mouse_pos.0;
			let mousey = mouse_pos.1;
	
			debug_image.fill(0xff << 24);
			game_image.draw_image(&(image_container.sky), 0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32);
			debug_image.draw_font(&font, !0, "Information", 1, 1, 1);

			debug_image.draw_font(&font, !0, format!("- Entity Count: {}", entity_handler.get_entity_count() as i32).as_str(), 1, 12, 1);
			debug_image.draw_font(&font, !0, format!("- Destroyed   : {}", unsafe {g_score}).as_str(), 1, 23, 1);
			debug_image.draw_font(&font, !0, "Level Information", 1, 78, 1);
			debug_image.draw_font(&font, !0, format!("PAR       : {}/{}", unsafe {g_stat_bombs_released}, unsafe{g_start_limit_bomb_count}).as_str(), 1, 89, 1);
			debug_image.draw_font(&font, !0, format!("CORES     : {}/{}", unsafe {g_stat_cores_destroyed}, unsafe{selected_level_core_count}).as_str(), 1, 100, 1);
			debug_image.draw_font(&font, !0, format!("STAR CORES: {}/{}", unsafe {g_stat_star_cores_destroyed}, unsafe{selected_level_star_core_count}).as_str(), 1, 111, 1);
			debug_image.draw_font(&font, !0, "Tips:", 1, 166, 1);
			let tips = unsafe {selected_level_tip.as_ref().unwrap()};
			for i in 0..tips.len() {
					debug_image.draw_font(&font, !0, tips[i].as_str(), 1, 166 + (i as i32 + 1)*11, 1);
			}
			window.get_keys().map(|keys| {
			let m_camera = unsafe{g_set_camera.as_mut().unwrap()};
  		  	for t in keys {
    	   		match t {
    	        		Key::Right => m_camera.x += 1.0,
    	        		Key::Left => m_camera.x -= 1.0,
						Key::Down => m_camera.y += 1.0,
   		         		Key::Up => m_camera.y -= 1.0,
    	        		_ => (),
    	    		}
    			}
			});
			
			exit_button.render(debug_image, mousex - WIDTH as f32, mousey);
			restart_button.render(debug_image, mousex - WIDTH as f32, mousey);
			
			if window.get_mouse_down(MouseButton::Left) {
				if !b {
					b = true;
					let mut e = false;
					exit_button.on_mouse_left_press(mousex - WIDTH as f32, mousey, &mut window);
					restart_button.on_mouse_left_press(mousex - WIDTH as f32, mousey, &mut window);
					if bounce_bomb_button.on_mouse_left_press(mousex, mousey, &mut window) { e = true; }
				 	if drop_bomb_button.on_mouse_left_press(mousex, mousey, &mut window) { e = true; }
					if !e && unsafe{g_stat_selected_bomb} != 0 {
						if mousey/2.0 + camera.y < 30.0 {
							click_position = [mousex/2.0 + camera.x, mousey/2.0 + camera.y];
						}
					}
				}
			} else {
				if b {
					b = false;
					bounce_bomb_button.on_mouse_left_release(mousex, mousey, &mut window);
					drop_bomb_button.on_mouse_left_release(mousex, mousey, &mut window);
					exit_button.on_mouse_left_release(mousex - WIDTH as f32, mousey, &mut window);
					restart_button.on_mouse_left_release(mousex - WIDTH as f32, mousey, &mut window);
					if click_position != [0.0, 0.0] {
						let mx = mousex/2.0 + camera.x;
						let my = mousey/2.0 + camera.y;
						let mut vx = (click_position[0] - mx)/30.0;
						let mut vy = (click_position[1] - my)/30.0;
						vx = clampf32(vx, -3.0, 3.0);
						vy = clampf32(vy, -3.0, 3.0);
						let mut e = true;
						match unsafe{g_stat_selected_bomb} {
							BOMB_BOUNCE => {entity_handler.add_bounce_bomb(BounceBomb::new(click_position[0], click_position[1], vx, vy));},
							BOMB_DROP   => {entity_handler.add_bomb(Bomb::new(click_position[0], click_position[1], vx, vy));},
							_ => { e = false; }
						}
						if e {
							unsafe { g_stat_bombs_released += 1 };
						}
						click_position = [0.0, 0.0];
					}
				}
			}
			
		//	game_image.draw_image(map_background, -(camera.x) as i32, -(camera.y) as i32, map.get_width() as i32, map.get_height() as i32);	
			game_image.draw_image(map, -(camera.x) as i32, -(camera.y) as i32, map.get_width() as i32, map.get_height() as i32);	

			entity_handler.update();
			entity_handler.render(game_image, camera);
			unsafe {
				if g_y_sum_count == 0 {
					g_y_sum = 0.0;
					g_y_sum_count = 1;
				}
				g_set_camera.as_mut().unwrap().y = clampf32(((g_y_sum/g_y_sum_count as f32) + camera.y*1.0)/2.0 - 40.0, -50.0, HEIGHT as f32 - map.get_height() as f32 - 41.0);
			}
			game_image.draw_line(!0, 0, 30-(camera.y) as i32, GAME_WIDTH as i32, 30-(camera.y) as i32);	
		
			if click_position != [0.0, 0.0] {
				match unsafe{g_stat_selected_bomb} {
					BOMB_BOUNCE => {game_image.fill_circle(!0, (click_position[0] - camera.x) as i32, (click_position[1] - camera.y) as i32, 5);},
					BOMB_DROP => {
						game_image.fill_circle(!0, (click_position[0] - camera.x) as i32, (click_position[1] - camera.y) as i32, 4);
						game_image.fill_rect(!0, (click_position[0] - camera.x) as i32 - 3, (click_position[1] - camera.y) as i32 - 5, 7, 1);
						game_image.fill_rect(!0, (click_position[0] - camera.x) as i32 - 1, (click_position[1] - camera.y) as i32 - 4, 3, 1);
						game_image.fill_rect(!0, (click_position[0] - camera.x) as i32 - 2, (click_position[1] - camera.y) as i32 - 3, 5, 1);
					},
					_ => {}
				}
				game_image.draw_line(!0, (click_position[0] - camera.x)as i32, (click_position[1] - camera.y) as i32, (mousex/2.0) as i32, (mousey/2.0) as i32);
			}
			
			if unsafe {g_stat_cores_destroyed >= selected_level_core_count} {
				unsafe{
					g_game_state = STATE_END;
				}
			}
				
			window_image.draw_image(&game_image, 0, 0, WIDTH as i32, HEIGHT as i32);
			bounce_bomb_button.render(&mut window_image, mousex, mousey);
			drop_bomb_button.render(&mut window_image, mousex, mousey);
			
			window_image.draw_image(&debug_image, WIDTH as i32, 0, DEBUG_WIDTH as i32, HEIGHT as i32);
		} else if unsafe{g_game_state} == STATE_LOADING {
			window_image.fill(0xff << 24);
			let text = "Loading...";
			let size = 2;
			window_image.draw_font(font, !0, text, ((WIDTH + DEBUG_WIDTH) as i32 - font.get_text_width(text, size) as i32)/2, (HEIGHT - font.get_height()*size) as i32 - 10, size);
			window.update_with_buffer(window_image.get_buffer()).unwrap();
			if unsafe{g_loading_mode} == LOADING_MAP {
				let cores = unsafe{selected_level_cores.as_ref().unwrap()};
				let star_cores = unsafe{selected_level_star_cores.as_ref().unwrap()};
			
				click_position = [0.0, 0.0];
				unsafe{
					g_game_state = STATE_GAME;
					g_entity_handler = Some(EntityHandler::create());
					g_camera = Some(Camera::new());
					g_map = Some(Image::load(selected_level_name.as_ref().unwrap()));
					entity_handler.refresh_time();
					reset_global_level_stats();
					for	i in 0..cores.len() {
						entity_handler.add_core(Core::new(cores[i][0] as f32, cores[i][1] as f32));	
					}
					for	i in 0..star_cores.len() {
						entity_handler.add_star_core(StarCore::new(star_cores[i][0] as f32, star_cores[i][1] as f32));	
					}
				}
			} else if unsafe{g_loading_mode} == LOADING_MENU {
				unsafe{
					g_game_state = STATE_MAIN_MENU;
				}
			}
		} else if unsafe{g_game_state} == STATE_END {
			let mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap();
			let mousex = mouse_pos.0;
			let mousey = mouse_pos.1;
	
			window_image.draw_image(&game_image, 0, 0, WIDTH as i32, HEIGHT as i32);
			window_image.draw_image(&debug_image, WIDTH as i32, 0, DEBUG_WIDTH as i32, HEIGHT as i32);
			
			window_image.draw_rect(!0, (WIDTH + DEBUG_WIDTH) as i32/5, HEIGHT as i32/5,  (WIDTH + DEBUG_WIDTH) as i32*3/5, HEIGHT as i32*3/5);
			window_image.fill_rect(0xff << 24, (WIDTH + DEBUG_WIDTH) as i32/5 + 1, HEIGHT as i32/5 + 1,  (WIDTH + DEBUG_WIDTH) as i32*3/5 - 2, HEIGHT as i32*3/5 - 2);
			
			window_image.draw_image(unsafe { 
				if g_stat_star_cores_destroyed >= selected_level_star_core_count || level_stars.as_mut().unwrap()[selected_level as usize][2] {
					level_stars.as_mut().unwrap()[selected_level as usize][2] = true;
					&(image_container.star_filled)	
				} else {
					&(image_container.star_empty)
				}
			}, (WIDTH + DEBUG_WIDTH) as i32/2 + 40, HEIGHT as i32/5 + 30, 60, 60);
			window_image.draw_image(&(image_container.star_core), (WIDTH + DEBUG_WIDTH) as i32/2 + 80, HEIGHT as i32/5 + 70, 25, 25);
		
			unsafe {level_stars.as_mut().unwrap()[selected_level as usize][0] = true};
			window_image.draw_image(unsafe { 
				if g_stat_bombs_released <= g_start_limit_bomb_count || level_stars.as_mut().unwrap()[selected_level as usize][1] {
					level_stars.as_mut().unwrap()[selected_level as usize][1] = true;
					&(image_container.star_filled)	
				} else {
					&(image_container.star_empty)
				}
			}, (WIDTH + DEBUG_WIDTH) as i32/2 - 30, HEIGHT as i32/5 + 10, 60, 60);
		
			window_image.draw_image(&(image_container.star_filled), (WIDTH + DEBUG_WIDTH) as i32/2 - 100, HEIGHT as i32/5 + 30, 60, 60);
			window_image.draw_image(&(image_container.core), (WIDTH + DEBUG_WIDTH) as i32/2 - 60, HEIGHT as i32/5 + 70, 25, 25);
			
			let text = "Par";
			let size = 1;
			window_image.draw_font(font, !0, text, ((WIDTH + DEBUG_WIDTH) as i32 - font.get_text_width(text, size) as i32)/2, (HEIGHT - font.get_height()*size) as i32/5 + 74, size);
			restart_button_win.render(&mut window_image, mousex, mousey);
			exit_button_win.render(&mut window_image, mousex, mousey);
			if unsafe {selected_level < level_count - 1} {next_button_win.render(&mut window_image, mousex, mousey);}
		
			if window.get_mouse_down(MouseButton::Left) {
				if !b {
					b = true;
					restart_button_win.on_mouse_left_press(mousex, mousey, &mut window);
					exit_button_win.on_mouse_left_press(mousex, mousey, &mut window);
					if unsafe {selected_level < level_count - 1} {next_button_win.on_mouse_left_press(mousex, mousey, &mut window);}
				}
			} else {
				if b {
					b = false;
					restart_button_win.on_mouse_left_release(mousex, mousey, &mut window);
					exit_button_win.on_mouse_left_release(mousex, mousey, &mut window);
					if unsafe {selected_level < level_count - 1} {next_button_win.on_mouse_left_release(mousex, mousey, &mut window);}
				}
			}
		}

		window.update_with_buffer(window_image.get_buffer()).unwrap();
	}
	
}

fn reset_global_level_stats() {
	unsafe {
		g_stat_selected_bomb = 0;
		g_stat_bombs_released = 0;
		g_stat_cores_destroyed = 0;
		g_stat_star_cores_destroyed = 0;
		g_score = 0;
	}
}
