use std::f64::consts::PI;

/// Boltzmann constant in MeV/K
const KB: f64 = 8.617e-11;

/// Calculates the trapezoidal area for numerical integration
///
/// # Arguments
/// * `f` - The function to integrate
/// * `x1` - Initial x value (energy)
/// * `x2` - Final x value (energy)
/// * `y1` - Initial y value (cross section)
/// * `y2` - Final y value (cross section)
///
/// # Returns
/// The area under the curve between (x1,y1) and (x2,y2)
fn trapezoid_area(f: &dyn Fn(f64, f64) -> f64, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let f1 = f(x1, y1);
    let f2 = f(x2, y2);
    0.5 * (f1 + f2) * (x2 - x1)
}

/// Calculates the Maxwellian-Averaged Cross Section (MACS)
///
/// The MACS is calculated using the formula:
/// MACS = (2*a²/(√π * (kT)²)) * ∫ σ(E) * E * exp(-a*E/(kT)) dE
///
/// where:
/// - a = A/(1+A) is the reduced mass factor
/// - A is the atomic mass number
/// - kT is the thermal energy (Boltzmann constant × temperature)
/// - σ(E) is the energy-dependent cross section
///
/// # Arguments
/// * `energies` - Energy points in MeV
/// * `cross_sections` - Cross section values in barns
/// * `atomic_mass` - Atomic mass number (e.g., 94 for Mo-94)
/// * `temperature_kev` - Temperature in keV
///
/// # Returns
/// * `Ok(macs)` - MACS value in millibarns
/// * `Err(msg)` - Error message if inputs are invalid
///
/// # Example
/// ```
/// let energies = vec![0.001, 0.002, 0.003]; // MeV
/// let cross_sections = vec![10.0, 8.0, 6.0]; // barns
/// let macs = calculate_macs(&energies, &cross_sections, 94.0, 30.0)?;
/// println!("MACS at 30 keV: {} mb", macs);
/// ```
pub fn calculate_macs(
    energies: &[f64],
    cross_sections: &[f64],
    atomic_mass: f64,
    temperature_kev: f64,
) -> Result<f64, String> {
    if energies.len() != cross_sections.len() {
        return Err("Energy and cross section vectors must have the same length".to_string());
    }

    if energies.is_empty() {
        return Err("Input vectors cannot be empty".to_string());
    }

    if temperature_kev <= 0.0 {
        return Err("Temperature must be positive".to_string());
    }

    // Convert temperature from keV to Kelvin
    // kT [MeV] = temperature_kev * 1e-3
    // T [K] = kT [MeV] / KB
    let temperature_k = (temperature_kev * 1e-3) / KB;

    // Reduced mass factor: a = A/(1+A)
    let a = atomic_mass / (1.0 + atomic_mass);

    // Integrand function: σ(E) * E * exp(-a*E/(kT))
    // where kT = KB * T
    let f = |e: f64, cs: f64| -> f64 { cs * e * (-(a * e) / (KB * temperature_k)).exp() };

    // Calculate the integral using the trapezoidal rule
    let mut macs_integral = 0.0;
    for i in 1..energies.len() {
        macs_integral += trapezoid_area(
            &f,
            energies[i - 1],
            energies[i],
            cross_sections[i - 1],
            cross_sections[i],
        );
    }

    // Normalization factor: 2*a²/(√π * (kT)²)
    let kt = KB * temperature_k;
    let normalization = (2.0 * a.powi(2)) / (PI.sqrt() * kt.powi(2));

    // MACS in barns
    let macs_barns = normalization * macs_integral;

    // Convert from barns to millibarns
    Ok(macs_barns * 1000.0)
}
