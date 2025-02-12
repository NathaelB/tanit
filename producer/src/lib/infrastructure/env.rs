use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub messages: i32,
}
