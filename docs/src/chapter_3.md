# Chapter 3

## Profiling and performance

### Instrumentation

To compile a special version of binary with code instrumentation aimed to
generate flame graph:

`cargo build --release --features flame_it`

After running, the program will output flame-graph.html you can open
in any modern [browser](./flame-graph.html)

### Benchmarking

To run the benchmarks just type:

`cargo bench --all`

There is currently just one bench for the battle turn:

```text
...
turn                    time:   [101.55 ns 101.96 ns 102.39 ns]                 
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

...
```

Meaning each round runs in 102 nanosecond +/- 3% deviation, 1 nano = 10^-9 or 10 millions requests/round per second per core