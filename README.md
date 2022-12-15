# Asahi Battery Threshold

This is a small daemon that makes it possible to set a
charging threshold for laptops running Asahi Linux.

## Usage

The program takes *one* command line argument,
containing a path to a config file. A config file
can look something like this:

```toml
stop_charging_threshold = 85
start_charging_threshold = 80
```

**Note**: The program requires root privileges in
order to change the charging behaviour.

# Logging

Logging can be turned off by setting the `CHARGE_LOG_LEVEL`
environment variable to `off`.
