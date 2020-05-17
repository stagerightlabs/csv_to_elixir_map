# CSV to Elixir Map Array

I recently found myself creating a handful of large static maps in Elixir using data from CSV files.  I wrote this tool to automate the conversion for me. The goal was to be able to paste the output directly into a `seeder.exs` file.

## Setup

```
git clone git@github.com:stagerightlabs/csv_to_elixir_map.git
cd csv_to_elixir_map
cargo build
```

## Usage

```
csvmap path/to/file.csv output_file.txt
```

## Credits

- https://docs.rs/csv/1.1.3/csv/tutorial/index.html
- https://raw.githubusercontent.com/BurntSushi/rust-csv/master/examples/data/uspop.csv
