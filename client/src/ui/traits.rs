use ui::utils::{ Dimension, EventResult };
use ui::render_state::UIRender;
use glium::glutin::Event;

pub trait UIElement {
	fn get_initial_position(&self, parent_dimensions: &Dimension) -> (u32, u32);
	fn get_desired_size(&self, parent_dimensions: &Dimension) -> (u32, u32);
	fn draw(&self, render: &mut UIRender);
	fn update(&mut self, delta_time: f32);
	fn handle_event(&mut self, ev: &Event) -> EventResult;
	fn click(&mut self) -> EventResult;
	fn set_focus(&mut self) -> bool;

}
