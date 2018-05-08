#[derive(Debug,StructOpt)]
pub struct Bring {
    /// A list of repositories to fetch in "$ORGANIZATION/$PROJECT" GitHub format.
    pub repositories: Vec<String>
}
