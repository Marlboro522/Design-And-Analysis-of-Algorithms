#![feature(isqrt)]

use plotly::common::{Mode,Title};
use plotly::layout::Layout;
use plotly::{Scatter,Plot,ImageFormat};
use std::ops::RangeInclusive;
use std::i128;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, One};

// TODO: Need to add more information in the title of the function for easier examination.
// TODO: Need to see why division functions are not working as expected. Resolved.
// TODO: After submission make this more beautiful and modular. Use enums to define functions and traits to genneralize logic on this. 



// defining the two functions
// Graphs for function in problem 1
//DELIVERABLE 1
fn f(x:i128) -> i128{
    (0.5 * x as f32 * (x as f32 - 1.0) + 10.0) as i128
}

//it panick, I believe due to this, can i32 store a value or a square of 1000000 ??? 
//Need more info on this. 

fn g(x:i128) -> i128{
    x.pow(2)
}

//DELIVERABLE 3
fn f_deli3(x: i128) -> i128 {
    let x_big = BigInt::from(x);
    let result = x_big.pow(5) + BigInt::from(3) * x_big + BigInt::one();
    let sqrt_result = result.sqrt(); // BigInt has a sqrt method for large integers
    // println!("f_deli3: {}", sqrt_result.to_i128().unwrap_or(0));
    sqrt_result.to_i128().unwrap_or(0)
    
}

fn g_deli3(x:i128)->i128{
    // println!("g_deli3: {}", 5 * x.pow(2));
    5 * x.pow(2)
}

//Deliverable 4

fn f_deli4(x:i128) ->i128 {
    x.ilog2() as i128
}

fn g_deli4(x:i128) ->i128{
    x.isqrt()
}

fn f_deli5(x:i128) ->i128{
    x.ilog2() as i128
}

fn g_deli5(x:i128) ->i128{
    x.ilog10() as i128
}

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
    // plot.show();
    // plot.show();
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    
}

fn plot_deliverable_2(range: RangeInclusive<i128>, filename: &str, title: &str, step: usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<f64> = x_vals
        .iter()
        .filter_map(|&n| {
            let g_val = g(n);
            if g_val != 0 {
                Some(f(n) as f64 / g_val as f64)
            } else {
                None
            }
        })
        .collect(); //values for f(n)\g(n)
    // let mut count: i32 = 0;
    // for item in &f_vals{
    //     println!("{}", item);
    //     count=count+1;
    //     if count==100{
    //         break;
    //     }
    // }
    //trace of f valus
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)/g(n)");
    // let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);
    // plot.add_trace(trace2);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}

// fn plot_deliverable_3(range: RangeInclusive<i128>,filename: &str, title: &str, step:usize){
//     let x_vals: Vec<i128> = range.step_by(step).collect();
//     let f_vals: Vec<i128> = x_vals.iter().map(|&n| f_deli3(n)).collect(); //output of the functionn f(x)
//     let g_vals: Vec<i128> = x_vals.iter().map(|&n| g_deli3(n)).collect(); //output of the fucntion g(x)
//     for item in &g_vals{
//         println!("{}", item);
//     }
//     //trace of f valus
//     let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
//     let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

//     let mut plot= Plot::new();
//     plot.add_trace(trace1);
//     plot.add_trace(trace2);

//     let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
//     plot.set_layout(layout);
//     plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
//     // plot.show();
// }
//g(n) flattens out as it doesn't really have any signnificant values, using logarithmic scale to emphasize the graph. 
fn plot_deliverable_3(range: RangeInclusive<i128>, filename: &str, title: &str, step: usize) {
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals.iter().map(|&n| f_deli3(n)).collect(); //output of the function f(x)
    let g_vals: Vec<i128> = x_vals.iter().map(|&n| g_deli3(n)).collect(); //output of the function g(x)

    //trace of f values
    let trace1 = Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
    let trace2 = Scatter::new(x_vals.clone(), g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::with_text(title))
        .x_axis(plotly::layout::Axis::new().title("n").type_(plotly::layout::AxisType::Log))
        .y_axis(plotly::layout::Axis::new().title("Function values").type_(plotly::layout::AxisType::Log));

    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}


fn plot_deliverable_3_part2(range: RangeInclusive<i128>,filename: &str, title: &str, step:usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<f64> = x_vals
        .iter()
        .filter_map(|&n| {
            let g_val = g_deli3(n);
            if g_val != 0 {
                Some(f_deli3(n) as f64 / g_val as f64)
            } else {
                None
            }
        })
        .collect(); //values for f(n)\g(n)
    // let mut count: i32 = 0;
    // for item in &f_vals{
    //     println!("{}", item);
    //     count=count+1;
    //     if count==100{
    //         break;
    //     }
    // }
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)/g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}

fn plot_deliverable_4_part1(range: RangeInclusive<i128>,filename: &str, title: &str, step:usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals.iter().map(|&n| f_deli4(n)).collect(); //output of the functionn f(x)
    let g_vals: Vec<i128> = x_vals.iter().map(|&n| g_deli4(n)).collect(); //output of the fucntion g(x)

    //trace of f valus
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
    let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}

