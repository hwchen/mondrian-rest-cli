# Mondrian Rest CLI

cli utility for interacting with [mondrian-rest](https://github.com/jazzido/mondrian-rest)

# Interface

cubes
    - no arg: all cubes and dims info
    - arg: cube name: cube info

test
    - no arg: all cubes
    - arg: cube name

flush
    - arg/env var: key

query
    - arg: cube name
    - option: drilldown
    - option: cut
    - option: measure
    - flags: parents, debug, etc
    - option: output (json, jsonrecord, csv)
    - option: debug (url, js)


global option/env var: base url

# Dependencies
Testing out some new libs

- anterofit (rest client framework)
- structopt (clap with custom derive)
- failure (error management, moving on from error-chain)
