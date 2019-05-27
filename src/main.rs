use ggez::*;
use ggez::graphics::{DrawMode};
use shapekit::{shape::Shape, vector::Vector};
use lame::{entity::Entity, world::World};

mod square;

struct MainState {
    world: World<square::Rect>
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {world: init()};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for i in 0..self.world.read_list().len() {
            let list = self.world.read_list();
            let rec = list[i].read().unwrap();
            if !rec.visible {
                continue;
            }
            graphics::rectangle(ctx, DrawMode::Fill, rec.snapshot())?;
        }
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

fn init() -> World<square::Rect> {
    let world = World::new();
    world.push(square::Rect::new(-100.0, -100.0, 1000.0, 100.0, Vector::new(0.0, 0.0), false));
    world.push(square::Rect::new(-100.0, 0.0, 100.0, 600.0, Vector::new(0.0, 0.0), false));
    world.push(square::Rect::new(-100.0, 600.0, 1000.0, 100.0, Vector::new(0.0, 0.0), false));
    world.push(square::Rect::new(800.0, 0.0, 100.0, 600.0, Vector::new(0.0, 0.0), false));
    for _i in 0..400 {
        world.push(square::Rect::new_square(400.0, 250.0));
        world.push(square::Rect::new_square(400.0, 200.0));
    }
    world.start();
    world
}