mod error;
mod parser;

pub use error::{Error, Result};
pub use parser::byte_length;

#[derive(Debug, PartialEq)]
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Option<Vec<RespType>>),
}

impl RespType {
    pub fn from_str(input: &str) -> Result<Self> {
        if input.is_empty() {
            return Err(Error::UnexpectedEOF);
        }

        let mut parser = Parser { s: input };

        parser.parse()
    }
}

struct Parser<'a> {
    s: &'a str,
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Result<RespType> {
        match self.s.chars().next() {
            // SimpleString
            Some('+') => {
                let end = 1 + self.s[1..].find("\r\n").ok_or(Error::UnexpectedEOF)?;

                let res = RespType::SimpleString(self.s[1..end].to_string());
                self.s = &self.s[end + 2..];
                Ok(res)
            }

            // Error
            Some('-') => {
                let end = 1 + self.s[1..].find("\r\n").ok_or(Error::UnexpectedEOF)?;
                let res = RespType::Error(self.s[1..end].to_string());
                self.s = &self.s[end + 2..];
                Ok(res)
            }

            // Integer
            Some(':') => {
                let end = 1 + self.s[1..].find("\r\n").ok_or(Error::UnexpectedEOF)?;

                let as_str = &self.s[1..end];

                let res = RespType::Integer(as_str.parse::<i64>().or(Err(Error::Other))?);
                self.s = &self.s[end + 2..];
                Ok(res)
            }

            // BulkString
            Some('$') => {
                let start = self.s.find("\r\n").ok_or(Error::UnexpectedEOF)? + 2;

                let byte_length = self.s[1..start - 2].parse::<i32>().or(Err(Error::Other))?;

                if byte_length == -1 {
                    return Ok(RespType::BulkString(None));
                }

                let byte_length = byte_length as usize;

                let as_str = &self.s[start..start + byte_length];
                let res = RespType::BulkString(Some(as_str.to_string()));
                self.s = &self.s[start + byte_length + 2..];
                Ok(res)
            }

            // Array
            Some('*') => {
                let start = self.s.find("\r\n").ok_or(Error::UnexpectedEOF)? + 2;

                let elements = self.s[1..start - 2].parse::<i32>().or(Err(Error::Other))?;

                if elements == -1 {
                    return Ok(RespType::Array(None));
                }

                let elements = elements as usize;

                let mut res = Vec::new();

                self.s = &self.s[start..];

                for _ in 0..elements {
                    let item = self.parse()?;
                    res.push(item);
                }

                Ok(RespType::Array(Some(res)))
            }

            _ => Err(Error::Other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_simple_string() {
        let input = "+simple_string\r\n";
        let result = RespType::from_str(input).unwrap();
        assert_eq!(RespType::SimpleString("simple_string".to_string()), result);
    }

    #[test]
    fn deserialize_error() {
        let input = "-ERR\r\n";
        let result = RespType::from_str(input).unwrap();
        assert_eq!(RespType::Error("ERR".to_string()), result);
    }

    #[test]
    fn deserialize_integer() {
        let input = ":1234\r\n";
        let result = RespType::from_str(input).unwrap();
        assert_eq!(RespType::Integer(1234), result);
    }

    #[test]
    fn deserialize_bulk_string() {
        let input = "$5\r\nhello\r\n";
        let result = RespType::from_str(input).unwrap();
        assert_eq!(RespType::BulkString(Some("hello".to_string())), result);
    }

    #[test]
    fn deserialize_array() {
        let input = "*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$5\r\nhello\r\n";
        let result = RespType::from_str(input).unwrap();

        let expected = RespType::Array(Some(vec![
            RespType::Integer(1),
            RespType::Integer(2),
            RespType::Integer(3),
            RespType::Integer(4),
            RespType::BulkString(Some("hello".to_string())),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn deserialize_null_array() {
        let input = "*-1\r\n";
        let result = RespType::from_str(input).unwrap();
        assert_eq!(RespType::Array(None), result);
    }

    #[test]
    fn deserialize_nested_array() {
        let input = "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Hello\r\n-World\r\n";
        let result = RespType::from_str(input).unwrap();

        let expected = RespType::Array(Some(vec![
            RespType::Array(Some(vec![
                RespType::Integer(1),
                RespType::Integer(2),
                RespType::Integer(3),
            ])),
            RespType::Array(Some(vec![
                RespType::SimpleString("Hello".to_string()),
                RespType::Error("World".to_string()),
            ])),
        ]));
        assert_eq!(expected, result);
    }
}
