use crate::http::request;

use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    //<'buf> ' + 문자 = 수명의 나타냄, 수명은 제네릭
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl Request<'_> {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // 수명을 impl 앞에서 먼저 선언해준다
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err((ParseError::InvalidRequest)),
        // }
        // -> 다음줄과 같은 코드,

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        println!("request: {}", request);
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // let mut query_string = None;
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // 위에 대체 코드
        // let q = path.find("?");
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]); //some에서 ? 뒤에 문자열 자르기
        //     path = &path[..i]; //물음표까지 패스로 변경
        // }

        // 다시 위에 대체 코드 => some에 언랩핑되는 i가 존재하면 실행
        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..])); //some에서 ? 뒤에 문자열 자르기, &를 삭제했는데 여기서 더 이상 사용이 안되니까 필요없어서
            path = &path[..i]; //물음표까지 패스로 변경
        }

        Ok(Self {
            path,
            method,
            query_string,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, item) in request.chars().enumerate() {
        if item == ' ' {
            return Some((&request[..index], &request[index + 1..]));
        }
        // item == ' ' || item == "\r"
    }
    None
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }
}

// 리턴의 수명이 일치해야 한다
// fn get_next_word<'a, 'b>(request: &'a str, b: &'a str) -> Option<&'a str, &'a str> {
//     for (index, item) in request.chars().enumerate() {
//         if item == ' ' {
//             return Some((&requestp[..i], &request[i + 1..]));
//         }
//     }
//     None
// }

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
