pub trait Entity {
	///Return false with update if entity dies.
	fn update(&mut self) -> bool;
	fn kill(&mut self); 
	fn render(&mut self, screen: &mut Image, camera: &mut Camera);
}