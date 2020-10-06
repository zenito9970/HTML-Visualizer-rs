use html_visualizer as vis;

fn main() {
    let n = 10;
    for i in 0..n {
        for y in 0..n {
            for x in 0..n {
                vis::circle(x, y, 0.5, vis::Color::GRAY);
            }
        }
        vis::circle(i, i, 0.5, vis::Color::BLACK);
        vis::line(i, i, n - 1, n - 1, vis::Color::RED);
        vis::nextpage();
    }
    vis::finish();
}
