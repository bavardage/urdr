# Urðr [![Build Status](https://travis-ci.com/bavardage/urdr.svg?branch=master)](https://travis-ci.com/bavardage/urdr)
<img src="https://upload.wikimedia.org/wikipedia/commons/4/48/Die_Nornen_Urd%2C_Werdanda%2C_Skuld%2C_unter_der_Welteiche_Yggdrasil_by_Ludwig_Burger.jpg" width="100" style="margin: auto"/>

Monitor and log the currently active window on OS X. These log files can then be analysed to determine where time is being spent.

## Usage

```
urdr 0.1.0

USAGE:
    urdr [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    The directory to output log files. Defaults to current directory
```

Files will be written in csv format.

<img src="example.gif" width="800"/>
