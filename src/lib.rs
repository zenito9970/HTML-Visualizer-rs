mod color;
mod html;
mod visualizer;
use visualizer::*;

pub use color::Color;

pub fn newpage() {
    if cfg!(debug_assertions) {
        visualizer().newpage();
    }
}

pub fn circle<T1, T2, T3>(x: T1, y: T2, r: T3, color: Color)
where
    T1: Into<f64>,
    T2: Into<f64>,
    T3: Into<f64>,
{
    if cfg!(debug_assertions) {
        let (x, y, r) = (x.into(), y.into(), r.into());
        visualizer().circle(x, y, r, color);
    }
}

pub fn line<T1, T2, T3, T4>(x1: T1, y1: T2, x2: T3, y2: T4, color: Color)
where
    T1: Into<f64>,
    T2: Into<f64>,
    T3: Into<f64>,
    T4: Into<f64>,
{
    if cfg!(debug_assertions) {
        let (x1, y1, x2, y2) = (x1.into(), y1.into(), x2.into(), y2.into());
        visualizer().line(x1, y1, x2, y2, color);
    }
}

pub fn finish() {
    if cfg!(debug_assertions) {
        visualizer().write_to_file();
    }
}
