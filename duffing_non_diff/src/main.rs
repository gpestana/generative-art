#[allow(dead_code)]
extern crate cairo;
extern crate rand;

use cairo::{Context, Format, ImageSurface};
use rand::Rng;
use std::f64;
use std::fs::File;
use std::time::SystemTime;

const CANVAS_WIDTH: i32 = 1200;
const CANVAS_HEIGHT: i32 = 1200;

// chaotic constants
const CHAOS_A: f64 = 2.75;
const CHAOS_B: f64 = 0.2;

const COEF: f64 = 500.0;
const JUMP_COEF: u8 = 5;
const SEED: f64 = 1.0;

const N_STRUCTS: u8 = 10;

fn main() {
    let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    let surface = ImageSurface::create(Format::ARgb32, CANVAS_WIDTH, CANVAS_HEIGHT)
        .expect("Err creating surface");
    let ctx = Context::new(&surface);

    // background
    //ctx.set_source_rgb(250.0, 250.0, 250.0);
    //ctx.paint();

    for _i in 0..N_STRUCTS {
        let (xi, yi) = init_generator();
        construct_struct(&ctx, xi, yi);
    }

    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke();

    let fname: String = String::from(format!("duffing_{:?}.png", start_time.unwrap()));
    let mut f = File::create(fname).expect("Err creating new file");
    surface.write_to_png(&mut f).expect("Err writting to file");
}

fn rnd_generator(seed: f64) -> (f64, f64) {
    (seed * 1 as f64, seed * 1 as f64)
}

fn init_generator() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0, 1200) as f64, rng.gen_range(1, 1200) as f64)
}

fn construct_struct(ctx: &Context, xi: f64, yi: f64) {
    // x_n+1 = y_n
    // y_n+1 = -b x_n + a y_n - y_n^3
    ctx.move_to(xi, yi);
    let (mut x, mut y) = rnd_generator(SEED);
    for i in 0..1000 {
        if i % JUMP_COEF == 0 {
            x = y;
            y = CHAOS_B * -1.0 + CHAOS_A * y - f64::powi(y, 3);
            print!("[{} {}] ", x * COEF, y * COEF);
            ctx.line_to(x as f64 * COEF, y as f64 * COEF);
            ctx.close_path();
        }
    }
}
