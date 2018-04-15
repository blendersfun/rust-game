extern crate ggez;
use std::path::PathBuf;
use std::env;

use ggez::{
    Context,
    ContextBuilder,
    error::GameResult,
    audio,
    graphics::{
        self,
        Color,
    },
    event::{
        EventHandler,
        Keycode,
        Mod,
    },
    conf,
};

struct MyState {
    running: bool,
    background: Color,
    keys_down: u32,
    key_clicks: u32,
    music: audio::Source,
    image: graphics::Image,
}

impl EventHandler for MyState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.running {
            self.music.play().unwrap();
            self.running = true;
        }
        if self.key_clicks == 10 {
            ctx.quit().unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, self.background);
        graphics::clear(ctx);

        // Draw a beautiful kitten:
        let dest_point = if self.keys_down == 0 {
            graphics::Point2::new(50.0, 50.0)
        } else {
            graphics::Point2::new(150.0, 150.0)
        };
        graphics::draw(ctx, &self.image, dest_point, 0.0).unwrap();

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        repeat: bool,
    ) {
        if !repeat {
            self.keys_down += 1;
            self.background = match keycode {
                Keycode::R => Color::new(0.5, 0.0, 0.0, 1.0),
                Keycode::G => Color::new(0.0, 0.5, 0.0, 1.0),
                Keycode::B => Color::new(0.0, 0.0, 0.5, 1.0),
                _ => self.background,
            };
        }
        if keycode == Keycode::Escape {
            ctx.quit().unwrap();
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        self.background = Color::new(0.1, 0.1, 0.1, 1.0);
        self.keys_down -= 1;
        self.key_clicks += 1;
    }
}

fn main() {
    let path: PathBuf = [&env::var("CARGO_MANIFEST_DIR").unwrap(), "resources"].iter().collect();
    let ctx = &mut ContextBuilder::new("Excellent Color Game", "Anonymous")
        .add_resource_path(path)
        .window_setup(conf::WindowSetup::default().title("Excellent Color Game"))
        .window_mode(conf::WindowMode::default().fullscreen_type(conf::FullscreenType::Desktop))
        .build().unwrap();

    let res = *graphics::get_fullscreen_modes(ctx, 0).unwrap().first().unwrap();
    // graphics::set_resolution(ctx, res.0, res.1).unwrap();
    // graphics::set_fullscreen(ctx, true).unwrap();
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, res.0 as f32, res.1 as f32)).unwrap();
    println!("{:?}", res);

    let mut state = MyState {
        running: false,
        background: Color::new(0.1, 0.1, 0.1, 1.0),
        key_clicks: 0,
        keys_down: 0,
        music: audio::Source::new(ctx, "/celeste/Lena Raine - Celeste Original Soundtrack - 01 Prologue.ogg").unwrap(),
        image: graphics::Image::new(ctx, "/beautiful-kitten.jpg").unwrap(),
    };

    ggez::event::run(ctx, &mut state).unwrap();
}
