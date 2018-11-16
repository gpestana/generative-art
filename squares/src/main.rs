extern crate cairo;
extern crate rand;

use cairo::{Context, Format, ImageSurface};
use rand::random;
use rand::Rng;
use std::fs::File;

const CANVAS_WIDTH: i32 = 1200;
const CANVAS_HEIGHT: i32 = 1200;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, CANVAS_WIDTH, CANVAS_HEIGHT)
        .expect("Err creating surface");
    let ctx = Context::new(&surface);

    // background
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint();

    let width = 40.0;
    let height = 40.0;

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    for j in 0..50 {
        for i in 0..100 {
            // generates square at random
            if rand::random() {
                let x = 10.0 + (i as f64 * 35 as f64);
                let y = 10.0 + (j as f64 * 35 as f64);

                // generate square
                create_rectangle(&ctx, x, y, width, height);

                // color square
                let (r, b, g, a) = generate_color_pallete();
                ctx.set_source_rgba(r, b, g, a);
                ctx.fill();
            }
        }
    }

    ctx.stroke();

    let mut f = File::create("squares.png").expect("Err creating new file");
    surface.write_to_png(&mut f).expect("Err writing to file")
}

#[allow(dead_code)]
fn generate_color_pallete() -> (f64, f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let pallete_1 = vec![0.153, 0.153, 0.153, 0.80];
    let pallete_2 = vec![0.17, 0.17, 0.17, 0.80];
    let pallete_3 = vec![0.85, 0.85, 0.85, 0.80];
    let palletes = vec![&pallete_1, &pallete_2, &pallete_3];
    let p = rng.gen_range(0, 3);
    (
        palletes[p][0],
        palletes[p][1],
        palletes[p][2],
        palletes[p][3],
    )
}

fn create_rectangle(ctx: &Context, x: f64, y: f64, width: f64, height: f64) {
    let mut rng = rand::thread_rng();
    let (r1, r2, r3) = (
        rng.gen_range(-5, 5),
        rng.gen_range(-5, 5),
        rng.gen_range(-5, 5),
    );
    ctx.move_to(x, y);
    ctx.rel_line_to(width, r1 as f64);
    ctx.rel_line_to(r2 as f64, height);
    ctx.rel_line_to(-width, r3 as f64);
    ctx.close_path();
}
