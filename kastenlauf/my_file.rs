use whiteread as w;
use w::prelude::*;

fn run() -> w::ReaderResult<()> {
    let input = std::io::stdin();
    let input = input.lock();
    let mut input = w::Reader::new(input);

    let _x: i32 = input.line()?;

    Ok(())
}

fn main() {
    run().pretty_unwrap()
}

// From https://github.com/krdln/whiteread on MIT license
#[allow(dead_code)]
mod whiteread {
    use std::path::Path;
    use std::io;
    pub mod stream {
        use std::str::SplitWhitespace;
        use std::io;
        pub trait StrStream {
            fn next(&mut self) -> io::Result<Option<&str>>;
        }
        impl<'a> StrStream for SplitWhitespace<'a> {
            fn next(&mut self) -> io::Result<Option<&str>> {
                Ok(Iterator::next(self))
            }
        }
        pub struct SplitAsciiWhitespace<'a> {
            s: &'a str,
            position: usize,
        }
        impl<'a> SplitAsciiWhitespace<'a> {
            pub fn new(s: &'a str) -> Self { SplitAsciiWhitespace { s: s, position: 0 } }
            pub fn position(&self) -> usize { self.position }
            pub fn from_parts(s: &'a str, position: usize) -> Self {
                SplitAsciiWhitespace { s: s, position: position }
            }
        }
        impl<'a> Iterator for SplitAsciiWhitespace<'a> {
            type Item = &'a str;
            fn next(&mut self) -> Option<&'a str> {
                let bytes = self.s.as_bytes();
                let mut start = self.position;
                while let Some(&c) = bytes.get(start) {
                    if c > b' ' {
                        break;
                    }
                    start += 1;
                }
                let mut end = start;
                while let Some(&c) = bytes.get(end) {
                    if c <= b' ' {
                        break;
                    }
                    end += 1;
                }
                self.position = end;
                if start != end {
                    Some(&self.s[start..end])
                } else {
                    None
                }
            }
        }
        impl<'a> StrStream for SplitAsciiWhitespace<'a> {
            fn next(&mut self) -> io::Result<Option<&str>> {
                Ok(Iterator::next(self))
            }
        }
        pub trait StrExt {
            fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace;
        }
        impl StrExt for str {
            fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace {
                SplitAsciiWhitespace::new(self)
            }
        }
    }
    pub mod white {
        use super::stream::StrStream;
        use std::io;
        use std::result;
        pub trait White: Sized {
            fn read<I: StrStream>(it: &mut I) -> Result<Self>;
        }
        pub type Result<T> = result::Result<T, Error>;
        #[derive(Debug)]
        pub enum Error {
            TooShort,
            Leftovers,
            ParseError,
            IoError(io::Error),
        }
        pub use self::Error::*;
        impl Error {
            pub fn is_too_short(&self) -> bool {
                match *self {
                    TooShort => true,
                    _ => false,
                }
            }
            pub fn is_leftovers(&self) -> bool {
                match *self {
                    Leftovers => true,
                    _ => false,
                }
            }
            pub fn is_parse_error(&self) -> bool {
                match *self {
                    ParseError => true,
                    _ => false,
                }
            }
            pub fn is_io_error(&self) -> bool {
                match *self {
                    IoError(_) => true,
                    _ => false,
                }
            }
        }
        impl From<io::Error> for Error {
            fn from(e: io::Error) -> Error {
                IoError(e)
            }
        }
        impl ::std::error::Error for Error {
            fn description(&self) -> &str {
                match *self {
                    TooShort => "not enough input to parse a value",
                    Leftovers => "excessive input provided",
                    ParseError => "parse error occured",
                    IoError(ref e) => e.description(),
                }
            }
            fn cause(&self) -> Option<&::std::error::Error> {
                match *self {
                    IoError(ref e) => e.cause(),
                    _ => None,
                }
            }
        }
        impl ::std::fmt::Display for Error {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use ::std::error::Error;
                match *self {
                    IoError(ref e) => e.fmt(fmt),
                    _ => fmt.write_str(self.description()),
                }
            }
        }
        impl From<Error> for io::Error {
            fn from(e: Error) -> io::Error {
                match e {
                    IoError(e) => e,
                    e => io::Error::new(io::ErrorKind::InvalidData, e),
                }
            }
        }
        macro_rules! white {
            ($T:ident) => (
                impl White for $T {
                    fn read<I: StrStream>(it: &mut I) -> Result<$T> {
                        try!( it.next() ).ok_or(TooShort).and_then( |s| s.parse().or(Err(ParseError)) )
                    }
                }
            )
        }
        white!(bool);
        white!(u8);
        white!(u16);
        white!(u32);
        white!(u64);
        white!(usize);
        white!(i8);
        white!(i16);
        white!(i32);
        white!(i64);
        white!(isize);
        white!(String);
        white!(f32);
        white!(f64);
        impl White for char {
            fn read<I: StrStream>(it: &mut I) -> Result<char> {
                let s = it.next()?;
                s.and_then(|s| s.chars().next()).ok_or(TooShort)
            }
        }
        macro_rules! impl_tuple {
            ( $($x:ident),* ) => {
                impl< $( $x: White ),* > White for ( $( $x, )* ) {
                    fn read<I: StrStream>(_it: &mut I) -> Result<Self> {
                        Ok(( $( $x::read(_it)?, )* ))
                    }
                }
            };
        }
        impl_tuple!();
        impl_tuple!(A);
        impl_tuple!(A, B);
        impl_tuple!(A, B, C);
        impl_tuple!(A, B, C, D);
        impl_tuple!(A, B, C, D, E);
        impl_tuple!(A, B, C, D, E, F);
        impl<T: White> White for Vec<T> {
            fn read<I: StrStream>(it: &mut I) -> Result<Vec<T>> {
                let mut v = vec![];
                loop {
                    match White::read(it) {
                        Err(TooShort) => break,
                        x => v.push(x?),
                    }
                }
                Ok(v)
            }
        }
        #[derive(Default, Debug, Eq, PartialEq)]
        pub struct Skip;
        impl White for Skip {
            fn read<I: StrStream>(it: &mut I) -> Result<Skip> {
                it.next()?;
                Ok(Skip)
            }
        }
        #[derive(Default, Debug, Eq, PartialEq)]
        pub struct SkipAll;
        impl White for SkipAll {
            fn read<I: StrStream>(it: &mut I) -> Result<SkipAll> {
                while let Some(_) = it.next()? {};
                Ok(SkipAll)
            }
        }
        #[derive(Default, Debug, Eq, PartialEq)]
        pub struct Lengthed<T>(pub Vec<T>);
        impl<T: White> White for Lengthed<T> {
            fn read<I: StrStream>(it: &mut I) -> Result<Lengthed<T>> {
                let sz = White::read(it)?;
                let mut v = Vec::with_capacity(sz);
                loop {
                    if v.len() == sz {
                        return Ok(Lengthed(v));
                    }
                    v.push(White::read(it)?);
                }
            }
        }
        #[derive(Default, Debug)]
        pub struct Zeroed<T>(pub Vec<T>);
        impl<T: White + Default + PartialEq> White for Zeroed<T> {
            fn read<I: StrStream>(it: &mut I) -> Result<Zeroed<T>> {
                let mut v = vec![];
                let zero = Default::default();
                loop {
                    let x = White::read(it)?;
                    if x == zero {
                        return Ok(Zeroed(v));
                    } else {
                        v.push(x)
                    }
                }
            }
        }
    }
    pub use self::white::{White, Skip, SkipAll, Lengthed, Zeroed};
    pub use self::white::{TooShort, ParseError, Leftovers};
    pub mod reader {
        use std::io;
        use std::fmt;
        use std::path::Path;
        use std::fs;
        use std::error::Error as StdError;
        use super::white;
        use super::white::Error::*;
        use super::White;
        use super::stream::StrStream;
        use super::stream::SplitAsciiWhitespace;
        pub struct Reader<B: io::BufRead> {
            buf: B,
            row: u64,
            line: String,
            col: usize,
        }
        unsafe fn erase_lifetime<'a, 'b, T: 'a + 'b>(x: &'a mut T) -> &'b mut T {
            &mut *(x as *mut _)
        }
        impl<B: io::BufRead> Reader<B> {
            pub fn new(buf: B) -> Reader<B> {
                Reader {
                    buf: buf,
                    row: 0,
                    line: String::new(),
                    col: 0,
                }
            }
        }
        impl Reader<io::BufReader<fs::File>> {
            pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Reader<io::BufReader<fs::File>>> {
                let file = fs::File::open(path)?;
                Ok(Reader::new(io::BufReader::new(file)))
            }
        }
        impl<B: io::BufRead> Reader<B> {
            pub fn continue_<T: White>(&mut self) -> BorrowedResult<T> {
                White::read(self).add_lineinfo(self)
            }
            pub fn parse<T: White>(&mut self) -> BorrowedResult<T> {
                White::read(self).add_lineinfo(self)
            }
            pub fn p<T: White>(&mut self) -> T {
                self.parse().unwrap()
            }
            pub fn finish<T: White>(&mut self) -> BorrowedResult<T> {
                let value = unsafe { erase_lifetime(self) }.parse()?;
                if let Ok(Some(_)) = StrStream::next(self) {
                    Err(Leftovers).add_lineinfo(self)
                } else {
                    Ok(value)
                }
            }
        }
        impl<B: io::BufRead> Reader<B> {
            fn read_line(&mut self) -> io::Result<Option<()>> {
                self.row += 1;
                self.line.clear();
                self.col = 0;
                let n_bytes = self.buf.read_line(&mut self.line)?;
                if n_bytes == 0 {
                    return Ok(None);
                }
                Ok(Some(()))
            }
            fn next_within_line(&mut self) -> Option<&str> {
                let mut splitter = SplitAsciiWhitespace::from_parts(&self.line, self.col);
                let ret = Iterator::next(&mut splitter);
                self.col = splitter.position();
                ret
            }
            pub fn line<T: White>(&mut self) -> BorrowedResult<T> {
                if let None = self.read_line()? {
                    return Err(TooShort).add_lineinfo(self);
                };
                self.finish_line()
            }
            pub fn start_line<T: White>(&mut self) -> BorrowedResult<T> {
                if let None = self.read_line()? {
                    return Err(TooShort).add_lineinfo(self);
                };
                self.continue_line()
            }
            pub fn continue_line<T: White>(&mut self) -> BorrowedResult<T> {
                let result = {
                    let mut splitter = SplitAsciiWhitespace::from_parts(&self.line, self.col);
                    let result = White::read(&mut splitter);
                    self.col = splitter.position();
                    result
                };
                result.add_lineinfo(self)
            }
            pub fn finish_line<T: White>(&mut self) -> BorrowedResult<T> {
                let value = unsafe { erase_lifetime(self) }.continue_line()?;
                if let Some(_) = self.next_within_line() {
                    Err(Leftovers).add_lineinfo(self)
                } else {
                    Ok(value)
                }
            }
        }
        impl<B: io::BufRead> Reader<B> {
            pub fn next_line(&mut self) -> BorrowedResult<&str> {
                if let None = self.read_line()? {
                    return Err(TooShort).add_lineinfo(self);
                }
                Ok(&self.line)
            }
            pub fn into_inner(self) -> B {
                self.buf
            }
        }
        impl<B: io::BufRead> StrStream for Reader<B> {
            fn next(&mut self) -> io::Result<Option<&str>> {
                loop {
                    match unsafe { erase_lifetime(self) }.next_within_line() {
                        None => (),
                        some => return Ok(some),
                    }
                    if let None = self.read_line()? {
                        return Ok(None);
                    };
                }
            }
        }
        #[derive(Debug)]
        pub struct BorrowedError<'line> {
            error: white::Error,
            line: &'line str,
            row: u64,
            col: usize,
        }
        #[derive(Debug)]
        pub struct OwnedError {
            error: white::Error,
            line: Box<str>,
            row: u64,
            col: usize,
        }
        impl<'a> BorrowedError<'a> {
            pub fn to_owned(self) -> OwnedError {
                OwnedError {
                    error: self.error,
                    line: self.line.to_owned().into_boxed_str(),
                    row: self.row,
                    col: self.col,
                }
            }
            pub fn into_inner(self) -> white::Error { self.error }
            pub fn location(&self) -> Option<(u64, usize)> {
                if self.row > 0 { Some((self.row, self.col)) } else { None }
            }
        }
        impl OwnedError {
            pub fn into_inner(self) -> white::Error { self.error }
            pub fn location(&self) -> Option<(u64, usize)> {
                if self.row > 0 { Some((self.row, self.col)) } else { None }
            }
        }
        impl StdError for OwnedError {
            fn description(&self) -> &str { self.error.description() }
            fn cause(&self) -> Option<&StdError> { Some(&self.error) }
        }
        impl<'a> AsRef<white::Error> for BorrowedError<'a> {
            fn as_ref(&self) -> &white::Error { &self.error }
        }
        impl AsRef<white::Error> for OwnedError {
            fn as_ref(&self) -> &white::Error { &self.error }
        }
        impl From<io::Error> for OwnedError {
            fn from(e: io::Error) -> OwnedError {
                BorrowedError::from(e).to_owned()
            }
        }
        impl<'a> From<BorrowedError<'a>> for OwnedError {
            fn from(e: BorrowedError<'a>) -> OwnedError {
                e.to_owned()
            }
        }
        impl<'a> From<BorrowedError<'a>> for Box<StdError> {
            fn from(e: BorrowedError<'a>) -> Box<StdError> {
                Box::from(e.to_owned())
            }
        }
        impl<'a> From<BorrowedError<'a>> for Box<StdError + Send + Sync> {
            fn from(e: BorrowedError<'a>) -> Self {
                Box::from(e.to_owned())
            }
        }
        impl<'a> From<io::Error> for BorrowedError<'a> {
            fn from(e: io::Error) -> BorrowedError<'a> {
                BorrowedError { error: white::IoError(e), row: 0, col: 0, line: "" }
            }
        }
        fn display(
            error: &white::Error,
            line: &str,
            row: u64,
            mut col: usize,
            f: &mut fmt::Formatter
        ) -> fmt::Result {
            write!(f, "{}", error)?;
            if row != 0 {
                let line = line.trim_right_matches(&['\n', '\r'][..]);
                if line.len() <= 120 {
                    if col > line.len() {
                        col = line.len()
                    }
                    if (error.is_parse_error() || error.is_leftovers()) && col > 0 {
                        col -= 1;
                    }
                    writeln!(f, " at")?;
                    let number = row.to_string();
                    write!(f, "{}| ", number)?;
                    writeln!(f, "{}", line)?;
                    for _ in 0 .. number.len() + 2 {
                        write!(f, " ")?;
                    }
                    for c in line[..col].chars() {
                        if c <= b' ' as char {
                            write!(f, "{}", c)?;
                        } else {
                            write!(f, " ")?;
                        }
                    }
                    write!(f, "^")?;
                } else {
                    write!(f, " at line {}, column {}", row, col + 1)?;
                }
            }
            Ok(())
        }
        impl<'a> fmt::Display for BorrowedError<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                display(&self.error, self.line, self.row, self.col, f)
            }
        }
        impl fmt::Display for OwnedError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                display(&self.error, &self.line, self.row, self.col, f)
            }
        }
        pub type BorrowedResult<'line, T> = ::std::result::Result<T, BorrowedError<'line>>;
        pub type OwnedResult<T> = ::std::result::Result<T, OwnedError>;
        pub trait BorrowedResultExt<'a, T> {
            fn none_on_too_short(self) -> BorrowedResult<'a, Option<T>>;
            fn to_owned(self) -> OwnedResult<T>;
        }
        pub trait PrettyUnwrap {
            type Target;
            fn pretty_unwrap(self) -> Self::Target;
        }
        impl<'a, T> BorrowedResultExt<'a, T> for BorrowedResult<'a, T> {
            fn none_on_too_short(self) -> BorrowedResult<'a, Option<T>> {
                match self {
                    Ok(x) => Ok(Some(x)),
                    Err(BorrowedError { error: TooShort, .. } ) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            fn to_owned(self) -> OwnedResult<T> {
                self.map_err(BorrowedError::to_owned)
            }
        }
        impl<T, E> PrettyUnwrap for Result<T, E>
        where E: fmt::Display {
            type Target = T;
            fn pretty_unwrap(self) -> T {
                match self {
                    Ok(x) => x,
                    Err(e) => {
                        use self::io::Write;
                        writeln!(io::stderr(), "{}", e).ok();
                        panic!("PrettyUnwrap::pretty_unwrap failed");
                    }
                }
            }
        }
        fn add_lineinfo<'line, B>(error: white::Error, reader: &'line Reader<B>) -> BorrowedError<'line>
        where B: io::BufRead {
            BorrowedError {
                error: error,
                row: reader.row,
                col: reader.col,
                line: &reader.line,
            }
        }
        trait AddLineinfoExt<T> {
            fn add_lineinfo<'line, B>(self, reader: &'line Reader<B>) -> BorrowedResult<'line, T>
            where B: io::BufRead;
        }
        impl<T> AddLineinfoExt<T> for white::Result<T> {
            fn add_lineinfo<'a, B>(self, reader: &'a Reader<B>) -> BorrowedResult<'a, T>
            where B: io::BufRead {
                self.map_err(|e| add_lineinfo(e, reader))
            }
        }
    }
    pub use self::reader::Reader;
    pub use self::reader::OwnedError as ReaderError;
    pub use self::reader::OwnedResult as ReaderResult;
    pub mod prelude {
        pub use super::reader::BorrowedResultExt;
        pub use super::reader::PrettyUnwrap;
    }
    pub fn parse_line<T: White>() -> white::Result<T> {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        parse_string(&line)
    }
    pub fn parse_string<T: White>(s: &str) -> white::Result<T> {
        let mut stream = stream::SplitAsciiWhitespace::new(s);
        let value = White::read(&mut stream)?;
        if let Ok(Some(_)) = stream::StrStream::next(&mut stream) {
            Err(Leftovers)
        } else {
            Ok(value)
        }
    }
    pub fn parse_file<T: White, P: AsRef<Path>>(path: P) -> ReaderResult<T> {
        Ok( Reader::open(path)?.finish()? )
    }
}
