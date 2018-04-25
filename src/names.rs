use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Name<'a> {
    Ident(&'a str),
    Type(&'a str),
    Unique(&'a str, i32),
}

pub fn ident(s: &str) -> Name {
    Name::Ident(s)
}

pub fn typ(s: &str) -> Name {
    Name::Type(s)
}

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Name::Ident(s) => s.fmt(f),
            Name::Type(s) => s.fmt(f),
            Name::Unique(s, i) => f.write_fmt(format_args!("{}${}", s, i)),
        }
    }
}
