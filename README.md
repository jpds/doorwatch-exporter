# doorwatch-exporter

This is a exporter which reads a value from a GPIO PIN and converts the time
that PIN has spent in an on state into a second value.

In another words, it can be used to turn a simple reed switch on a Raspberry Pi
into a security system that monitors how long a door/window has been opened for.

This time value can then be tracked by a Prometheus server for plotting on a
graph.

Two threads are used: one for the HTTP endpoint Prometheus interacts with, and
another for polling the GPIO PIN every 500 milliseconds and recording the value.

## Building

Simply run:

```bash
cargo build --release
```

The binary file will be in `target/release/` directory.

## Usage

Specify the GPIO PIN to monitor with the `--gpio-pin` flag:

```bash
doorwatch-exporter --gpio-pin 26
```

The metrics endpoint to provide to Prometheus will be accessible on port `9184`.

```bash
curl http://localhost:9184/metrics
```
