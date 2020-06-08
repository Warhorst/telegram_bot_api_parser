use clap::Clap;

#[derive(Clap)]
pub struct ApiParserArguments {
    #[clap(short, long)]
    pub load_local: bool
}

impl ApiParserArguments {
    pub fn parse_arguments() -> Self {
        ApiParserArguments::parse()
    }
}