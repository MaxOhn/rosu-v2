use crate::{
    model::{
        comments::{CommentBundle, CommentSort},
        Cursor,
    },
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

/// Get a list of comments and their replies up to two levels deep
/// in form of a [`CommentBundle`](crate::model::comments::CommentBundle).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetComments<'a> {
    fut: Option<Pending<'a, CommentBundle>>,
    osu: &'a Osu,
    commentable_type: Option<String>,
    commentable_id: Option<u32>,
    parent_id: Option<u32>,
    sort: Option<CommentSort>,
    cursor: Option<Cursor>,
}

impl<'a> GetComments<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            commentable_type: None,
            commentable_id: None,
            parent_id: None,
            sort: None,
            cursor: None,
        }
    }

    /// Sort the result by date, newest first
    #[inline]
    pub fn sort_new(mut self) -> Self {
        self.sort.replace(CommentSort::New);

        self
    }

    /// Sort the result by vote count
    #[inline]
    pub fn sort_top(mut self) -> Self {
        self.sort.replace(CommentSort::Top);

        self
    }

    /// Sort the result by date, oldest first
    #[inline]
    pub fn sort_old(mut self) -> Self {
        self.sort.replace(CommentSort::Old);

        self
    }

    /// Limit to comments which are reply to the specified id. Specify 0 to get top level comments
    #[inline]
    pub fn parent(mut self, parent_id: u32) -> Self {
        self.parent_id.replace(parent_id);

        self
    }

    /// The id of the resource to get comments for
    #[inline]
    pub fn commentable_id(mut self, commentable_id: u32) -> Self {
        self.commentable_id.replace(commentable_id);

        self
    }

    /// The type of resource to get comments for
    #[inline]
    pub fn commentable_type(mut self, commentable_type: impl Into<String>) -> Self {
        self.commentable_type.replace(commentable_type.into());

        self
    }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, CommentBundle> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.comments.inc();

        let mut query = Query::new();

        if let Some(sort) = self.sort {
            query.push("sort", &sort.to_string());
        }

        if let Some(parent) = self.parent_id {
            query.push("parent_id", &parent);
        }

        if let Some(commentable) = self.commentable_id {
            query.push("commentable_id", &commentable);
        }

        if let Some(commentable) = self.commentable_type.take() {
            query.push("commentable_type", &commentable);
        }

        if let Some(cursor) = self.cursor.take() {
            cursor.push_to_query(&mut query);
        }

        let req = Request::with_query(Route::GetComments, query);

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetComments => CommentBundle);
