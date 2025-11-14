# Advent-of-Code
My solutions for Advent of Code, organized by year.

## Usage

The project exposes a single binary that runs the solution for a specific Advent of Code **year** and **day**.

You must provide two parameters:

* `--year` or `-y` → the target year
* `--day` or `-d` → the target day

### Running with Cargo

When running through Cargo, arguments must be passed **after** `--` (this tells Cargo to stop parsing flags and forward them to your binary):

```bash
cargo run -- -y 2024 -d 1
```

### Running the compiled binary directly

```bash
./target/release/advent_of_code -y 2024 -d 1
```

### Example

Run the solution for **Day 01 of 2024**:

```bash
cargo run -- -y 2024 -d 1
```

If the year/day exist, the runner loads the corresponding module and executes the solution.
