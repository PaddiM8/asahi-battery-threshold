# asahi-battery-threshold

A small daemon that makes it possible to set a
charging threshold for laptops running Asahi Linux.

## DEPRECATED

This tool is finally no longer necessary! See [More information](https://github.com/PaddiM8/asahi-battery-threshold/issues/3)

## How it works

Since the kernel-side doesn't yet support setting an actual threshold,
this daemon monitors battery usage and interacts with files
in `/sys/class/power_supply/macsmc-battery/` to enable/disable
charging. It can *not* change the charging behaviour during sleep.

Preventing the battery from charging after a certain point is useful
when you have it plugged in for extended periods of time, in order
to put less strain on the battery. By default, this program will
prevent charging after reaching 85% capacity.

## Installation

On Arch-based systems, the program can be installed
with the following command:

```sh
makepkg -si
```

It can then be started by running

```sh
systemctl start asahi-battery-threshold
```
or
```
asahi-battery-threshold
```

### Auto-start

```sh
systemctl enable asahi-battery-threshold
```

## Usage

### Default configuration path

The default configuration file is located in `/etc/asahi-battery-threshold.conf`.

### Command-line
The program takes at most *one* command line argument,
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
