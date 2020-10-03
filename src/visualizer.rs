use crate::color::Color;
use crate::html::*;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

enum Inst {
    NewPage,
    Circle(f64, f64, f64, Color),    // (x, y, r, color)
    Line(f64, f64, f64, f64, Color), // (x1, y1, x2, y2, color)
}

struct VisualizerInternal {
    insts: Vec<Inst>,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

pub struct Visualizer {
    inner: Arc<Mutex<VisualizerInternal>>,
}

impl Visualizer {
    pub fn newpage(&mut self) {
        self.inner.lock().unwrap().insts.push(Inst::NewPage);
    }

    pub fn circle(&mut self, x: f64, y: f64, r: f64, color: Color) {
        let v = &mut self.inner.lock().unwrap();
        v.insts.push(Inst::Circle(x, y, r, color));
        v.min_x = v.min_x.min(x - r);
        v.min_y = v.min_y.min(y - r);
        v.max_x = v.max_x.max(x + r);
        v.max_y = v.max_y.max(y + r);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        let v = &mut self.inner.lock().unwrap();
        v.insts.push(Inst::Line(x1, y1, x2, y2, color));
        v.min_x = v.min_x.min(x1.min(x2));
        v.min_y = v.min_y.min(y1.min(y2));
        v.max_x = v.max_x.max(x1.max(x2));
        v.max_y = v.max_y.max(y1.max(y2));
    }

    pub fn write_to_file(&mut self) {
        let mut w = BufWriter::new(File::create("result.html").unwrap());
        write!(w, "{}", HTML_HEADER).unwrap();
        let v = &mut self.inner.lock().unwrap();

        while let Some(Inst::NewPage) = v.insts.last() {
            v.insts.pop();
        }

        let width = v.max_x - v.min_x;
        let height = v.max_y - v.min_y;
        let scale = 800. / width.max(height);
        let offset_x = -v.min_x;
        let offset_y = -v.min_y;

        let mut page_count = 1;
        let mut before_color: Option<Color> = None;
        for inst in &v.insts {
            match inst {
                Inst::NewPage => {
                    page_count += 1;
                    write!(w, "}}\nfunction page{}(c){{", page_count).unwrap();
                    before_color = None;
                }
                &Inst::Circle(x, y, r, color) => {
                    let (x, y, r) = ((offset_x + x) * scale, (offset_y + y) * scale, r * scale);
                    if before_color != Some(color) {
                        write!(w, "c.fillStyle=\"{0}\";c.strokeStyle=\"{0}\";", color).unwrap();
                        before_color = Some(color);
                    }
                    write!(
                        w,
                        "c.beginPath();c.arc({:.2},{:.2},{:.2},0,6.28);c.fill();",
                        x, y, r
                    )
                    .unwrap();
                }
                &Inst::Line(x1, y1, x2, y2, color) => {
                    let (x1, y1, x2, y2) = (
                        (offset_x + x1) * scale,
                        (offset_y + y1) * scale,
                        (offset_x + x2) * scale,
                        (offset_y + y2) * scale,
                    );
                    if before_color != Some(color) {
                        write!(w, "c.fillStyle=\"{0}\";c.strokeStyle=\"{0}\";", color).unwrap();
                        before_color = Some(color);
                    }
                    write!(w, "c.beginPath();c.moveTo({},{});c.lineTo({},{});c.closePath();c.stroke();", x1, y1, x2, y2).unwrap();
                }
            }
        }

        write!(w, "}}\nconst page_func=[page1").unwrap();
        for i in 1..page_count {
            write!(w, ",page{}", i + 1).unwrap();
        }
        write!(w, "];\n{}", HTML_TAIL).unwrap();
        w.flush().unwrap();
    }
}

pub fn visualizer() -> Visualizer {
    static INSTANCE: Lazy<Arc<Mutex<VisualizerInternal>>> = Lazy::new(|| {
        Arc::new(Mutex::new(VisualizerInternal {
            insts: vec![],
            min_x: f64::MAX,
            min_y: f64::MAX,
            max_x: f64::MIN,
            max_y: f64::MIN,
        }))
    });
    Visualizer {
        inner: INSTANCE.clone(),
    }
}
