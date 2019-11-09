extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use graphics::Context;
use opengl_graphics::{ GlGraphics };

pub struct Rect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub rotation: f64,
  pub color: [f32; 4]
}

impl Rect {
  pub fn render(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
    use graphics::*;

    let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
    let shape = [0.0, 0.0, self.width, self.height];

    let transform = c.transform
      .trans(x, y)
      .rot_rad(self.rotation)
      .trans(-self.width/2.0, -self.height/2.0);

    rectangle(self.color, shape, transform, gl);
  }

  pub fn update(&mut self, args: &UpdateArgs) {
    self.rotation += 0.5 * args.dt;
  }
}

pub struct Point {
  pub x: f64,
  pub y: f64,
  pub color: [f32; 4]
}

impl Point {
  pub fn render(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
    use graphics::*;

    let shape = [0.0, 0.0, 1.0, 1.0];

    let transform = c.transform
      .trans(self.x, self.y);

    rectangle(self.color, shape, transform, gl);
  }

  pub fn update(&mut self, args: &UpdateArgs) {
  
  }
}

