#[allow(dead_code)]
extern crate cairo;
extern crate rand;

use cairo::{Context, Format, ImageSurface};
use rand::distributions::{Distribution, Uniform};
use std::fmt;
use std::fs::File;
use std::time::SystemTime;
use std::vec::Vec;

const CANVAS_WIDTH: i32 = 1200;
const CANVAS_HEIGHT: i32 = 1200;
const SMOOTH_FACTOR: f64 = -10.0;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    let surface = ImageSurface::create(Format::ARgb32, CANVAS_WIDTH, CANVAS_HEIGHT)
        .expect("Err creating surface");
    let ctx = Context::new(&surface);

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.paint();

    let data: Vec<Point> = vec![
        Point { x: 10.0, y: 10.0 },
        Point { x: 20.0, y: 60.0 },
        Point { x: 70.0, y: 70.0 },
        Point { x: 80.0, y: 80.0 },
        Point { x: 120.0, y: 80.0 },
        Point { x: 500.0, y: 90.0 },
        Point { x: 700.0, y: 220.0 },
    ];

    let ctx = fitting_path(data, SMOOTH_FACTOR, ctx.clone());
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke();

    let data: Vec<Point> = vec![
        Point { x: 20.0, y: 20.0 },
        Point { x: 79.0, y: 70.0 },
        Point { x: 100.0, y: 30.0 },
        Point { x: 80.0, y: 80.0 },
        Point { x: 150.0, y: 120.0 },
        Point {
            x: 1000.0,
            y: 190.0,
        },
        Point {
            x: 1200.0,
            y: 1200.0,
        },
    ];

    let ctx = fitting_path(data, SMOOTH_FACTOR, ctx.clone());
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke();

    let gen_points =
        gen_random_uniform_dist(Point { x: 1.0, y: 1.0 }, Point { x: 1103.0, y: 2.0 }, 15);
    println!("{:?}", gen_points);
    let ctc = fitting_path(gen_points, SMOOTH_FACTOR, ctx.clone());
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke();

    let fname: String = String::from(format!("paths_{:?}.png", start_time.unwrap()));
    let mut f = File::create(fname).expect("Err creating new file");
    surface.write_to_png(&mut f).expect("Err writing to file");
}

fn fitting_path(points: Vec<Point>, smooth_f: f64, ctx: Context) -> Context {
    if 
    for (i, _) in points.iter().enumerate() {
        // starting point
        if i == 0 {
            ctx.move_to(points[0].x, points[0].y);
            continue;
        }

        // jumps to next point
        if i % 2 == 0 {
            continue;
        }

        let clutch_point = Point {
            x: points[i].x + smooth_f,
            y: points[i].y + smooth_f,
        };
        ctx.rel_curve_to(
            points[i].x,
            points[i].y,
            clutch_point.x,
            clutch_point.y,
            points[i + 1].x,
            points[i + 1].y,
        );
    }
    ctx
}

// returns set of random points from 'start' to 'end', where y is constant and x
// is defined by the distribution of points
fn gen_random_uniform_dist(start: Point, end: Point, coarse: u8) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let between = Uniform::from(1..10);
    let mut rng = rand::thread_rng();
    for i in 0..end.x as u8 {
        if i % coarse != 0 {
            continue;
        }
        let sampled = between.sample(&mut rng);
        let point = Point {
            x: sampled as f64,
            y: start.y,
        };
        points.push(point);
    }

    points
}
