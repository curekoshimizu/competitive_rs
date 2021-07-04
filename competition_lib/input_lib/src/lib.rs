use std::str::FromStr;

#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{
        input_inner!{$sc,$($r)*}
    };
    ($($r:tt)*)=>{
        let mut sc = Scanner::new(std::io::stdin().lock());
        input_inner!{sc,$($r)*}
    };
}

#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{
        let $var=read_value!($sc,$t);
        input_inner!{$sc $($r)*}
    };
}

#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{
        ($(read_value!($sc,$t)),*)
    };
    ($sc:expr,[$t:tt;$len:expr])=>{
        (0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()
    };
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>().unwrap()};
}
pub struct Scanner {
    s: Box<str>,
    input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,
}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let s = {
            let mut s = String::new();
            reader.read_to_string(&mut s).unwrap();
            s.into_boxed_str()
        };
        let mut sc = Scanner {
            s,
            input: "".split_ascii_whitespace().peekable(),
        };
        use std::mem;
        let s: &'static str = unsafe { mem::transmute(&*sc.s) };
        sc.input = s.split_ascii_whitespace().peekable();
        sc
    }
    #[inline]
    pub fn next<T: FromStr>(&mut self) -> Option<T>
    where
        T::Err: std::fmt::Debug,
    {
        Some(self.input.next()?.parse::<T>().expect("Parse error"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_u32() {
        let cursor = io::Cursor::new(b"0 2");
        let mut sc = Scanner::new(cursor);
        input! {
            sc=sc,
            n: u32,
            m: u32,
        }
        assert_eq!(n, 0);
        assert_eq!(m, 2);
    }

    #[test]
    fn test_array() {
        let cursor = io::Cursor::new(b"2 1 3");
        let mut sc = Scanner::new(cursor);
        input! {
            sc=sc,
            n: u32,
            a: [u32; n],
        }
        assert_eq!(n, 2);
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], 1);
        assert_eq!(a[1], 3);
    }

    #[test]
    fn test_chars() {
        let cursor = io::Cursor::new(b"abcd");
        let mut sc = Scanner::new(cursor);
        input! {
            sc=sc,
            s: Chars, // Vec<char>
        }
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], 'a');
        assert_eq!(s[1], 'b');
        assert_eq!(s[2], 'c');
        assert_eq!(s[3], 'd');
    }
    #[test]
    fn test_scanner_next() {
        let cursor = io::Cursor::new(b"1 2 a bcd");
        let mut sc = Scanner::new(cursor);
        assert_eq!(sc.next::<u32>(), Some(1));
        assert_eq!(sc.next::<u32>(), Some(2));
        assert_eq!(sc.next::<char>(), Some('a'));
        assert_eq!(sc.next::<String>(), Some("bcd".to_string()));
        assert_eq!(sc.next::<String>(), None);
    }
}
