use shapekit::{shape::Shape, vector::Vector};

use lame::{entity::Entity, world::World};

use ggez::graphics;

#[derive(Clone)]
pub struct Rect {
    pub hitbox: Shape,
    pub velocity: Vector,
    pub visible: bool,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32,v: Vector, vi: bool) -> Rect {
        let hbox = Shape::new(vec!(Vector::new(x, y), Vector::new(x + w, y), Vector::new(x + w, y + h), Vector::new(x, y + h)), Vector::new(x + (w/2.0), y + (h/2.0)));
        Rect{hitbox: hbox, velocity: v, visible: vi}
    }
    pub fn snapshot(&self) -> graphics::Rect {
        let zero = self.hitbox.get_point(0);
        let offsets = self.hitbox.get_point(2) - zero;
        graphics::Rect{x: zero.x, y: zero.y, w: offsets.x, h: offsets.y}
    }
    pub fn new_square(x: f32, y: f32) -> Rect {
        let start = Shape::new(vec![Vector::new(0.0, 0.0), Vector::new(50.0, 0.0), Vector::new(50.0, 50.0), Vector::new(0.0, 50.0)], Vector::new(x, y));
        Rect{hitbox: start, velocity: Vector::new(250.0, 250.0), visible: true}
    }
}

impl Entity for Rect {
    fn update(entity_num: usize, world: &World<Self>, delta: f32) {
        let mut clone = world.read_list()[entity_num].read().unwrap().clone();
        if clone.visible {
            clone.hitbox.move_by(clone.velocity * delta);
            for (i, ent) in world.read_list().iter().enumerate() {
                if i == entity_num {
                    continue;
                }
                let entity = ent.read().unwrap();
                if let Some(res) = clone.hitbox.resolve(&entity.hitbox) {
                    if res.x < 0.0 {
                        clone.velocity.x = clone.velocity.x.abs() * -1.0;
                    } else if res.x > 0.0 {
                        clone.velocity.x = clone.velocity.x.abs();
                    }
                    if res.y < 0.0 {
                        clone.velocity.y = clone.velocity.y.abs() * -1.0;
                    } else if res.y > 0.0 {
                        clone.velocity.y = clone.velocity.y.abs();
                    }
                }
            }
        }
        let list = world.read_list();
        let mut entity = list[entity_num].write().unwrap();
        *entity = clone;
    }
}