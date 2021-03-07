use crate::color::Color;
use crate::html::*;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

enum Inst {
    Circle(f64, f64, f64, Color),    // (x, y, r, color)
    Line(f64, f64, f64, f64, Color), // (x1, y1, x2, y2, color)
    Rect(f64, f64, f64, f64, Color), // (x1, y1, x2, y2, color)
}

struct VisualizerInternal {
    insts: BTreeMap<usize, Vec<Inst>>,
    now_page: usize,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

pub struct Visualizer {
    inner: Arc<Mutex<VisualizerInternal>>,
}

impl Visualizer {
    pub fn setpage(&mut self, page: usize) {
        self.inner.lock().unwrap().now_page = page;
    }

    pub fn nextpage(&mut self) {
        self.inner.lock().unwrap().now_page += 1;
    }

    pub fn circle(&mut self, x: f64, y: f64, r: f64, color: Color) {
        let v = &mut self.inner.lock().unwrap();
        let p = v.now_page;
        v.insts
            .entry(p)
            .or_insert(vec![])
            .push(Inst::Circle(x, y, r, color));
        v.min_x = v.min_x.min(x - r);
        v.min_y = v.min_y.min(y - r);
        v.max_x = v.max_x.max(x + r);
        v.max_y = v.max_y.max(y + r);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        let v = &mut self.inner.lock().unwrap();
        let p = v.now_page;
        v.insts
            .entry(p)
            .or_insert(vec![])
            .push(Inst::Line(x1, y1, x2, y2, color));
        v.min_x = v.min_x.min(x1.min(x2));
        v.min_y = v.min_y.min(y1.min(y2));
        v.max_x = v.max_x.max(x1.max(x2));
        v.max_y = v.max_y.max(y1.max(y2));
    }

    pub fn rect(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        let v = &mut self.inner.lock().unwrap();
        let p = v.now_page;
        v.insts
            .entry(p)
            .or_insert(vec![])
            .push(Inst::Rect(x1, y1, x2, y2, color));
        v.min_x = v.min_x.min(x1.min(x2));
        v.min_y = v.min_y.min(y1.min(y2));
        v.max_x = v.max_x.max(x1.max(x2));
        v.max_y = v.max_y.max(y1.max(y2));
    }

    pub fn write_to_file(&mut self) {
        let mut writer = BufWriter::new(File::create("result.html").unwrap());
        writeln!(writer, "{}", HTML_HEADER).unwrap();
        let v = &mut self.inner.lock().unwrap();

        let width = v.max_x - v.min_x;
        let height = v.max_y - v.min_y;
        let scale = 800. / width.max(height);
        let offset_x = -v.min_x;
        let offset_y = -v.min_y;

        for (page, insts) in &v.insts {
            write!(writer, "function page{}(c){{", page).unwrap();
            let mut before_color: Option<Color> = None;
            for inst in insts {
                match inst {
                    &Inst::Circle(x, y, r, color) => {
                        let (x, y, r) = ((offset_x + x) * scale, (offset_y + y) * scale, r * scale);
                        if before_color != Some(color) {
                            write!(writer, "s(c,\"{}\");", color).unwrap();
                            before_color = Some(color);
                        }
                        write!(writer, "a(c,{:.0},{:.0},{:.0});", x, y, r).unwrap();
                    }
                    &Inst::Line(x1, y1, x2, y2, color) => {
                        let (x1, y1, x2, y2) = (
                            (offset_x + x1) * scale,
                            (offset_y + y1) * scale,
                            (offset_x + x2) * scale,
                            (offset_y + y2) * scale,
                        );
                        if before_color != Some(color) {
                            write!(writer, "s(c,\"{}\");", color).unwrap();
                            before_color = Some(color);
                        }
                        write!(writer, "l(c,{:.0},{:.0},{:.0},{:.0});", x1, y1, x2, y2).unwrap();
                    }
                    &Inst::Rect(x1, y1, x2, y2, color) => {
                        let (x1, y1, x2, y2) = (
                            (offset_x + x1) * scale,
                            (offset_y + y1) * scale,
                            (offset_x + x2) * scale,
                            (offset_y + y2) * scale,
                        );
                        let (x, y, w, h) = (x1, y2, x2 - x1, y2 - y1);
                        if before_color != Some(color) {
                            write!(writer, "s(c,\"{}\");", color).unwrap();
                            before_color = Some(color);
                        }
                        write!(writer, "r(c,{:.0},{:.0},{:.0},{:.0});", x, y, w, h).unwrap();
                    }
                }
            }
            writeln!(writer, "}}").unwrap();
        }

        let s = v
            .insts
            .keys()
            .map(|p| format!("page{}", p))
            .collect::<Vec<_>>()
            .join(",");
        write!(writer, "const page_func=[{}];", s).unwrap();
        write!(writer, "{}", HTML_TAIL).unwrap();
        writer.flush().unwrap();
    }
}

pub fn visualizer() -> Visualizer {
    static INSTANCE: Lazy<Arc<Mutex<VisualizerInternal>>> = Lazy::new(|| {
        Arc::new(Mutex::new(VisualizerInternal {
            insts: BTreeMap::new(),
            now_page: 0,
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
