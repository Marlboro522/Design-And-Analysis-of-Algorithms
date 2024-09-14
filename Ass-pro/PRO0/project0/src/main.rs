use plotly::common::{Mode,Title};
use plotly::layout::Layout;
use plotly::{Scatter,Plot};
use std::ops::RangeInclusive;

// defining the two functions
// Graphs for function in problem 1
//DELIVERABLE 1
fn f(x:i128) -> i128{
    (0.5 * x as f32 * (x as f32 - 1.0) + 10.0) as i128
}

//it panick, I believe due to this, can i32 store a value or a square of 10000000 ??? 
//Need more info on this. 

fn g(x:i128) -> i128{
    x.pow(2)
}

//DELIVERABLE 2


fn plot_deliverable_1(range: RangeInclusive<i128>,filename: &str, title: &str, step: usize) {
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

fn plot_deliverable_2(range: RangeInclusive<i128>, filename: &str, title: &str, step: usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals
        .iter()
        .filter_map(|&n| {
            let g_val = g(n);
            if g_val != 0 {
                Some(f(n) / g_val)
            } else {
                None
            }
        })
        .collect(); //values for f(n)\g(n)
    let mut count: i32 = 0;
    for item in &f_vals{
        println!("{}", item);
        count=count+1;
        if count==100{
            break;
        }
    }
    //trace of f valus
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)/g(n)");
    // let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);
    // plot.add_trace(trace2);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_html(filename);
}

fn main(){
    //Deliverable 1 PART 1
    plot_deliverable_1(1..=10, "Deliverable 1: plot_1_to_10.html", "Deliverable 1: Functions f(n) and g(n) for n ranging from 1 to 10",1);
    plot_deliverable_1(1..=1000000, "Deliverable 1: plot_1_to_1000000.html", "Deliverable 1: Functions f(n) and g(n) for n ranging from 1 to 10^6",1000);


    //Deliverable 2
    plot_deliverable_2(1..=10, "Deliverable 2: plot_1_to_10.html", "Deliverable 2: Function f(n) / g(n) for n ranging from 1 to 10", 1);
    plot_deliverable_2(1..=100000000, "Deliverable 2: plot_1_to_1000000.html", "Deliverable 2: Function f(n) / g(n) for n ranging from 1 to 10^8", 10_000);
}