#![feature(isqrt)]

use plotly::common::{Mode, Title};
use plotly::layout::Layout;
use plotly::{Plot, Scatter};
use std::ops::RangeInclusive;
use std::i128;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, One};

#[derive(Debug)]
struct FunctionPair {
    f: fn(i128) -> i128,
    g: fn(i128) -> i128,
    name: &'static str,
}

enum Deliverable {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Deliverable {
    fn get_function_pair(&self) -> FunctionPair {
        match self {
            Deliverable::One => FunctionPair {
                f: |x| (0.5 * x as f32 * (x as f32 - 1.0) + 10.0) as i128,
                g: |x| x.pow(2),
                name: "Deliverable 1",
            },
            Deliverable::Two => FunctionPair {
                f: |x| (0.5 * x as f32 * (x as f32 - 1.0) + 10.0) as i128,
                g: |x| x.pow(2),
                name: "Deliverable 2",
            },
            Deliverable::Three => FunctionPair {
                f: |x| {
                    let x_big = BigInt::from(x);
                    let result = x_big.pow(5) + BigInt::from(3) * x_big + BigInt::one();
                    let sqrt_result = result.sqrt(); // BigInt has a sqrt method for large integers
                    sqrt_result.to_i128().unwrap_or(0)
                },
                g: |x| 5 * x.pow(2),
                name: "Deliverable 3",
            },
            Deliverable::Four => FunctionPair {
                f: |x| x.ilog2() as i128,
                g: |x| x.isqrt(),
                name: "Deliverable 4",
            },
            Deliverable::Five => FunctionPair {
                f: |x| x.ilog2() as i128,
                g: |x| x.ilog10() as i128,
                name: "Deliverable 5",
            },
        }
    }
}

fn plot_function_pair(range: RangeInclusive<i128>, filename: &str, title: &str, step: usize, func_pair: FunctionPair, ratio: bool) {
    let x_vals: Vec<i128> = range.step_by(step).collect();
    let f_vals: Vec<i128> = x_vals.iter().map(|&n| (func_pair.f)(n)).collect();
    let g_vals: Vec<i128> = x_vals.iter().map(|&n| (func_pair.g)(n)).collect();

    let mut plot = Plot::new();

    if ratio {
        let ratio_vals: Vec<i128> = x_vals
            .iter()
            .filter_map(|&n| {
                let g_val = (func_pair.g)(n);
                if g_val != 0 {
                    Some((func_pair.f)(n) / g_val)
                } else {
                    None
                }
            })
            .collect();

        let trace = Scatter::new(x_vals.clone(), ratio_vals).mode(Mode::Lines).name("f(n)/g(n)");
        plot.add_trace(trace);
    } else {
        let trace1 = Scatter::new(x_vals.clone(), f_vals).mode(Mode::Lines).name("f(n)");
        let trace2 = Scatter::new(x_vals.clone(), g_vals).mode(Mode::Lines).name("g(n)");
        plot.add_trace(trace1);
        plot.add_trace(trace2);
    }

    let layout = Layout::new()
        .title(Title::with_text(title))
        .x_axis(plotly::layout::Axis::new().title("n"))
        .y_axis(plotly::layout::Axis::new().title("Function values"));
    plot.set_layout(layout);
    plot.write_html(filename);
}

fn main() {
    let deliverables = [
        Deliverable::One,
        Deliverable::Two,
        Deliverable::Three,
        Deliverable::Four,
        Deliverable::Five,
    ];

    for deliverable in &deliverables {
        let func_pair = deliverable.get_function_pair();
        plot_function_pair(
            1..=10,
            &format!("{}_plot_1_to_10.html", func_pair.name),
            &format!("{}: Functions f(n) and g(n) for n ranging from 1 to 10", func_pair.name),
            1,
            func_pair,
            false,
        );

        plot_function_pair(
            1..=1_000_000,
            &format!("{}_plot_1_to_1000000.html", func_pair.name),
            &format!("{}: Functions f(n) and g(n) for n ranging from 1 to 10^6", func_pair.name),
            1_000,
            func_pair,
            false,
        );

        plot_function_pair(
            1..=10,
            &format!("{}_ratio_plot_1_to_10.html", func_pair.name),
            &format!("{}: Function f(n) / g(n) for n ranging from 1 to 10", func_pair.name),
            1,
            func_pair,
            true,
        );

        plot_function_pair(
            1..=100_000_000,
            &format!("{}_ratio_plot_1_to_1000000.html", func_pair.name),
            &format!("{}: Function f(n) / g(n) for n ranging from 1 to 10^8", func_pair.name),
            10_000,
            func_pair,
            true,
        );
    }
}
