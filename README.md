# rptree
A command line tool for generating directory tree.

## Install
`cargo insall rptree` or go to the [release](https://github.com/night-cruise/rptree/releases) page to download rptree.exe

## Usage
Enter `rptree - h` on the command line to view the detailed usage commands:
```text
Rp Tree 0.3.0
An useful tool for generating directory tree

USAGE:
    rptree.exe [FLAGS] [OPTIONS] <ROOT_DIR>

FLAGS:
    -d, --dir-only    Only output dir
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -c, --color <color>      Prints color, only supports green, blue and red
    -o, --output <output>    Prints to file

ARGS:
    <ROOT_DIR>    Root dir for generate directory tree
```
## LICENSE
This project is licensed under the MIT License (see the
[LICENSE](LICENSE) file for details).
