# rptree
*A command line tool for generating directory tree.*

## Install
`cargo install rptree`

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
    -f, --filter <filter>... Exclude specific directories and their children from the tree
    -o, --output <output>    Prints to file

ARGS:
    <ROOT_DIR>    Root dir for generate directory tree
```

## Examples
`rptree . --color red --filter target .git`
In this example, the 'target' directory and the '.git' directory, and all of their children, will be excluded from the tree which will be built from the current directory

## Reference
* https://pypi.org/project/rptree/

## LICENSE
This project is licensed under the MIT License (see the
[LICENSE](LICENSE) file for details).
