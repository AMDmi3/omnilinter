use std::path::Path;

pub struct RootMatchLocation<'a> {
    pub root: &'a Path,
}

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
    Root(RootMatchLocation<'a>),
    File(FileMatchLocation<'a>),
    Line(LineMatchLocation<'a>),
}
