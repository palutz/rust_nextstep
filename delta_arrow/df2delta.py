import polars as pl

df = pl.read_parquet("data/thegrid.parquet")
df.write_delta("data/thegrid.delta")