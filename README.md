# Mondrian Rest CLI

cli utility for interacting with [mondrian-rest](https://github.com/jazzido/mondrian-rest)

Works with mondrian-rest v.0.7.9. Not guaranteed to work with older versions, although many of the options should work.

# Examples

Describe one cube
```
mondrian-rest-cli -b http://10.100.10.10:5000 describe exports
```

Get members of a level (key: caption)
```
mondrian-rest-cli -b http://10.100.10.10:5000 describe exports -m 'Category.Category'
```

Query with drilldown, measure, and cut and format csv; sparse and debug options
```
mondrian-rest-cli -b http://10.100.10.10:5000 q exports -d 'Geography.County' -m 'Dollars Sum' -c 'Year.Year.2016' --sparse --debug -f csv
```

Test a cube
```
mondrian-rest-cli -b http://10.100.10.10:5000 t exports
```

Flush the mondrian server
```
mondrian-rest-cli -b http://10.100.10.10:5000 f secret123
```

# Installation
## From Source
- Install [rustup](https://rustup.rs)
- `$ cargo update`
- `$ cargo install --git https://github.com/hwchen/mondrian-rest-cli`

## Download binaries
- coming soon

# Usage
Note: all subcommands have alias of the first letter of the subcommand.

Note on naming:

- The easiest way to name levels is to separate each part of a level name by a period, e.g. "Geography.County".
- Names with square brackets should also parse correctly; please file a bug if there's a mistake. e.g. "[Geography].[County]"
- Names without a hierarchy will use the default hierarchy of the Dimension name.
- For member names, the following should all be equivalent. Note that there is easy syntax for multiple members:
  - Geography.County.1,2
  - Geography.County.&1
  - Geography.County.&1,&2
  - [Geography].[County].&[1,2]
  - [Geography].[County].[&1,&2]
  I try to remove as many leading ampersands as possible. File a bug if something unexpected happens.
  (This is _not_ how Mondrian parses multiple members afaik, this is just for cli convenience)

```
USAGE:
    mondrian-rest-cli [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Verbose flag

OPTIONS:
    -b, --base_url <base_url>    Base url; this or env var MON_CLI_BASE_URL must be set

SUBCOMMANDS:
    describe    Gets information about cubes
    flush       Asks mondrian server to flush schema and cache and reset
    help        Prints this message or the help of the given subcommand(s)
    query       Runs a query on a cube
    test        Tests schema for errors

NOTE:
    Multi-arg options can be specified using either one
    flag or several, the following are equivalent here:

    -o arg1 arg2 arg3
    -o arg1 -o arg2 -o arg3

    This is especially useful when constructing queries
```

## describe
Fetch description of a cube or cubes in schema.

```
selected FLAGS:
    -r, --raw        raw output for description

OPTIONS:
    -m, --members <members>    Get members info for specified level (fully qualified name)

ARGS:
    <cube_name>    Describe specified cube; empty arg will retrieve all cubes
```

## flush
Refresh Mondrian server

```
ARGS:
    <secret>    Secret; this or env var MON_CLI_SECRET must be set
```

## test
Testing for runtime errors such as wrong db columns.

The basic strategy is to construct queries which include one dim and all measures, for all dims.

Then to also construct queries for each property.

Note: named sets for testing not yet supported.

```
ARGS:
    <cube_name>    Test specified cube; empty arg will test all cubes
```

## query
Constructs general query to mondrian rest server.

Note on cuts:

There can be cuts on multiple dimensions, just use `-c` multiple times. One cut of a dimension can contain multiple members, e.g. `Geography.County.1,2,3`.

```
selected FLAGS:
    --debug
    --distinct
    --nonempty
    --parents
    --sparse

OPTIONS:
-c, --cut <cuts>...                Fully qualified name '.' delimited. Takes multiple.
-d, --drilldown <drilldowns>...    Fully qualified name '.' delimited. Takes multiple.
-f, --format <format>              json, jsonrecords, or csv [default: json]
-m, --measure <measures>...        Fully qualified name '.' delimited. Takes multiple.
-p, --property <properties>...     Fully qualified name '.' delimited. Takes multiple.

ARGS:
<cube_name>    Query specified cube

```

# Future work

- increase timeout?
- CI and binary releases
- redo url builder as state machine
- separate http request execution from url builder

