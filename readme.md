# rsldx

`rsldx` is a library and CLI tool for writing to Logic Controls LDX/LTX-model pole displays.

This is an entirely userland utility, and requires no special kernel modules


## Using the CLI Tool

`ldprint` can be installed via `cargo install`.

From there, `ldprint` will connect to the display and print any provided message:

```
ldprint print "Hello world!"
```


## Using `ldprint` without root

On linux, connecting to a raw USB device usually requires root; connecting without root requires a custom `udev` rule,
usually placed under `/etc/udev/rules.d/`. While the precise format depends on your distro, it'll usually look something like this:

```
cat /etc/udev/rules.d/30-lcidisplay.rules
SUBSYSTEM=="usb", ATTRS{idVendor}=="0fa8", ATTRS{idProduct}=="a090", ACTION=="add", GROUP="wheel", MODE="0664"
```