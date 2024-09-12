use plotly::common::{Mode,Title};
use plotly::layout::{Layout};
use plotly::{Plot, ImageFormat};
use plotly::Scatter;
use std::ops::RangeInclusive;

// defining the two functions
fn f(x:i128) -> i128{
    (0.5 * x as f32 * (x as f32 - 1.0) + 10.0) as i128
}

//it panick, I believe due to this, can i32 store a value or a square of 10000000 ??? 
//Need more info on this. 

fn g(x:i128) -> i128{
    x.pow(2)
}

fn plot(range: RangeInclusive<i128>,filename: &str, title: &str, step: usize) {
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals.iter().map(|&n| f(n)).collect(); //output of the functionn f(x)
    let g_vals: Vec<i128> = x_vals.iter().map(|&n| g(n)).collect(); //output of the fucntion g(x)

    //trace of f valus
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
    let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_html(filename);

}

fn main(){
    plot(1..=10, "plot_1_to_10.html", "Functions f(n) and g(n) for n ranging from 1 to 10",1);
    // Plot for n ranging from 1 to 10^6
    plot(1..=1000000, "plot_1_to_1000000.html", "Functions f(n) and g(n) for n ranging from 1 to 10^6",1000);
}