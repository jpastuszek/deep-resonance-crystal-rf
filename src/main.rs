#[macro_use]
extern crate structopt;
extern crate signifix;

use structopt::StructOpt;
use signifix::metric;
use signifix::TryFrom;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "Crystal RF")]
struct Crystal {
    #[structopt(name="Purity")]
    purity: f32,

    #[structopt(name="Strength")]
    strength: f32,

    #[structopt(name="Efficiency")]
    efficiency: f32,
}

fn total_rf(strength: f32, purity: f32) -> f32 {
    1_000_000_000.0 * (strength / 100.0) * ((purity + 30.0) / 130.0)
}

fn rf_per_tick(efficiency: f32, purity: f32) -> f32 {
    20_000.0 * (efficiency / 100.1) * (((purity + 2.0) / 102.0) + 1.0)
}

fn main() {
    let opt = Crystal::from_args();
    println!("{:?}", opt);
    println!("Total RF: {}", metric::Signifix::try_from(total_rf(opt.strength, opt.purity)).unwrap());
    println!("RF/tick: {}", metric::Signifix::try_from(rf_per_tick(opt.efficiency, opt.purity)).unwrap());
}
