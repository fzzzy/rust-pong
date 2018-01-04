extern crate music;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::char;
use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{ Events, EventSettings };
use piston::input::{ Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent };
use glutin_window::GlutinWindow;
use graphics::character::CharacterCache;
use opengl_graphics::{ Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings };

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Pong,
    Buzzer,
}

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
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
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
        let left_char = match char::from_digit(self.leftscore as u32, 10) {
            Some(c) => c,
            None => ' '
        };
        let right_char = match char::from_digit(self.rightscore as u32, 10) {
            Some(c) => c,
            None => ' '
        };
        let score = Image::new_color(FG);
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
            if let Ok(left_character) = glyphs.character(36, left_char) {
                score.draw(left_character.texture,
                    &c.draw_state,
                    c.transform.trans(20.0, 20.0),
                    gl);
            }
            if let Ok(right_character) = glyphs.character(36, right_char) {
                score.draw(right_character.texture,
                    &c.draw_state,
                    c.transform.trans(475.0, 20.0),
                    gl);
            }
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
                music::play_sound(&Sound::Buzzer, music::Repeat::Times(0), music::MAX_VOLUME);
                self.leftscore += 1;
                if self.leftscore >= 5 {
                    println!("Left wins!");
                    process::exit(0);
                }
                self.ballx = 256;
                self.bally = 171;
            } else {
                music::play_sound(&Sound::Pong, music::Repeat::Times(0), music::MAX_VOLUME);
            }
        }
        if self.ballx < 1 {
            self.velx = - self.velx;
            if self.bally < self.leftpos || self.bally > self.leftpos + 50 {
                music::play_sound(&Sound::Buzzer, music::Repeat::Times(0), music::MAX_VOLUME);
                self.rightscore += 1;
                if self.rightscore >= 5 {
                    println!("Right wins!");
                    process::exit(0);
                }
                self.ballx = 256;
                self.bally = 171;
            } else {
                music::play_sound(&Sound::Pong, music::Repeat::Times(0), music::MAX_VOLUME);
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

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("src/FiraMono-Regular.ttf", (), texture_settings)
        .expect("Could not load font");


    let mut app = App {
        gl: GlGraphics::new(opengl),
        leftscore: 0,
        leftpos: 146,
        leftvel: 0,
        rightscore: 0,
        rightpos: 146,
        rightvel: 0,
        ballx: 0,
        bally: 0,
        velx: 1,
        vely: 1
    };

    music::start::<Music, Sound, _>(16, || {
        music::bind_sound_file(Sound::Pong, "src/pong.wav");
        music::bind_sound_file(Sound::Buzzer, "src/buzzer_x.wav");
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                app.render(&r, glyphs);
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
    });
}

