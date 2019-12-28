// I referenced code written by termoshtt
// https://rust-jp.slack.com/archives/C8FLSR5F1/p1577469085069900

use plotlib::line::{Line, Style};
use plotlib::page::Page;
use plotlib::style::Line as _Line;
use plotlib::view::ContinuousView;

use rug::{ops::Pow, Float};

fn rump(precision: u32) -> Float {
    let f = |val: f64| -> Float { Float::with_val(precision, val) };
    let a = f(77617.);
    let b = f(33096.);
    let a2 = a.clone().pow(2);
    let b2 = b.clone().pow(2);
    let b4 = b.clone().pow(4);
    let b6 = b.clone().pow(6);
    let b8 = b.clone().pow(8);
    (f(333.75) - &a2) * &b6
        + &a2 * (f(11.0) * &a2 * &b2 - f(121.0) * &b4 - f(2.0))
        + f(5.5) * &b8
        + a / (f(2.0) * b)
}

trait LogScale {
    fn log_scale(self) -> Self;
}

impl LogScale for Float {
    fn log_scale(self) -> Self {
        let one = Float::with_val(self.prec(), 1.);
        if self.is_sign_positive() {
            (self + one).log2()
        } else {
            -(self.abs() + one).log2()
        }
    }
}

fn main() {
    let data: Vec<(f64, f64)> = (20..150)
        .map(|p| (p.into(), rump(p).log_scale().to_f64()))
        .collect();
    let line = Line::new(data.as_slice()).style(Style::new().colour("#4E79A7"));
    let v = ContinuousView::new()
        .add(&line)
        .x_label("precision")
        .y_label("sgn(y) * log_2(1 + |y|)");
    Page::single(&v).save("plot.svg").unwrap();
}
