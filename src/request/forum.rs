use crate::{
    model::forum::{ForumPosts, ForumPostsCursor},
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
    limit: Option<u32>,
    start: Option<u64>,
    end: Option<u64>,
    cursor: Option<ForumPostsCursor>,
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
    pub fn limit(mut self, limit: u32) -> Self {
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

    #[inline]
    pub fn cursor(mut self, cursor: ForumPostsCursor) -> Self {
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
            query.push("limit", limit.to_string());
        }

        if let Some(id) = self.start {
            query.push("start", id.to_string());
        }

        if let Some(id) = self.end {
            query.push("end", id.to_string());
        }

        if let Some(cursor) = self.cursor.take() {
            query.push("cursor[id]", cursor.post_id.to_string());
        }

        let req = Request::from((
            query,
            Route::GetForumPosts {
                topic_id: self.topic_id,
            },
        ));

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetForumPosts<'_> => ForumPosts);
