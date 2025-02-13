// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug)]
enum ParsingError {
    #[error("target cannot be used to create valid status")]
    InvalidStatusTarget,
}

impl TryFrom<String> for Status {
    type Error = ParsingError;

    fn try_from(target: String) -> Result<Self, Self::Error> {
        match target.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParsingError::InvalidStatusTarget),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParsingError;

    fn try_from(target: &str) -> Result<Self, Self::Error> {
        // NOTE:Is this implementation equivalent to `TryFrom<String>` because
        // `String` implements `Into<String, str>`?
        // match target.to_lowercase().as_str() {
        //     "todo" => Ok(Status::ToDo),
        //     "inprogress" => Ok(Status::InProgress),
        //     "done" => Ok(Status::Done),
        //     _ => Err(ParsingError::InvalidStatusTarget),
        // }

        // NOTE:Yes! We can take advantage of this fact and simplify
        // this implementation.
        target.to_lowercase().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
