use crate::{
    model::forum_::ForumPosts,
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get a [`ForumPosts`](crate::model::forum::ForumPosts) struct for a forum topic
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetForumPosts<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, ForumPosts>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    topic_id: u64,
    sort: Option<&'static str>,
    limit: Option<usize>,
    start: Option<u64>,
    end: Option<u64>,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetForumPosts<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, topic_id: u64) -> Self {
        Self {
            fut: None,
            osu,
            topic_id,
            sort: None,
            limit: None,
            start: None,
            end: None,
            cursor: None,
        }
    }

    /// Maximum number of posts to be returned (20 default, 50 at most)
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit.min(50));

        self
    }

    /// Sort by ascending post ids. This is the default.
    #[inline]
    pub const fn sort_ascending(mut self) -> Self {
        self.sort = Some("id_asc");

        self
    }

    /// Sort by descending post ids
    #[inline]
    pub const fn sort_descending(mut self) -> Self {
        self.sort = Some("id_desc");

        self
    }

    /// First post id to be returned if sorted ascendingly.
    /// Parameter is ignored if `cursor` is specified.
    #[inline]
    pub const fn start_id(mut self, start: u64) -> Self {
        self.start = Some(start);

        self
    }

    /// First post id to be returned if sorted descendingly.
    /// Parameter is ignored if `cursor` is specified.
    #[inline]
    pub const fn end_id(mut self, end: u64) -> Self {
        self.end = Some(end);

        self
    }

    /// Specify a page by providing a cursor
    #[inline]
    pub const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, ForumPosts> {
        let query = Query::encode(self);

        let route = Route::GetForumPosts {
            topic_id: self.topic_id,
        };

        let req = Request::with_query(route, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetForumPosts => ForumPosts);
