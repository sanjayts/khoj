# khoj (find implemented in Rust)
`find` implemented in Rust as part of reading "Command line rust" book. This repo uses the latest version of clap which has
quite a different API compared to the API used in the book.

This program supports the following capabilities:

```shell
khoj 0.1.0
sanjayts
find implemented in Rust

USAGE:
    khoj [OPTIONS] [--] [PATH]...

ARGS:
    <PATH>...    [default: .]

OPTIONS:
    -h, --help              Print help information
    -n, --name <NAME>...    
    -t, --type <TYPE>...    [possible values: d, f, l]
    -V, --version           Print version information
```

# Challenges/Thoughts

* I found this a bit boring to implement, especially given the vast number of flags and non-standard flag format used by the BSD version of `find`

# Reference

* https://man7.org/linux/man-pages/man1/find.1.html
* https://www.freebsd.org/cgi/man.cgi?find(1)
 