use plotly::{Layout, Scatter};
use plotly::{ImageFormat, Plot};

fn main(){
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);
    plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
}