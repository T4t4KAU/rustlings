// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

fn main() {
    const RADIUS: f32 = 5.0;

    let area = std::f32::consts::PI * RADIUS.powi(2);

    println!("The area of a circle with RADIUS {RADIUS:.2} is {area:.5}");
}