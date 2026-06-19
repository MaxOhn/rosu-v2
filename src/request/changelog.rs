use crate::{
    prelude::{Build, ChangelogListing},
    request::{Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChangelogBuild<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    stream: String,
    build: String,
}

impl<'a> GetChangelogBuild<'a> {
    pub(crate) const fn new(osu: &'a Osu, stream: String, build: String) -> Self {
        Self { osu, stream, build }
    }
}

into_future! {
    |self: GetChangelogBuild<'_>| -> Build {
        let route = Route::GetChangelogBuild {
            stream: self.stream,
            build: self.build,
        };

        Request::new(route)
    }
}

#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChangelogListing<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    from: Option<&'a str>,
    to: Option<&'a str>,
    max_id: Option<&'a str>,
    stream: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    message_formats: Vec<&'a str>,
}

impl<'a> GetChangelogListing<'a> {
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            from: None,
            to: None,
            max_id: None,
            // There are only two supported formats, it should be fine
            message_formats: Vec::new(),
            stream: None,
        }
    }

    /// Specify minimum build version
    #[inline]
    pub fn from(mut self, from: &'a str) -> Self {
        self.from = Some(from);

        self
    }

    /// Specify maximum build version
    pub fn to(mut self, to: &'a str) -> Self {
        self.to = Some(to);

        self
    }

    /// Specify the release stream
    pub fn stream(mut self, stream: &'a str) -> Self {
        self.stream = Some(stream);

        self
    }

    pub fn message_formats<I>(mut self, message_formats: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.message_formats = message_formats.into_iter().collect();

        self
    }
}

into_future! {
    |self: GetChangelogListing<'_>| -> ChangelogListing {
        Request::with_query(Route::GetChangelogListing, Query::encode(&self))
    }
}
