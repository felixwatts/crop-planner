use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ParamsPrint {
    #[structopt(short, long)]
    pub bed: Option<std::string::String>,

    #[structopt(short, long)]
    pub week: Option<usize>
}

#[derive(Debug, StructOpt)]
pub struct ParamsInstruct {
    #[structopt(short, long)]
    pub week: usize
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "init")]
    Init,

    #[structopt(name = "solve")]
    Solve,

    #[structopt(name = "reset")]
    Reset,

    #[structopt(name = "print")]
    Print(ParamsPrint),

    #[structopt(name = "tasks")]
    Instruct(ParamsInstruct)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "harvest", about = "A crop planning tool for market gardeners")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Cmd,

    #[structopt(short, long, default_value = "params.json", parse(from_os_str))]
    params: std::path::PathBuf,
}

