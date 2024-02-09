use std::path::Path;

#[derive(Copy, Clone)]
pub struct RootMatchLocation<'a> {
    pub root: &'a Path,
}

#[derive(Copy, Clone)]
pub struct FileMatchLocation<'a> {
    pub root: &'a Path,
    pub file: &'a Path,
}

impl<'a> FileMatchLocation<'a> {
    pub fn from_root(root: &RootMatchLocation<'a>, file: &'a Path) -> FileMatchLocation<'a> {
        FileMatchLocation {
            root: root.root,
            file,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LineMatchLocation<'a> {
    pub root: &'a Path,
    pub file: &'a Path,
    pub line: usize,
}

impl<'a> LineMatchLocation<'a> {
    pub fn from_file(file: &FileMatchLocation<'a>, line: usize) -> LineMatchLocation<'a> {
        LineMatchLocation {
            root: file.root,
            file: file.file,
            line,
        }
    }
}

pub enum MatchLocation<'a> {
    #[allow(dead_code)]
    Root(RootMatchLocation<'a>),
    File(FileMatchLocation<'a>),
    Line(LineMatchLocation<'a>),
}
