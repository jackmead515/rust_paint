extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use graphics::Context;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod shapes;
use shapes::{Rect, Point};

struct World {
  objects: Vec<Box<Rect>>,
  points: Vec<Box<Point>>
}

struct Input {
  mouse_pressed: bool,
  prev_mouse_position: [f64; 2],
  mouse_position: [f64; 2]
}

struct Engine {
  gl: GlGraphics,
  world: World,
  input: Input
}

impl Engine {

  fn render(&mut self, args: &RenderArgs) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    
    let context = self.gl.draw_begin(args.viewport());

    graphics::clear(BLACK, &mut self.gl);

    for object in self.world.objects.iter_mut() {
      object.render(args, context, &mut self.gl);
    }
    for point in self.world.points.iter_mut() {
      point.render(args, context, &mut self.gl);
    }

    self.gl.draw_end();
  }

  fn update(&mut self, args: &UpdateArgs) {
    for object in self.world.objects.iter_mut() {
      object.update(args);
    } 

    let pmp = self.input.prev_mouse_position;
    let mp = self.input.mouse_position;
    let mpb = self.input.mouse_pressed;

    if mpb && pmp[0] != mp[0] && pmp[1] != mp[1] {
      self.world.points.push(Box::new(Point {
        x: mp[0],
        y: mp[1],
        color: [1.0, 0.0, 0.0, 1.0]
      }));
    }
  }

}

fn main() {
  let opengl = OpenGL::V3_2;
  let title = "Rust Paint";
  let window_size = [800, 600];

  let mut window: Window = WindowSettings::new(title, window_size)
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
  
  let mut engine = Engine {
    world: World {
      objects: Vec::new(),
      points: Vec::new()
    },
    input: Input {
      mouse_pressed: false,
      prev_mouse_position: [-1.0, -1.0],
      mouse_position: [-1.0, -1.0]
    },
    gl: GlGraphics::new(opengl)
  };

  engine.world.objects.push(Box::new(Rect {
    x: 0.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
    rotation: 0.0,
    color: [1.0, 0.0, 0.0, 1.0]
  }));

  engine.world.points.push(Box::new(Point {
    x: 20.0,
    y: 20.0,
    color: [1.0, 0.0, 0.0, 1.0]
  }));

  let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
      if let Some(r) = e.render_args() {
        engine.render(&r);
      }

      if let Some(u) = e.update_args() {
        engine.update(&u);
      }

      if let Some(button) = e.press_args() {
        if button == Button::Mouse(MouseButton::Left) {
          engine.input.mouse_pressed = true;
        }
      }

      if let Some(button) = e.release_args() {
        if button == Button::Mouse(MouseButton::Left) {
          engine.input.mouse_pressed = false;
        }
      }

      if engine.input.mouse_pressed {
        if let Some(position) = e.mouse_cursor_args() {
          engine.input.prev_mouse_position = engine.input.mouse_position;
          engine.input.mouse_position = [position[0] as f64, position[1] as f64];
        }
      }
    }
}
