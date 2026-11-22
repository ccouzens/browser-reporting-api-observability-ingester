use serde::Serialize;

/**
Browser name and version parsed from a [`super::UserAgent`].
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Browser<'a> {
    pub name: &'a str,
    pub version: Option<(u16, u16)>,
}
