pub fn byte_length(input: &str) -> Option<usize> {
    match input.chars().next()? {
        // SimpleString
        '+' => {
            let end = input[..].find("\r\n")?;
            Some(end + 2)
        }

        // Error
        '-' => {
            let end = input[..].find("\r\n")?;
            Some(end + 2)
        }

        // Integer
        ':' => {
            let end = input[..].find("\r\n")?;
            Some(end + 2)
        }

        // BulkString
        '$' => {
            let start = input.find("\r\n")? + 2;

            let byte_length = input[1..start - 2].parse::<i32>().ok()?;

            if byte_length == -1 {
                return Some(start);
            }

            if input[start..].find("\r\n").is_some() {
                Some(start + byte_length as usize + 2)
            } else {
                None
            }
        }

        // Array
        '*' => {
            let start = input.find("\r\n")? + 2;

            let elements = input[1..start - 2].parse::<i32>().ok()?;

            if elements == -1 {
                return Some(start);
            }

            let elements = elements as usize;

            let mut res = 0;

            for _ in 0..elements {
                res += byte_length(&input[start + res..])?;
            }

            Some(start + res)
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_lengths() {
        let simple_string = "+asdf\r\n";
        assert_eq!(7, byte_length(simple_string).unwrap());

        let error = "-asdf\r\n";
        assert_eq!(7, byte_length(error).unwrap());

        let integer = ":1234\r\n";
        assert_eq!(7, byte_length(integer).unwrap());

        let bulk_string = "$4\r\nasdf\r\n";
        assert_eq!(10, byte_length(bulk_string).unwrap());

        let array = "*2\r\n$4\r\nasdf\r\n$4\r\nasdf\r\n";
        assert_eq!(24, byte_length(array).unwrap());
    }
}
