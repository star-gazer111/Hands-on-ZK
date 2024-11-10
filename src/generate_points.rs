use num::integer::Roots;
use plotters::prelude::*;

fn generate_points(modulus: u32) -> (Vec<u32>, Vec<u32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for x in 0..modulus {
        let y_squared = (x.pow(3) + 3) % modulus;

        if let Some(y) = mod_sqrt(y_squared, modulus) {
            xs.push(x);
            ys.push(y);
        }
    }
    (xs, ys)
}

fn mod_sqrt(a: u32, p: u32) -> Option<u32> {
    for x in 0..p {
        if (x.pow(2) % p) == a {
            return Some(x);
        }
    }
    None
}

fn plot_points(xs: Vec<u32>, ys: Vec<u32>, modulus: u32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y^2 = x^3 + 3 (mod p)", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..modulus, 0..modulus)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        xs.iter().zip(ys.iter()).map(|(&x, &y)| Circle::new((x, y), 3, BLUE.filled())),
    )?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modulus = 11;
    let (xs, ys) = generate_points(modulus);
    plot_points(xs, ys, modulus)?;
    println!("Plot saved as elliptic.png");
    Ok(())
}
