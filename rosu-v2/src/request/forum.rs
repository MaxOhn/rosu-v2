use crate::{
    model::{forum_::ForumPosts, Cursor},
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

/// Get a [`ForumPosts`](crate::model::forum::ForumPosts) struct for a forum topic
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetForumPosts<'a> {
    fut: Option<Pending<'a, ForumPosts>>,
    osu: &'a Osu,
    topic_id: u64,
    sort: Option<&'static str>,
    limit: Option<usize>,
    start: Option<u64>,
    end: Option<u64>,
    cursor: Option<Cursor>,
}

impl<'a> GetForumPosts<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, topic_id: u64) -> Self {
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
        self.limit.replace(limit.min(50));

        self
    }

    /// Sort by ascending post ids. This is the default.
    #[inline]
    pub fn sort_ascending(mut self) -> Self {
        self.sort.replace("id_asc");

        self
    }

    /// Sort by descending post ids
    #[inline]
    pub fn sort_descending(mut self) -> Self {
        self.sort.replace("id_desc");

        self
    }

    /// First post id to be returned if sorted ascendingly.
    /// Parameter is ignored if `cursor` is specified.
    #[inline]
    pub fn start_id(mut self, start: u64) -> Self {
        self.start.replace(start);

        self
    }

    /// First post id to be returned if sorted descendingly.
    /// Parameter is ignored if `cursor` is specified.
    #[inline]
    pub fn end_id(mut self, end: u64) -> Self {
        self.end.replace(end);

        self
    }

    /// Specify a page by providing a cursor
    #[inline]
    pub fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, ForumPosts> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.forum_posts.inc();

        let mut query = Query::new();

        if let Some(sort) = self.sort {
            query.push("sort", sort);
        }

        if let Some(limit) = self.limit {
            query.push("limit", limit);
        }

        if let Some(id) = self.start {
            query.push("start", id);
        }

        if let Some(id) = self.end {
            query.push("end", id);
        }

        if let Some(cursor) = self.cursor.take() {
            cursor.push_to_query(&mut query);
        }

        let route = Route::GetForumPosts {
            topic_id: self.topic_id,
        };

        let req = Request::with_query(route, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetForumPosts => ForumPosts);
