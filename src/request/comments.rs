use crate::{
    model::comments::{CommentBundle, CommentBundleCursor, CommentSort},
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
    cursor: Option<CommentBundleCursor>,
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

    #[inline]
    pub fn sort_new(mut self) -> Self {
        self.sort.replace(CommentSort::New);

        self
    }

    #[inline]
    pub fn sort_top(mut self) -> Self {
        self.sort.replace(CommentSort::Top);

        self
    }

    #[inline]
    pub fn sort_old(mut self) -> Self {
        self.sort.replace(CommentSort::Old);

        self
    }

    #[inline]
    pub fn parent(mut self, parent_id: u32) -> Self {
        self.parent_id.replace(parent_id);

        self
    }

    #[inline]
    pub fn commentable_id(mut self, commentable_id: u32) -> Self {
        self.commentable_id.replace(commentable_id);

        self
    }

    #[inline]
    pub fn commentable_type(mut self, commentable_type: impl Into<String>) -> Self {
        self.commentable_type.replace(commentable_type.into());

        self
    }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: CommentBundleCursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, CommentBundle> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.comments.inc();

        let mut query = Query::new();

        if let Some(sort) = self.sort {
            query.push("sort", sort.to_string());
        }

        if let Some(parent) = self.parent_id {
            query.push("parent_id", parent.to_string());
        }

        if let Some(commentable) = self.commentable_id {
            query.push("commentable_id", commentable.to_string());
        }

        if let Some(commentable) = self.commentable_type.take() {
            query.push("commentable_type", commentable);
        }

        if let Some(cursor) = self.cursor.take() {
            query.push("cursor[id]", cursor.id.to_string());
            query.push("cursor[created_at]", cursor.created_at.to_string()); // TODO: Test
        }

        let req = Request::from((query, Route::GetComments));

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetComments => CommentBundle);
