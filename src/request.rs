use std::{
    fmt::Display,
    io::{self, BufRead},
    net::TcpStream,
    time::Duration,
};

use crate::error::MyError;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
}

impl TryFrom<TcpStream> for Request {
    type Error = MyError;

    fn try_from(mut stream: TcpStream) -> Result<Self, Self::Error> {
        println!("\tReading request from {:?}", stream.local_addr());
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("Failed to set read timeout on stream");
        let mut reader = io::BufReader::new(&mut stream);
        let text: Vec<u8> = reader.fill_buf()?.to_vec();
        reader.consume(text.len());

        let mut cursor = 0usize;
        let method = read_token(&text[..], &mut cursor).try_into()?;
        consume_whitespace(&text, &mut cursor);
        let path =
            String::from_utf8(read_request_target(&text[cursor..], &mut cursor).to_vec()).unwrap();

        Ok(Self { method, path })
    }
}

fn consume_whitespace(r: &[u8], cursor: &mut usize) {
    loop {
        if r[*cursor].is_ascii_whitespace() {
            *cursor += 1;
        } else {
            break;
        }
    }
}

fn read_token<'a>(r: &'a [u8], cursor: &mut usize) -> &'a [u8] {
    for (i, c) in r.iter().enumerate() {
        if !is_tchar(*c) {
            *cursor += i;
            return &r[..i];
        }
    }
    r
}

/// Returns true if the given character is a tchar (token character), as defined in [Appendix B of RFC 7230](https://datatracker.ietf.org/doc/html/rfc7230#appendix-B)
fn is_tchar(c: u8) -> bool {
    c.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&c)
}

fn read_request_target<'a>(r: &'a [u8], cursor: &mut usize) -> &'a [u8] {
    for (i, c) in r.iter().enumerate() {
        if c.is_ascii_whitespace() {
            *cursor += i;
            return &r[..i];
        }
    }
    r
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.method, self.path)
    }
}

/// HTTP request methods
///
/// [Section 4 of [RFC ]](https://datatracker.ietf.org/doc/html/rfc7231#section-4)
#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl TryFrom<&[u8]> for Method {
    type Error = MyError;

    fn try_from(token: &[u8]) -> Result<Self, Self::Error> {
        match token {
            b"GET" => Ok(Method::Get),
            b"HEAD" => Ok(Method::Head),
            b"POST" => Ok(Method::Post),
            b"PUT" => Ok(Method::Put),
            b"DELETE" => Ok(Method::Delete),
            b"CONNECT" => Ok(Method::Connect),
            b"OPTIONS" => Ok(Method::Options),
            b"TRACE" => Ok(Method::Trace),
            _ => Err(MyError::InvalidMethodToken(
                String::from_utf8(token.to_vec()).unwrap(),
            )),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Trace => write!(f, "TRACE"),
        }
    }
}
