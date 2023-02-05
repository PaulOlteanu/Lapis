#[derive(Debug, Clone)]
pub struct SimpleString(pub String);

#[derive(Debug, Clone)]
pub struct Error(pub String);

#[derive(Debug, Clone)]
pub struct Integer(pub i64);

#[derive(Debug, Clone)]
pub struct BulkString(pub String);

#[derive(Debug, Clone)]
pub struct Array(pub Vec<Type>);

#[derive(Debug, Clone)]
pub enum Type {
    SimpleString(SimpleString),
    Error(Error),
    Integer(Integer),
    BulkString(BulkString),
    Array(Array),
    Null,
}

impl Type {
    pub fn parse(input: &str) -> Option<(Self, usize)> {
        if input.is_empty() {
            return None;
        }

        Self::parse_from(input, 0)
    }

    fn parse_from(input: &str, base_start: usize) -> Option<(Self, usize)> {
        let input = &input[base_start..];
        match input.chars().next()? {
            // SimpleString
            '+' => {
                println!("Parsing simple string");

                let end = input[1..].find("\r\n")?;
                Some((
                    Self::SimpleString(SimpleString(input[1..end].to_string())),
                    base_start + end + 2,
                ))
            }

            // Error
            '-' => {
                println!("Parsing error");

                let end = input[1..].find("\r\n")?;
                Some((
                    Self::Error(Error(input[1..end].to_string())),
                    base_start + end + 2,
                ))
            }

            // Integer
            ':' => {
                println!("Parsing integer");

                let end = input[1..].find("\r\n")?;
                let as_str = &input[1..end];
                Some((
                    Self::Integer(Integer(as_str.parse::<i64>().ok()?)),
                    base_start + end + 2,
                ))
            }

            // BulkString
            '$' => {
                println!("Parsing bulk string");

                let start = input.find("\r\n")? + 2;

                let byte_length = input[1..start - 2].parse::<i32>().ok()?;

                if byte_length == -1 {
                    // TODO: The right number
                    return Some((Self::Null, 0));
                }

                let byte_length = byte_length as usize;

                let as_str = &input[start..start + byte_length];
                Some((
                    Self::BulkString(BulkString(as_str.to_string())),
                    base_start + start + byte_length + 2,
                ))
            }

            // Array
            '*' => {
                println!("Parsing array");

                let start = input.find("\r\n")? + 2;

                let elements = input[1..start - 2].parse::<i32>().ok()?;

                if elements == -1 {
                    // TODO: The right number
                    return Some((Self::Null, 0));
                }

                let elements = elements as usize;

                let mut idx = start;
                let mut res = Vec::new();

                for _ in 0..elements {
                    let (item, new_idx) = Self::parse_from(input, idx)?;
                    res.push(item);
                    idx = new_idx;
                }

                Some((Self::Array(Array(res)), base_start + idx))
            }

            _ => None,
        }
    }
}
