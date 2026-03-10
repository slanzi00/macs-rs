//! MACS Calculator
//!
//! This program calculates the Maxwellian-Averaged Cross Section (MACS)
//! for neutron-induced reactions using data from the IAEA EXFOR database.
//!
//! The MACS is an important quantity in nuclear astrophysics, representing
//! the reaction rate averaged over a Maxwellian neutron energy distribution
//! at a given temperature.

mod exfor_client;
mod macs;

use clap::Parser;
use std::io::Write;

/// Command-line arguments for MACS calculation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target nucleus in the format Element-A (e.g., Er-166, Mo-94, Zr-92).
    /// The mass number is parsed automatically from this field.
    #[arg(short, long)]
    target: String,

    /// Nuclear data library (e.g., JEFF-4.0, ENDF/B-VIII.1, JENDL-5, TENDL-2025)
    #[arg(short, long)]
    library: String,

    /// Reaction type (default: n,g for neutron capture)
    #[arg(short, long, default_value = "n,g")]
    reaction: String,

    /// Temperatures in keV (comma-separated, e.g., 8,25,30,90)
    #[arg(
        short = 'T',
        long,
        value_delimiter = ',',
        default_value = "8.0,25.0,30.0,90.0"
    )]
    temperatures: Vec<f64>,

    /// Save cumulative MACS vs energy to a CSV file.
    /// The filename is auto-generated as <target>_<library>_cumulative_macs.csv
    /// (characters invalid in filenames are replaced with underscores).
    #[arg(short, long)]
    cumulative: bool,
}

/// Parses the atomic mass number from a target string (e.g., "Er-166" → 166.0).
fn parse_mass(target: &str) -> Result<f64, Box<dyn std::error::Error>> {
    target
        .split('-')
        .nth(1)
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| format!("Cannot parse mass number from target '{target}'. Expected format: Element-A (e.g., Er-166)").into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mass = parse_mass(&args.target)?;

    // Fetch cross section data from EXFOR database
    println!(
        "Downloading {} data for {}({})...",
        args.library, args.target, args.reaction
    );
    let cross_section_data =
        exfor_client::fetch_cross_section(&args.target, &args.reaction, &args.library).await?;

    // Extract energy and cross section vectors
    let (energies, cross_sections) = if let Some(dataset) = cross_section_data.datasets.first() {
        // Convert energy from eV to MeV
        let energies: Vec<f64> = dataset.points.iter().map(|p| p.energy * 1e-6).collect();
        let cross_sections: Vec<f64> = dataset.points.iter().map(|p| p.cross_section).collect();

        println!("Downloaded {} data points from API", energies.len());
        println!(
            "Energy range: {:.2e} - {:.2e} MeV",
            energies.first().unwrap_or(&0.0),
            energies.last().unwrap_or(&0.0)
        );
        (energies, cross_sections)
    } else {
        return Err("No dataset found in API response".into());
    };

    // Calculate MACS at specified temperatures
    println!(
        "\n=== MACS Calculation for {} {}({}) ===",
        args.library, args.target, args.reaction
    );
    println!("\nT(keV)    MACS(mb)");
    println!("--------------------");

    for &temp in &args.temperatures {
        let macs_value = macs::calculate_macs(&energies, &cross_sections, mass, temp)?;
        println!("{:6.1}    {:12.6}", temp, macs_value);
    }

    if args.cumulative {
        let rows = macs::calculate_cumulative_macs(
            &energies,
            &cross_sections,
            mass,
            &args.temperatures,
        )?;

        // Build a filename-safe string: replace '/' and any non-alphanumeric (except '-' '.') with '_'
        let safe_lib = args
            .library
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '.' { c } else { '_' })
            .collect::<String>();
        let filename = format!("{}_{}_{}_cumulative_macs.csv", args.target, safe_lib, args.reaction.replace(',', ""));

        let mut file = std::fs::File::create(&filename)?;

        // Header: library and target info as a comment, then column names
        writeln!(file, "# Library: {}  Target: {}  Reaction: ({})", args.library, args.target, args.reaction)?;
        let temp_headers: String = args
            .temperatures
            .iter()
            .map(|t| format!(",MACS_cum_at{}keV(mb)", t))
            .collect();
        writeln!(file, "E(keV),sigma(barn){}", temp_headers)?;

        for ((e_mev, cum_vals), &sigma) in rows.iter().zip(cross_sections.iter()) {
            let e_kev = e_mev * 1000.0;
            let vals: String = cum_vals.iter().map(|v| format!(",{:.10e}", v)).collect();
            writeln!(file, "{:.6},{:.10e}{}", e_kev, sigma, vals)?;
        }

        println!("\nCumulative MACS saved to '{}'", filename);
    }

    Ok(())
}
