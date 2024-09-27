pub enum KeyValueTyping<'s> {
    Key(&'s str),
    Value(&'s str),
}

pub enum StartString {
    Key(usize),
    Value(usize),
}

pub enum ParsingState<'s> {
    None,
    Key(&'s str),
    Semicolon,
    ValueOpenQuote,
    Value(&'s str),
    ValueCloseQuote,
    Separator,
}

impl<'s> KeyValueTyping<'s> {
    pub fn parse(src: &'s str) -> Option<Self> {
        None
        /*
        let mut result = None;

        let mut start_string: Option<StartString> = None;

        let i = 0;
        for c in src.chars() {
            match result {
                ParsingState::None => {
                    if c != ' ' {
                        start_string = Some(StartString::Key(i));
                    }
                }
                ParsingState::Key(_) => if c == ':' {},
                ParsingState::Semicolon => if c == '\'' {},
                ParsingState::ValueOpenQuote => if is_open_close_string(c) {},
                ParsingState::Value(_) => if c == '\'' {},

                ParsingState::ValueCloseQuote => if is_open_close_string(c) {},
                ParsingState::Separator => {
                    if c == ' ' {
                        result = None;
                    }
                }
            }

            i += 1;
        }

        result
         */
    }
}

fn is_open_close_string(c: char) -> bool {
    if c == '\'' {
        return true;
    }

    if c == '"' {
        return true;
    }

    false
}
