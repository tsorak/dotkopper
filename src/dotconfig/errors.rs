use super::Dotfile;
use std::io;

#[derive(Debug)]
pub struct DotfileLinkError(Dotfile, io::Error);

impl DotfileLinkError {
    pub(super) fn new(error: io::Error, dotfile: Dotfile) -> Self {
        Self(dotfile, error)
    }

    pub fn format_error(&self) -> String {
        let Self(
            Dotfile {
                target: t,
                origin: o,
                ..
            },
            io_error,
        ) = self;

        let io_error = io_error.to_string();

        format!("[{}] '{}' -> '{}'", io_error, o.display(), t.display())
    }
}

#[derive(Debug)]
pub struct LinkError<'a>(pub &'a str, pub Vec<DotfileLinkError>);

impl<'a> LinkError<'a> {
    pub(super) fn new(summary: &'a str) -> Self {
        Self(summary, vec![])
    }
}
