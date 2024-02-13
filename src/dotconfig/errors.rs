use super::Dotfile;
use std::io;

#[derive(Debug)]
pub struct DotfileLinkError(Dotfile, io::Error);

impl DotfileLinkError {
    pub(super) fn new(error: io::Error, dotfile: Dotfile) -> Self {
        Self(dotfile, error)
    }
}

#[derive(Debug)]
pub struct LinkError<'a>(pub &'a str, pub Vec<DotfileLinkError>);

impl<'a> LinkError<'a> {
    pub(super) fn new(summary: &'a str) -> Self {
        Self(summary, vec![])
    }
}
