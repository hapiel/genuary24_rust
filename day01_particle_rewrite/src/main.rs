use whiskers::prelude::*;
use geo::{BooleanOps, LineString, Polygon, MultiPolygon};

#[derive(Sketch)]
struct MySketch {
    /* add sketch parameters here */
    pen_width: f64,
    width: f64,
    height: f64,
    circle_count: usize,
    circle_radius: f64,
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            pen_width: 0.3,
            width: 190.0,
            height: 120.0,
            circle_count: 20,
            circle_radius: 2.
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here

        sketch
            .scale(Unit::Mm)
            .stroke_width(self.pen_width)
            .color(Color::new(0, 0, 20, 220));

        let mut circles = MultiPolygon::new(vec![]);

        for _i in 0..self.circle_count {
            let x = _ctx.rng_range(0.0..self.width);
            let y = _ctx.rng_range(0.0..self.height);

            let circle = create_circle(x, y, self.circle_radius, 100);

            circles.0.push(circle);
        }

        let mut union_result = MultiPolygon::new(vec![circles.0[0].clone()]);

        

            for polygon in circles.0.iter().skip(1) {
                union_result = union_result.union(&MultiPolygon::new(vec![polygon.clone()]));
            }

        sketch.add_path(union_result);

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

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(205., 130., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}