fn plot_deliverable_4_part2(range: RangeInclusive<i128>,filename: &str, title: &str, step:usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<f64> = x_vals
        .iter()
        .filter_map(|&n| {
            let g_val = g_deli4(n);
            if g_val != 0 {
                Some(f_deli4(n) as f64 / g_val as f64)
            } else {
                None
            }
        })
        .collect(); //values for f(n)\g(n)
    // let mut count: i32 = 0;
    // for item in &f_vals{
    //     println!("{}", item);
    //     count=count+1;
    //     if count==100{
    //         break;
    //     }
    // }
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)/g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    // plot.show();
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
}

fn plot_deliverable_5_part1(range: RangeInclusive<i128>,filename: &str, title: &str, step:usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals.iter().map(|&n| f_deli5(n)).collect(); //output of the functionn f(x)
    let g_vals: Vec<i128> = x_vals.iter().map(|&n| g_deli5(n)).collect(); //output of the fucntion g(x)

    //trace of f valus
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
    let trace2=Scatter::new(x_vals.clone(),g_vals).mode(Mode::Lines).name("g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}

fn plot_deliverable_5_part2(range: RangeInclusive<i128>,filename: &str,title: &str,step: usize){
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<f64> = x_vals
        .iter()
        .filter_map(|&n| {
            let g_val = g_deli5(n);
            if g_val != 0 {
                Some(f_deli5(n) as f64 / g_val as f64)
            } else {
                None
            }
        })
        .collect(); //values for f(n)\g(n)
    // let mut count: i32 = 0;
    // for item in &f_vals{
    //     println!("{}", item);
    //     count=count+1;
    //     if count==100{
    //         break;
    //     }
    // }
    let trace1=Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)/g(n)");

    let mut plot= Plot::new();
    plot.add_trace(trace1);

    let layout = Layout::new().title(Title::with_text(title)).x_axis(plotly::layout::Axis::new().title("n")).y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    // plot.show();
}
fn main(){
    //Deliverable 1 PART 1
    plot_deliverable_1(1..=10, "Deliverable 1: plot_1_to_10", "Deliverable 1: Functions f(n) and g(n) for n ranging from 1 to 10",1);
    plot_deliverable_1(1..=1000000, "Deliverable 1: plot_1_to_1000000", "Deliverable 1: Functions f(n) and g(n) for n ranging from 1 to 10^6",10000);


    // Deliverable 2
    plot_deliverable_2(1..=10, "Deliverable 2: plot_1_to_10", "Deliverable 2: Function f(n) / g(n) for n ranging from 1 to 10", 1);
    plot_deliverable_2(1..=1000000, "Deliverable 2: plot_1_to_1000000", "Deliverable 2: Function f(n) / g(n) for n ranging from 1 to 10^6", 10000);

    //Deliverable 3: part 1
    plot_deliverable_3(1..=10, "Deliverable 3: plot_1_to_10", "Deliverable 3: Functions f(n) and g(n) for n ranging from 1 to 10",1);
    plot_deliverable_3(1..=1000000, "Deliverable 3: plot_1_to_1000000", "Deliverable 3: Functions f(n) and g(n) for n ranging from 1 to 10^6",10000);

    //Deliverable 3: part 2
    plot_deliverable_3_part2(1..=10, "Deliverable 31: plot_1_to_10", "Deliverable 3: Function f(n) / g(n) for n ranging from 1 to 10",1);
    plot_deliverable_3_part2(1..=1000000, "Deliverable 31: plot_1_to_1000000", "Deliverable 3: Function f(n) / g(n) for n ranging from 1 to 10^6",10000);

    //Deliverable 4: part 1
    plot_deliverable_4_part1(1..=10, "Deliverable 4: plot_1_to_10", "Deliverable 4: Functions f(n) and g(n) for n ranging from 1 to 10",1);
    plot_deliverable_4_part1(1..=1000000, "Deliverable 4: plot_1_to_1000000", "Deliverable 4: Functions f(n) and g(n) for n ranging from 1 to 10^6",10000);

    //Deliverable 4: part 2
    plot_deliverable_4_part2(1..=10, "Deliverable 41: plot_1_to_10", "Deliverable 4: Function f(n) / g(n) for n ranging from 1 to 10",1);
    plot_deliverable_4_part2(1..=1000000, "Deliverable 41: plot_1_to_1000000", "Deliverable 4: Function f(n) / g(n) for n ranging from 1 to 10^6",10000);

    //Deliverable 5: part 1
    plot_deliverable_5_part1(1..=10, "Deliverable 5: plot_1_to_10", "Deliverable 5: Functions f(n) and g(n) for n ranging from 1 to 10",1);
    plot_deliverable_5_part1(1..=1000000, "Deliverable 5: plot_1_to_1000000", "Deliverable 5: Functions f(n) and g(n) for n ranging from 1 to 10^6",10000);

    //Deliverable 5: part2
    plot_deliverable_5_part2(1..=1000, "Deliverable 51: plot_1_to_1000", "Deliverable 5: Function f(n) / g(n) for n ranging from 1 to 10",1);
    plot_deliverable_5_part2(1..=1000000, "Deliverable 51: plot_1_to_1000000", "Deliverable 5: Function f(n) / g(n) for n ranging from 1 to 10^6",10000);

    println!("Done");
}