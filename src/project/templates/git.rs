use askama::Template;

use std::io::Write;

#[derive(Default, Template, WritableTemplate)]
#[template(path = "git/gitignore.j2")]
pub struct GitIgnore {
    pub golang: bool,
    pub java: bool,
    pub native: bool,
    pub node: bool,
    pub python: bool,
    pub ruby: bool,
    pub rust: bool,
}

impl GitIgnore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn golang(mut self) -> Self {
        self.golang = true;
        self
    }

    pub fn java(mut self) -> Self {
        self.java = true;
        self
    }

    pub fn native(mut self) -> Self {
        self.native = true;
        self
    }

    pub fn node(mut self) -> Self {
        self.node = true;
        self
    }

    pub fn python(mut self) -> Self {
        self.python = true;
        self
    }

    pub fn ruby(mut self) -> Self {
        self.ruby = true;
        self
    }

    pub fn rust(mut self) -> Self {
        self.rust = true;
        self
    }
}
