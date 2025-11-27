mod exfor_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cross_section_data =
        exfor_client::fetch_cross_section("Mo-94", "n,g", "ENDF/B-VIII.1").await?;

    println!("Format: {}", cross_section_data.format);
    println!("Number of datasets: {}", cross_section_data.datasets.len());

    if let Some(dataset) = cross_section_data.datasets.first() {
        println!("Dataset ID: {}", dataset.id);
        println!("Library: {}", dataset.library);
        println!("Target: {}", dataset.target);
        println!("Reaction: {}", dataset.reaction);
        println!("Number of points: {}", dataset.n_pts);
        println!("First 3 points:");
        for point in dataset.points.iter().take(3) {
            println!(
                "  E={:.5e} eV, Sig={:.5e} b, dSig={:.5e} b",
                point.energy, point.cross_section, point.uncertainty
            );
        }
    }

    Ok(())
}
