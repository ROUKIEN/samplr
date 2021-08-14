# Samplr

A CLI tool to get a sample dataset from an existing file.

> Under the hood, it uses reservoir sampling algorithms.

# Usage

> :under_construction: keep in mind that the API is not stable yet so CLI args may change from a release to another.

```shell
# keep 100 lines from the file.csv
samplr --keep=100 file.csv > output.csv
```

# Installation

Get the latest binary available from the releases section.

I only support linux for now but I plan to provide a windows build too.

# Use cases

## Reducing a huge dataset to work locally

As a developer, I often need to read huge csv files and process them during my development process. However, my computer isn't a 64Gb workstation and does not have the same hardware specs than the production servers that will run the code later.

Using `samplr` allow me to reduce an existing dataset to something viable during development/staging, having a realistic dataset that I can keep somewhere and regenerate whenever I need it.

# Supported formats

## CSV

For now, it only supports CSV files (or any file which contains one item per line) as the sampling is done per line.

## JSON/XML

I plan for JSON/XML support later.
