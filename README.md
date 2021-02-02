# Captain Crunch

![u n da captain make it happen](https://github.com/graves/captain_crunch/raw/main/readme.png)

Captain Crunch is a modern wordlist generator that lets you specify a collection of character sets and then generate all possible permutations. It is the spiritual successor to [Crunch.](https://sourceforge.net/projects/crunch-wordlist/)

Captain Crunch is multi-threaded, relatively fast, and written in Rust.

## Usage

Download one of the prebuilt binaries or install [Rust](https://www.rust-lang.org/learn/get-started) and build from source using:

`cargo build --release`

The binary provides a `--help` flag for displaying usage information.

```
./captain_crunch --help

Captain Crunch 0.1.0
Thomas Graves <0o0o0o0o0@protonmail.ch>
Captain Crunch is a modern wordlist generator that lets you specify a collection of character sets
and then generate all possible permutations.

USAGE:
    captain_crunch [FLAGS] --config <FILE> --output <FILE>

FLAGS:
    -h, --help        Prints help information
    -p, --progress    Display progress bar (VERY SLOW!)
    -V, --version     Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file
    -o, --output <FILE>    Sets the file the wordlist will be written to

```

Captain Crunch requires a configuration file in yaml format beginning with `parts:` followed by a line for each part of the word you'd like generated. See `sample.yml` for an example:

``` yaml
parts:
  - 'c|C'
  - 'at'
  - '!|'
```

This sample file produces the following combinations:

```
cat!
cat
Cat!
Cat
```

As you can see:
- Each part of the word begins on it's own line and MUST BE SINGLE QUOTED (this is important for correctly parsing yaml files)
- The first letter can be either a capital or lowercase C (the possibilities are seperated by |)
- The second two letters are always: at
- The last character is either an ! or empty

This list can be infinitely long or complex but mind the complexity as the resulting wordlist sizes grow rather large, rather quickly.

The following command was used to build the example wordlist:

``` shell
./target/release/captain_crunch --progress --config sample.yml --output output.txt
```

Beware of using the `--progress` flag when generating extremely large lists as it substantially slows the process due to threads needing to wait for a second Mutex lock.

## Advanced

Captain Crunch configuration files also support more advanced options like character escaping and generating all possible strings based on a Regular Expression. See `sample_advanced.yml` for an example:

``` yaml
parts:
  - 'c|C|\|'
  - 'at'
  - '!|'
  - regex: '[a-z]{2}'
```

Starting with the first line we see that the \ (backslash) character is used to escape characters. This is useful if the | or ' characters are required in a part of the words you'd like to generate.

Looking at the last line, we see that we can start any line with `regex:` and follow it with a single quoted regex string. Captain Crunch will generate all possible strings that match the regular expression and use them to build the wordlist. This regex will end every word generated with exactly 2 lowercase characters in the english alphabet from a to z, for example:

```
aa
ab
ac
ad
...
...
zx
zy
zz
```

The `sample_advanced.yml` file generates 4056 different permutations. A portion of the output is displayed below:

```
Catdb
Catdc
Catdd
Catde
Catdf
Catdg
Catdh
Catdi
|atzs
|atzt
|atzu
|atzv
|atzw
|atzx
|atzy
|atzz
catsj
catbp
|at!wy
|atjr
|at!rx
|at!ts
Cat!zp
|atob
Cattd
|atzc
|at!tw
```

