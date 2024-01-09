use whiskers::prelude::*;

#[derive(Sketch)]
struct MySketch {
    /* add sketch parameters here */
    pen_width: f64,
    width: f64,
    height: f64,
    cols: usize,
    rows: usize,
    spacing: f64,
    cell_width_multiplier: f64,
    random_width_multiplier: f64,
}


impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            pen_width: 0.3,
            width: 400.0,
            height: 300.0,
            cols: 10,
            rows: 10,
            spacing: 5.,
            cell_width_multiplier: 0.5,
            random_width_multiplier: 1.,
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

        Grid::from_total_size([self.width, self.height])
            .columns(self.cols)
            .rows(self.rows)
            .build(sketch, |sketch, cell| {
                let p1 = cell.position + (Point::new(
                    ctx.rng_range(0.0..cell.size[0]) * self.random_width_multiplier,
                    0,
                ));

                let p2 = cell.position + (Point::new(
                    cell.size[0] * self.cell_width_multiplier
                        + ctx.rng_range(0.0..cell.size[0] * self.random_width_multiplier),
                    0,
                ));
                let p3 = cell.position + (Point::new(
                    cell.size[0] * self.cell_width_multiplier
                        + ctx.rng_range(0.0..cell.size[0] * self.random_width_multiplier),
                    cell.size[1] - self.spacing,
                ));
                let p4 = cell.position + (Point::new(
                    ctx.rng_range(0.0..cell.size[0]) * self.random_width_multiplier,
                    cell.size[1] - self.spacing,
                ));

                sketch.polyline([p1, p2, p3, p4].into_iter(), true);
            });

        // sketch
        //     .rect(200., 200., self.width, self.height);

        Ok(())
    }
}

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(205., 130., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}
