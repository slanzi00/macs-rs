# macs-rs

A Rust program for calculating Maxwellian-Averaged Cross Sections (MACS) for neutron-induced reactions using data from the IAEA EXFOR database.

## Overview

The MACS is an important quantity in nuclear astrophysics, representing the reaction rate averaged over a Maxwellian neutron energy distribution at a given temperature. This tool automatically fetches cross section data from various nuclear data libraries and computes MACS values at user-specified temperatures.

## Features

- Automatic data retrieval from IAEA EXFOR API
- Support for multiple nuclear data libraries (JEFF-3.1, JEFF-4.0, ENDF-B-VIII.1, JENDL-5, etc.)
- Support for various reaction types (n,g), (n,p), etc.)

## Installation

Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

Clone the repository and build:

```bash
cargo build --release
```

## Usage

### Basic Usage

```bash
cargo run --release -- --target <NUCLEUS> --library <LIBRARY> --mass <MASS>
```

### Required Arguments

- `-t, --target <TARGET>` - Target nucleus (e.g., Mo-94, Zr-92)
- `-l, --library <LIBRARY>` - Nuclear data library name
- `-m, --mass <MASS>` - Atomic mass number

### Optional Arguments

- `-r, --reaction <REACTION>` - Reaction type (default: `n,g`)
- `-T, --temperatures <TEMPS>` - Comma-separated temperatures in keV (default: `8.0,25.0,30.0,90.0`)

### Examples

**Calculate MACS for Mo-94 using JEFF-3.1:**
```bash
cargo run --release -- --target Mo-94 --library JEFF-3.1 --mass 94
```

**Calculate MACS for Zr-92 using ENDF-B-VIII.1:**
```bash
cargo run --release -- --target Zr-92 --library ENDF-B-VIII.1 --mass 92
```

**Custom temperatures:**
```bash
cargo run --release -- --target Mo-94 --library JEFF-4.0 --mass 94 -T 5,10,20,30,50,100
```

**Different reaction type:**
```bash
cargo run --release -- --target Mo-94 --library JEFF-3.1 --mass 94 --reaction n,p -T 30
```

## Output

The program outputs MACS values in millibarns (mb) for each specified temperature:

```
=== MACS Calculation for JEFF-4.0 Mo-94(n,g) ===

T(keV)    MACS(mb)
--------------------
   8.0      195.468628
  25.0      103.541586
  30.0       93.522032
  90.0       53.676243
```

## Dependencies

- `reqwest` - HTTP client for API requests
- `serde` / `serde_json` - JSON serialization
- `tokio` - Async runtime
- `clap` - Command-line argument parsing

## References

- [IAEA EXFOR Database](https://www-nds.iaea.org/exfor/)
- Nuclear data libraries documentation
