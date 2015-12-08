# emusense

Emusense is a simple command line utility written in Rust 1.4 (stable)
generates fake sensor data and saves it as both CSV and binary (.pps) file
formats.

## Usage

The utility can be invoked as follows:

```
emusense <num-readings> [--num-values=<n>, --rate=<hz>]
```

where `num-readings` specifies how many readings (rows in CSV) should be
generated, `num-values` is an optional argument that specifies the number of
sensor values per row (measurement), and `rate` is an optional argument that
specifies the sampling rate in Hz.

*Defaults:*
+ `num-values`: 26, each 2-byte long signed integer
+ `rate`: 10kHz

*NOTE:* it is assumed that the minimum time increment is 1us; hence,
`rate` should not exceed 1MHz.

## Building and testing

To build the utility, run in the command line:

```
$ cargo build
```

To test it, run:

```
$ cargo test
```

## License

[MIT license.](LICENSE.md)
