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

/// Command-line arguments for MACS calculation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target nucleus (e.g., Mo-94, Zr-92)
    #[arg(short, long)]
    target: String,

    /// Nuclear data library (e.g., JEFF-3.1, JEFF-4.0, ENDF-B-VIII.1, JENDL-5)
    #[arg(short, long)]
    library: String,

    /// Reaction type (default: n,g for neutron capture)
    #[arg(short, long, default_value = "n,g")]
    reaction: String,

    /// Atomic mass number (e.g., 94 for Mo-94)
    #[arg(short, long)]
    mass: f64,

    /// Temperatures in keV (comma-separated, e.g., 8,25,30,90)
    #[arg(
        short = 'T',
        long,
        value_delimiter = ',',
        default_value = "8.0,25.0,30.0,90.0"
    )]
    temperatures: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

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
        let macs_value = macs::calculate_macs(&energies, &cross_sections, args.mass, temp)?;
        println!("{:6.1}    {:12.6}", temp, macs_value);
    }

    Ok(())
}
