# rsldx

`rsldx` is a library (`poletermrs`) and CLI tool (`ptprint`) for writing to Logic Controls LDX/LTX-model pole displays.

This is an entirely userland utility, and requires no special kernel modules.

As of now (12/2023), this has been tested on the following hardware:
- LTX9000UP-GY


## Using the CLI Tool

`ptprint` can be installed via `cargo install`.

From there, `ptprint` will connect to the display and print any provided message:

```
ldprint print "Hello world!"
```

## Using `ptprint` without root

On linux, connecting to a raw USB device usually requires root; connecting without root requires a custom `udev` rule,
usually placed under `/etc/udev/rules.d/`. While the precise format depends on your distro, it'll usually look something like this:

```
cat /etc/udev/rules.d/30-lcidisplay.rules
SUBSYSTEM=="usb", ATTRS{idVendor}=="0fa8", ATTRS{idProduct}=="a090", ACTION=="add", GROUP="wheel", MODE="0664"
```