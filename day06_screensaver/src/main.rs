use geo::{BooleanOps, LineString, Polygon};
use whiskers::prelude::*;

// Write path crawler, right angles, not going over itself
// Write system that creates segments that can cross over other segments.
// Perhaps also under segments? save segment height, and render later in correct order?

// the grid crawler system is too complex for me at the moment.
// I might come back to this later.


#[derive(Sketch)]
struct MySketch {
    /* add sketch parameters here */
    pen_width: f64,
    width: f64,
    height: f64,
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            pen_width: 0.3,
            width: 100.0,
            height: 110.0,
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here

        sketch
            .scale(Unit::Mm)
            .stroke_width(self.pen_width)
            .color(Color::new(0, 0, 20, 220));

        let rect = geo::Polygon::new(
            geo::LineString::from(vec![(0., 0.), (10., 0.), (10., 50.), (0., 50.), (0., 0.)]),
            vec![],
        );

        let circle = create_circle(0., 0., 5., 100);

        let joint = circle.union(&rect);

        sketch.add_path(joint);

        let cols = 6;
        let rows = 5;

        let mut tile_grid = create_2d_array(cols, rows);

        let path_length = 6;
        let start_x = ctx.rng_range(0..cols);
        let start_y = ctx.rng_range(0..rows);

        for i in 0..path_length {

        }


        // sketch
        //     .add_path(circle)
        //     .add_path(rect);

        Ok(())
    }
}

fn create_circle(x: f64, y: f64, radius: f64, num_points: usize) -> Polygon<f64> {
    let mut circle_points = Vec::with_capacity(num_points + 1);

    for i in 0..=num_points {
        let theta = 2.0 * std::f64::consts::PI * (i as f64) / (num_points as f64);
        let x = x + radius * theta.cos();
        let y = y + radius * theta.sin();
        circle_points.push((x, y).into());
    }

    // Close the circle
    circle_points.push(circle_points[0]);

    Polygon::new(LineString(circle_points), Vec::new())
}

#[derive(Debug, Clone, Copy)] // idk what I'm doing, but chatGPT does
enum Tile {
    Empty,
    Hor(isize),
    Ver(isize),
    HorVer,
    VerHor,
    CornerNW,
    CornerSW,
    CornerSE,
    CornerNE,
}

fn create_2d_array(cols: usize, rows: usize) -> Vec<Vec<Tile>> {
    vec![vec![Tile::Empty; cols]; rows]
}

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(20., 10., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}
