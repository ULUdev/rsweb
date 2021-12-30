use std::env::args;

pub struct Arguments {
    pub logfile: Option<String>,
    pub configfile: Option<String>,
}

impl Arguments {
    /// load arguments
    pub fn load() -> Arguments {
        let mut arguments = args();
        let mut out = Arguments {
            logfile: None,
            configfile: None,
        };
        while let Some(n) = arguments.next() {
            match n.as_str() {
                "-h" | "--help" => {
                    eprintln!(
                        "
USAGE
  rsweb [OPTIONS]

OPTIONS
  -h,--help: print this help and exit
  -l,--logfile <logfile>: log to <logfile> instead of default or configured logfile
  -c,--config <config>: use <config> as a config file instead of default
  -v,--version: print the version and exit
                            "
                    );
                    std::process::exit(0);
                }
                "-l" | "--logfile" => {
                    if let Some(lf) = arguments.next() {
                        out.logfile = Some(lf);
                    } else {
                        eprintln!("no logfile provided");
                        std::process::exit(1);
                    }
                }
                "-c" | "--config" => {
                    if let Some(conf) = arguments.next() {
                        out.configfile = Some(conf);
                    } else {
                        eprintln!("no config provided");
                        std::process::exit(1);
                    }
                }
                "-v" | "--version" => {
                    eprintln!("rsweb: version {}", crate::RSWEB_VERSION);
                    std::process::exit(0);
                }
                _ => (),
            }
        }
        out
    }
}
