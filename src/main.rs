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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading JEFF-3.1 data for Mo-94(n,g)...");
    let cross_section_data = exfor_client::fetch_cross_section("Mo-94", "n,g", "JEFF-3.1").await?;

    let (energies, cross_sections) = if let Some(dataset) = cross_section_data.datasets.first() {
        let energies: Vec<f64> = dataset.points.iter().map(|p| p.energy * 1e-6).collect();
        let cross_sections: Vec<f64> = dataset.points.iter().map(|p| p.cross_section).collect();

        println!("Downloaded {} data points from API", energies.len());
        println!(
            "First point: E = {} MeV, σ = {} barn",
            energies[0], cross_sections[0]
        );
        (energies, cross_sections)
    } else {
        return Err("No dataset found in API response".into());
    };

    println!("\n=== MACS Calculation for JEFF-3.1 Mo-94(n,g) ===");
    let atomic_mass = 94.0;
    let temperatures_kev = vec![8.0, 25.0, 30.0, 90.0];

    println!("\nT(keV)    MACS(mb)");
    println!("--------------------");

    for &temp in &temperatures_kev {
        let macs_value = macs::calculate_macs(&energies, &cross_sections, atomic_mass, temp)?;
        println!("{:6.1}    {:12.6}", temp, macs_value);
    }

    Ok(())
}
