pub struct Config {
    pub query: String,
    pub subject: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let subject = args[2].clone();

        Ok(Config {
            query: query,
            subject: subject,
        })
    }
}