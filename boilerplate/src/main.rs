use whiskers::prelude::*;

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
            width: 400.0,
            height: 300.0,
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here

        sketch
            .scale(Unit::Mm)
            .stroke_width(self.pen_width)
            .color(Color::new(0,0,20,220))
            .push_matrix_reset();

        sketch
            .rect(200., 200., self.width, self.height);

        Ok(())
    }
}

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(205., 130., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}