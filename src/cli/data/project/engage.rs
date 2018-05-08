#[derive(Debug,StructOpt)]
pub struct Engage {
    /// The project to engage in "$ORGANIZATION/$PROJECT" GitHub format.
    pub project: String,
}
