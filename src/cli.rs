use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Days to zero point
    #[arg(short, long)]
    pub dtz: f64,

    /// Days to calculate after zero point
    #[arg(short, long)]
    pub negative_bailout: f64,

    /// Time step in minutes
    #[arg(short, long)]
    pub step: f64,

    /// Wave factor (default: 64)
    #[arg(short, long, default_value_t = 64)]
    pub wave_factor: u32,
}

pub fn parse_args() -> Config {
    let mut config = Config::parse();
    config.negative_bailout *= -1.0;
    config.step /= 24.0 * 60.0; // Convert minutes to days
    config
}

pub fn display_results(results: &[crate::calculator::TimewaveResult]) {
    println!("DTZ,Kelley,Watkins,Sheliak,Huang Ti");
    for result in results {
        println!(
            "{:.6},{:.6},{:.6},{:.6},{:.6}",
            result.dtz, result.kelley, result.watkins, result.sheliak, result.huang_ti
        );
    }
}
