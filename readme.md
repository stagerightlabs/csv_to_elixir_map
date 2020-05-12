# CSV to Map

A CLI tool to convert a csv file to a list of elixir maps.  I found myself creating some very large maps in elixir using data from CSV files, so I wrote this script to do the conversion for me. The goal was to be able to paste the output directly into a `seeder.exs` file.

## Setup

```
git clone git@github.com:stagerightlabs/csv_to_elixir_map.git
cd csv_to_elixir_map
composer install
```

## Usage

```
php convert.php path/to/file.csv output_filename.txt
```
