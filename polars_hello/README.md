# Intro to Polars

## Installation

```
cargo add polars -F lazy
```

Or in *Cargo.toml*
```
[dependencies]
polars = { version = "x", features = ["lazy", ...]}
```

or 

```
[dependencies]
polars = { version = "...", features = ["lazy", "temporal", "describe", "json", "parquet", "dtype-datetime"] }
```