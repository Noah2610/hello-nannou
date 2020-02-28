#[macro_use]
extern crate derive_builder;
extern crate nannou;
extern crate rand;

use nannou::draw::Draw;
use nannou::prelude::*;
use rand::prelude::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
struct Block {
    pos:   Point2,
    size:  Vector2,
    color: Rgb,
}

impl Block {
    pub fn draw(&self, draw: &Draw) {
        draw.rect().color(self.color).xy(self.pos).wh(self.size);
    }
}

#[derive(Default)]
struct Model {
    blocks: Vec<Block>,
}

impl Model {
    pub fn new(_app: &App) -> Self {
        let mut rng = rand::thread_rng();

        let gen_coord =
            |rng: &mut ThreadRng| -> f32 { rng.gen_range(-300.0, 300.0) };
        let gen_size =
            |rng: &mut ThreadRng| -> f32 { rng.gen_range(-50.0, 50.0) };
        let gen_color = |rng: &mut ThreadRng| -> Rgb {
            Rgb::new(
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
            )
        };

        let mut model = Self::default();

        for i in 0 .. 50 {
            let i = i as f32;
            model.blocks.push(
                BlockBuilder::default()
                    .pos(Point2::new(i * 20.0, i * 10.5))
                    .size(Vector2::new(10.0 + i * 3.0, 7.0 + i * 4.0))
                    .color(gen_color(&mut rng))
                    .build()
                    .unwrap(),
            );

            model.blocks.push(
                BlockBuilder::default()
                    .pos(Point2::new(gen_coord(&mut rng), gen_coord(&mut rng)))
                    .size(Vector2::new(gen_size(&mut rng), gen_size(&mut rng)))
                    .color(gen_color(&mut rng))
                    .build()
                    .unwrap(),
            );
        }

        model
    }
}

fn main() {
    nannou::app(Model::new)
        .update(update)
        .simple_window(render)
        .run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    let mut rng = rand::thread_rng();

    let mouse = &app.mouse;
    let mouse_pos = Point2::new(mouse.x, mouse.y);

    let gen_coord =
        |rng: &mut ThreadRng| -> f32 { rng.gen_range(-100.0, 100.0) * dt };
    let gen_size =
        |rng: &mut ThreadRng| -> f32 { rng.gen_range(-30.0, 30.0) * dt };
    let gen_move_mult =
        |rng: &mut ThreadRng| -> f32 { rng.gen_range(0.0, 200.0) * dt };

    for block in model.blocks.iter_mut() {
        // jitter around
        block.pos.x += gen_coord(&mut rng);
        block.pos.y += gen_coord(&mut rng);
        block.size.x += gen_size(&mut rng);
        block.size.y += gen_size(&mut rng);

        // follow mouse cursor
        if mouse.window.is_some() {
            let mult = if mouse.buttons.left().is_down()
                && block.pos.distance(mouse_pos) < 100.0
            {
                // push block away from cursor
                -gen_move_mult(&mut rng)
            } else {
                gen_move_mult(&mut rng)
            };

            block.pos.x += (mouse.x - block.pos.x).signum() * mult;
            block.pos.y += (mouse.y - block.pos.y).signum() * mult;
        }
    }
}

fn render(app: &App, model: &Model, frame: &Frame) {
    // https://guide.nannou.cc/tutorials/basics/drawing-2d-shapes.html

    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(DARKGRAY);

    for block in model.blocks.iter() {
        block.draw(&draw);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
