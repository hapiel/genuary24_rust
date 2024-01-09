use whiskers::prelude::*;

#[derive(Sketch)]
struct MySketch {
    /* add sketch parameters here */
    pen_width: f64,
    width: f64,
    height: f64,

    r1: f64,
    r2: f64,
    m1: f64,
    m2: f64,
    a1: f64,
    a2: f64,
    g: f64,
    amount: usize,
    a_difference: f64,
    r1_factor: f64,
    catmull_rom: bool,
    tension: f64,
    x_offset: f64,
    y_offset: f64,
    rotation: f64,
    scale_width: f64,
    scale_height: f64,
}

impl Default for MySketch {
    fn default() -> Self {
        Self {
            /* initialize sketch parameters to default values here */
            pen_width: 0.3,
            width: 400.0,
            height: 300.0,
            r1: 50.,
            r2: 50.,
            m1: 10.,
            m2: 10.,
            a1: 0.,
            a2: 0.,
            g: 1.,
            amount: 50,
            a_difference: 0.003,
            r1_factor: 0.5,
            catmull_rom: false,
            tension: 0.5,
            x_offset: 0.,
            y_offset: 0.,
            rotation: 0.,
            scale_width: 1.,
            scale_height: 1.,
        }
    }
}

impl App for MySketch {
    fn update(&mut self, sketch: &mut Sketch, ctx: &mut Context) -> anyhow::Result<()> {
        // draw code goes here

        let g = self.g;
        let m1 = self.m1;
        let m2 = self.m2;

        // let mut a1 = degrees_to_radians(self.a1);
        // let mut a2 = degrees_to_radians(self.a2);
        // let mut a1_v = 0.;
        // let mut a2_v = 0.;

        let mut a1 = vec![degrees_to_radians(self.a1); self.amount];
        let mut a2 = vec![degrees_to_radians(self.a2); self.amount];
        let mut a1_v = vec![0.; self.amount];
        let mut a2_v = vec![0.; self.amount];
        let mut a1_a = vec![0.; self.amount];
        let mut a2_a = vec![0.; self.amount];

        for i in 0..self.amount {
            a1[i] += self.a_difference * i as f64;

            for _ in 0..(ctx.time * 25.) as isize {
                // top pendulum
                let num1 = -g * (2. * m1 + m2) * a1[i].sin();
                let num2 = -m2 * g * (a1[i] - 2. * a2[i]).sin();
                let num3 = -2. * (a1[i] - a2[i]).sin() * m2;
                let num4 = a2_v[i] * a2_v[i] * self.r2
                    + a1_v[i] * a1_v[i] * self.r1 * (a1[i] - a2[i]).cos();
                let den = self.r1 * (2. * m1 + m2 - m2 * (2. * a1[i] - 2. * a2[i]).cos());
                a1_a[i] = (num1 + num2 + num3 * num4) / den;

                // bottom pendulum
                let num1 = 2. * (a1[i] - a2[i]).sin();
                let num2 = a1_v[i] * a1_v[i] * self.r1 * (m1 + m2);
                let num3 = g * (m1 + m2) * a1[i].cos();
                let num4 = a2_v[i] * a2_v[i] * self.r2 * m2 * (a1[i] - a2[i]).cos();
                let den = self.r2 * (2. * m1 + m2 - m2 * (2. * a1[i] - 2. * a2[i]).cos());
                a2_a[i] = (num1 * (num2 + num3 + num4)) / den;

                a1_v[i] += a1_a[i];
                a1[i] += a1_v[i];
                a2_v[i] += a2_a[i];
                a2[i] += a2_v[i];
            }
        }

        sketch
            .scale(Unit::Mm)
            .stroke_width(self.pen_width)
            .color(Color::new(0, 0, 20, 220))
            .translate(100., 20.);

        sketch
            .scale_non_uniform(self.scale_width, self.scale_height)
            .rotate(degrees_to_radians(self.rotation));

        for i in 0..self.amount {

            let x0 = self.x_offset * i as f64 / 10.;
            let y0 = self.y_offset * i as f64 / 10.;

            let x1 = self.r1 * a1[i].sin() + x0;
            let y1 = self.r1 * a1[i].cos() + y0;
            let x2 = x1 + self.r2 * a2[i].sin();
            let y2 = y1 + self.r2 * a2[i].cos();

            if self.catmull_rom {
                sketch.catmull_rom([Point::new(lerp(x0, x1, self.r1_factor) , lerp(y0, y1, self.r1_factor) ), Point::new(x1, y1), Point::new( x2, y2)], self.tension);
            } else {
                sketch.line(lerp(x0, x1, self.r1_factor) , lerp(y0, y1, self.r1_factor) , x1, y1).line(x1, y1, x2, y2);
            }

            
        }

        Ok(())
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (std::f64::consts::PI / 180.0)
}

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn main() -> Result {
    Runner::new(MySketch::default())
        .with_page_size_options(PageSize::Custom(205., 130., Unit::Mm))
        .with_layout_options(LayoutOptions::Center)
        .run()
}
