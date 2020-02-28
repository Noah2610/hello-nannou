#[macro_use]
extern crate derive_builder;
extern crate nannou;
extern crate rand;

use nannou::prelude::*;
use rand::prelude::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
struct Block {
    pos:  Point2,
    size: Vector2,
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

        let mut model = Self::default();

        for i in (0 .. 10) {
            let i = i as f32;
            model.blocks.push(
                BlockBuilder::default()
                    .pos(Point2::new(i * 20.0, i * 10.5))
                    .size(Vector2::new(10.0 + i * 3.0, 7.0 + i * 4.0))
                    .build()
                    .unwrap(),
            );

            model.blocks.push(
                BlockBuilder::default()
                    .pos(Point2::new(gen_coord(&mut rng), gen_coord(&mut rng)))
                    .size(Vector2::new(gen_size(&mut rng), gen_size(&mut rng)))
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

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    let mut rng = rand::thread_rng();

    let gen_coord =
        |rng: &mut ThreadRng| -> f32 { rng.gen_range(-10.0, 10.0) * dt };
    let gen_size =
        |rng: &mut ThreadRng| -> f32 { rng.gen_range(-3.0, 3.0) * dt };

    for block in model.blocks.iter_mut() {
        block.pos.x += gen_coord(&mut rng);
        block.pos.y += gen_coord(&mut rng);
        block.size.x += gen_size(&mut rng);
        block.size.y += gen_size(&mut rng);
    }
}

fn render(app: &App, model: &Model, frame: &Frame) {
    // https://guide.nannou.cc/tutorials/basics/drawing-2d-shapes.html

    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(DARKGRAY);

    for block in model.blocks.iter() {
        draw.rect().color(RED).xy(block.pos).wh(block.size);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
