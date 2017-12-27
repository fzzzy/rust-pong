extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{ Events, EventSettings };
use piston::input::{ Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent };
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,
    leftscore: i32,
    leftpos: i32,
    leftvel: i32,
    rightscore: i32,
    rightpos: i32,
    rightvel: i32,
    ballx: i32,
    bally: i32,
    velx: i32,
    vely: i32
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FG: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let leftpos = self.leftpos as f64;
        let right = rectangle::square(0.0, 0.0, 50.0);
        let rightpos = self.rightpos as f64;
        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ballx = self.ballx as f64;
        let bally = self.bally as f64;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG, gl);
            rectangle(FG, left, c.transform
                .trans(-40.0, leftpos), gl);
            rectangle(FG, right, c.transform
                .trans(args.width as f64 - 10.0, rightpos), gl);
            rectangle(FG, ball, c.transform
                .trans(
                    ballx,
                    bally), gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if (self.leftvel == 1 && self.leftpos < 291)
            || (self.leftvel == -1 && self.leftpos >= 1) {
            self.leftpos += self.leftvel;
        }
        if (self.rightvel == 1 && self.rightpos < 291)
            || (self.rightvel == -1 && self.rightpos >= 1) {
            self.rightpos += self.rightvel;
        }
        self.ballx += self.velx;
        if self.ballx > 502 {
            self.velx = - self.velx;
            if self.bally < self.rightpos || self.bally > self.rightpos + 50 {
                self.leftscore += 1;
                println!("Left point! {} vs {}", self.leftscore, self.rightscore);
                if self.leftscore >= 5 {
                    println!("Left wins!");
                    process::exit(0);
                }
                self.ballx = 256;
                self.bally = 171;
            }
        }
        if self.ballx < 1 {
            self.velx = - self.velx;
            if self.bally < self.leftpos || self.bally > self.leftpos + 50 {
                self.rightscore += 1;
                println!("Right point! {} vs {}", self.leftscore, self.rightscore);
                if self.rightscore >= 5 {
                    println!("Right wins!");
                    process::exit(0);
                }
                self.ballx = 256;
                self.bally = 171;
            }
        }
        self.bally += self.vely;
        if self.bally > 332  || self.bally < 1 {
            self.vely = - self.vely;
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args  {
            match key {
                Key::Up => {
                    self.rightvel = -1;
                }
                Key::Down => {
                    self.rightvel = 1;
                }
                Key::W => {
                    self.leftvel = -1;
                }
                Key::S => {
                    self.leftvel = 1;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args  {
            match key {
                Key::Up => {
                    self.rightvel = 0;
                }
                Key::Down => {
                    self.rightvel = 0;
                }
                Key::W => {
                    self.leftvel = 0;
                }
                Key::S => {
                    self.leftvel = 0;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rust-piston-hello", [512, 342])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        leftscore: 0,
        leftpos: 1,
        leftvel: 0,
        rightscore: 0,
        rightpos: 1,
        rightvel: 0,
        ballx: 0,
        bally: 0,
        velx: 1,
        vely: 1
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}

