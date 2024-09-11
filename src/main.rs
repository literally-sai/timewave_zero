mod calculator;
mod cli;
mod data;

use calculator::calculate_timewave;

fn main() {
    let config = cli::parse_args();
    let results = calculate_timewave(
        config.dtz,
        config.step,
        config.negative_bailout,
        config.wave_factor,
    );
    cli::display_results(&results);
}
