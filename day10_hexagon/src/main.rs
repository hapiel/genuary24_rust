use whiskers::prelude::*;

#[derive(Sketch)]
struct MySketch {
    /* add sketch parameters here */
    pen_width: f64,
    width: f64,
    height: f64,
    scale_width: f64,
    scale_height: f64,
    rotate: f64,
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            pen_width: 0.3,
            width: 400.0,
            height: 300.0,
            scale_width: 1.,
            scale_height: 1.,
            rotate: 0.,
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here

        sketch
            .scale(Unit::Mm)
            .stroke_width(self.pen_width)
            .color(Color::new(0, 0, 20, 220))
            .scale_non_uniform(self.scale_width, self.scale_height)
            .rotate(degrees_to_radians(self.rotate));

        

        sketch.rect(200., 200., self.width, self.height);

        Ok(())
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(205., 130., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}
