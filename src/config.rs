///# Interface
///
///cubes
///    - no arg: all cubes and dims info
///    - arg: cube name: cube info
///
///test
///    - no arg: all cubes
///    - arg: cube name
///
///flush
///    - arg/env var: key
///
///query
///    - arg: cube name
///    - option: drilldown
///    - option: cut
///    - option: measure
///    - flags: parents, debug, etc
///    - option: output (json, jsonrecord, csv)
///    - option: debug (url, js)
///
///
///global option/env var: base url
///

use failure::Error;
use std::env;
use structopt::StructOpt;

use api::ResponseFormat;

#[derive(StructOpt, Debug)]
#[structopt(
    name="Mondrian Rest Cli",
    about="Cli interface for Mondrian Rest API",
    after_help="NOTE:\n    \
        Multi-arg options can be specified using either one\n    \
        flag or several, the following are equivalent here:\n\n      \
        -o arg1 arg2 arg3\n      \
        -o arg1 -o arg2 -o arg3\n\n    \
        This is especially useful when constructing queriesi\n\n    \
        However cuts within a Dimension must be specified using\n    \
        one -c flag, with members comma-delimited:\n\n      \
        \"Geography.State.State.1,2,3\"
        ",
)]
pub struct Config {
    // TODO figure out how to not have this as Option,
    // because it's actuall required in the end, but just
    // not in the cli config
    #[structopt(
        short="b",
        long="base_url",
        help="Base url; this or env var MON_CLI_BASE_URL must be set",
    )]
    pub base_url: Option<String>,

    #[structopt(
        short="v",
        help="Verbose flag",
    )]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(
        name="describe",
        alias="d",
        about="Gets information about cubes",
    )]
    Describe {
        #[structopt(
            help="Describe specified cube; empty arg will retrieve all cubes")
        ]
        cube_name: Option<String>,

        #[structopt(
            requires="cube_name",
            short="m",
            long="members",
            help="Get members info for specified level (fully qualified name)",
        )]
        members: Option<String>,

        #[structopt(
            short="r",
            long="raw",
            help="raw output for description",
        )]
        raw: bool,
    },

    #[structopt(
        name="test",
        alias="t",
        about="Tests schema for errors",
    )]
    Test {
        #[structopt(
            help="Test specified cube; empty arg will test all cubes")
        ]
        cube_name: Option<String>,
    },

    #[structopt(
        name="flush",
        alias="f",
        about="Asks mondrian server to flush schema and cache and reset",
    )]
    Flush {
        #[structopt(
            help="Secret; this or env var MON_CLI_SECRET must be set")
        ]
        secret: Option<String>,
    },

    // TODO add options, and flush before query
    #[structopt(
        name="query",
        alias="q",
        about="Runs a query on a cube",
    )]
    Query {
        #[structopt(
            help="Query specified cube")
        ]
        cube_name: String,

        #[structopt(
            short="d",
            long="drilldown",
            help="Fully qualified name '.' delimited. Takes multiple.",
        )]
        drilldowns: Vec<String>,

        #[structopt(
            short="m",
            long="measure",
            help="Fully qualified name '.' delimited. Takes multiple.",
        )]
        measures: Vec<String>,

        #[structopt(
            short="c",
            long="cut",
            help="Fully qualified name '.' delimited. Takes multiple.",
        )]
        cuts: Vec<String>,

        #[structopt(
            short="p",
            long="property",
            help="Fully qualified name '.' delimited. Takes multiple.",
        )]
        properties: Vec<String>,

        #[structopt(
            long="debug",
        )]
        debug: bool,

        #[structopt(
            long="parents",
        )]
        parents: bool,

        #[structopt(
            long="nonempty",
        )]
        nonempty: bool,

        #[structopt(
            long="distinct",
        )]
        distinct: bool,

        #[structopt(
            long="sparse",
        )]
        sparse: bool,

        #[structopt(
            short="f",
            long="format",
            help="json, jsonrecords, or csv",
            default_value="json",
        )]
        format: ResponseFormat,
    }
}

pub fn get_config() -> Result<Config, Error> {
    let mut config = Config::from_args();
    // check base url presence
    // TODO parse to url path?
    if config.base_url.is_none() {
        if let Ok(base_url) = env::var("MON_CLI_BASE_URL") {
            config.base_url = Some(base_url);
        } else {
            bail!("Base url must be supplied");
        }
    }

    // check secret presence
    if let Command::Flush{ref mut secret, ..} = config.cmd {
        if secret.is_none() {
            if let Ok(s) = env::var("MON_CLI_SECRET") {
                *secret = Some(s);
            } else {
                bail!("Secret must be supplied");
            }
        }
    }

    // see if this error check can be pushed to structopt
    // check that query has query has at least one drilldown
    // and at least one measure
    if let Command::Query{ref drilldowns, ref measures, ..} = config.cmd {
        if drilldowns.is_empty() || measures.is_empty() {
            bail!("Dimension and measure must be supplied");
        }
    }

    Ok(config)
}
