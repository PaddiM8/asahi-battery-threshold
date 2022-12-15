# Asahi Battery Threshold

A small daemon that makes it possible to set a
charging threshold for laptops running Asahi Linux.

## Usage

The program takes *one* command line argument,
containing a path to a config file. Below is
an example of a config file:

```toml
stop_charging_threshold = 85
start_charging_threshold = 80
```

**Note**: The program requires root privileges
in order to change the charging behaviour.

## Logging

Logging can be turned off by setting the `CHARGE_LOG_LEVEL`
environment variable to `off`.
