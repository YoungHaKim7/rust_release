use clap::Subcommand;

#[derive(Subcommand)]
pub enum Pr {
    /// Create a pull request
    Create {
        /// The PR's title
        #[arg(long, short)]
        title: String,
        /// Mark the PR as a draft
        #[arg(long, short)]
        draft: bool,
    },
    /// List pull request
    List,
}

impl Pr {
    pub fn exec(&self) {
        match self {
            Pr::Create { title, draft } => {
                println!("Pr with title {title} is created and \n the draft status is: {draft} ")
            }
            Pr::List => {
                println!("List Pressed")
            }
        }
    }
}
