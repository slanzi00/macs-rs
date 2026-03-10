# macs-rs

A Rust program for calculating Maxwellian-Averaged Cross Sections (MACS) for neutron-induced reactions using data from the IAEA EXFOR database.

## Overview

The MACS is an important quantity in nuclear astrophysics, representing the reaction rate averaged over a Maxwellian neutron energy distribution at a given temperature. This tool automatically fetches cross section data from various nuclear data libraries and computes MACS values at user-specified temperatures.

## Features

- Automatic data retrieval from IAEA EXFOR API
- Atomic mass number parsed automatically from the target name (no need to pass it manually)
- Support for multiple nuclear data libraries (JEFF-4.0, ENDF/B-VIII.1, JENDL-5, TENDL-2025, etc.)
- Support for various reaction types (`n,g`, `n,p`, etc.)
- Cumulative MACS vs energy export to CSV (`--cumulative`)
- Python plotting script for cross section and cumulative MACS (`plot_cumulative_macs.py`)

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

Clone the repository and build:

```bash
cargo build --release
```

## Usage

### Basic Usage

```bash
cargo run --release -- --target <NUCLEUS> --library <LIBRARY>
```

### Required Arguments

- `-t, --target <TARGET>` — Target nucleus in the format `Element-A` (e.g., `Er-166`, `Mo-94`). The mass number is parsed automatically.
- `-l, --library <LIBRARY>` — Nuclear data library name (use the exact name returned by the EXFOR API, e.g., `ENDF/B-VIII.1` with the slash).

### Optional Arguments

- `-r, --reaction <REACTION>` — Reaction type (default: `n,g`)
- `-T, --temperatures <TEMPS>` — Comma-separated temperatures in keV (default: `8.0,25.0,30.0,90.0`)
- `-c, --cumulative` — Save cumulative MACS vs energy to a CSV file (filename auto-generated)

### Available libraries for a given nucleus

To see which libraries are available for a target, query the EXFOR API directly:

```
https://www-nds.iaea.org/exfor/e4list?Target=Er-166&Reaction=n,g&Quantity=SIG&json
```

The `LibName` field in each section entry is the exact string to pass to `--library`.

### Examples

**Calculate MACS for Er-166 using ENDF/B-VIII.1:**
```bash
cargo run --release -- --target Er-166 --library "ENDF/B-VIII.1"
```

**Custom temperatures:**
```bash
cargo run --release -- --target Er-166 --library "JEFF-4.0" -T 5,10,20,30,50,100
```

**Save cumulative MACS to CSV:**
```bash
cargo run --release -- --target Er-166 --library "ENDF/B-VIII.1" -T 8,25,30,90 --cumulative
```

This generates a file named `Er-166_ENDF_B-VIII.1_ng_cumulative_macs.csv` with columns:

```
# Library: ENDF/B-VIII.1  Target: Er-166  Reaction: (n,g)
E(keV),sigma(barn),MACS_cum_at8keV(mb),MACS_cum_at25keV(mb),MACS_cum_at30keV(mb),MACS_cum_at90keV(mb)
```

**Different reaction type:**
```bash
cargo run --release -- --target Mo-94 --library "JEFF-3.1" --reaction n,p -T 30
```

## Output

The program prints MACS values in millibarns (mb) for each specified temperature:

```
=== MACS Calculation for ENDF/B-VIII.1 Er-166(n,g) ===

T(keV)    MACS(mb)
--------------------
   8.0      1199.056605
  25.0       665.818215
  30.0       606.106442
  90.0       324.170...
```

## Plotting

A Python script is provided to plot the cumulative MACS and cross section from a CSV file:

```bash
# Interactive
python plot_cumulative_macs.py Er-166_ENDF_B-VIII.1_ng_cumulative_macs.csv

# Save to PNG
python plot_cumulative_macs.py Er-166_ENDF_B-VIII.1_ng_cumulative_macs.csv --out plot.png
```

The plot shows:
- **Left axis** (log): cross section σ in barn (dashed)
- **Right axis** (linear): cumulative MACS in mb at each temperature (solid lines)
- **X axis** (log): energy in keV, range 10⁻³ – 10⁴ keV
- Colorblind-safe Okabe-Ito palette

Requires: `matplotlib`, `numpy` (no `pandas`).

## Dependencies

- `reqwest` — HTTP client for API requests
- `serde` / `serde_json` — JSON serialization
- `tokio` — Async runtime
- `clap` — Command-line argument parsing

## References

- [IAEA EXFOR Database](https://www-nds.iaea.org/exfor/)
- Nuclear data libraries documentation
