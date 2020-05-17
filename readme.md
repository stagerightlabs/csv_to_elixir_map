# CSV to Map

A CLI tool to convert a csv file to a list of elixir maps.  I found myself creating some very large static maps in elixir using data from CSV files, so I wrote this script to do the conversion for me. The goal was to be able to paste the output directly into a `seeder.exs` file.

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

## Inspiration

- https://docs.rs/csv/1.1.3/csv/tutorial/index.html
- https://raw.githubusercontent.com/BurntSushi/rust-csv/master/examples/data/uspop.csv
