# emusense

Emusense is a simple command line utility written in Rust 1.4 (stable)
generates fake sensor data and saves it as both CSV and binary (.pps) file
formats.

## Usage

The utility can be invoked as follows:

```
emusense <num-readings>
```

where `num-readings` specifies how many readings (rows in CSV) should be
generated.

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
