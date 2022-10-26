use std::ops;

use intern::intern;
use intern::IString;

pub struct Label {
    text: IString,
}

impl<'a> From<&'a str> for Label {
    fn from(source: &'a str) -> Self {
        Self {
            text: intern(source),
        }
    }
}

impl ops::Deref for Label {
    type Target = str;

    fn deref(&self) -> &str {
        &*self.text
    }
}
