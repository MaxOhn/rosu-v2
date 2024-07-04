use crate::{
    model::comments::{CommentBundle, CommentSort},
    request::{serialize::maybe_comment_sort, Pending, Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get a list of comments and their replies up to two levels deep
/// in form of a [`CommentBundle`](crate::model::comments::CommentBundle).
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetComments<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, CommentBundle>>,
    #[serde(skip)]
    osu: &'a Osu,
    commentable_type: Option<String>,
    commentable_id: Option<u32>,
    parent_id: Option<u32>,
    #[serde(serialize_with = "maybe_comment_sort")]
    sort: Option<CommentSort>,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetComments<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu) -> Self {
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
    pub const fn sort_new(mut self) -> Self {
        self.sort = Some(CommentSort::New);

        self
    }

    /// Sort the result by vote count
    #[inline]
    pub const fn sort_top(mut self) -> Self {
        self.sort = Some(CommentSort::Top);

        self
    }

    /// Sort the result by date, oldest first
    #[inline]
    pub const fn sort_old(mut self) -> Self {
        self.sort = Some(CommentSort::Old);

        self
    }

    /// Limit to comments which are reply to the specified id. Specify 0 to get top level comments
    #[inline]
    pub const fn parent(mut self, parent_id: u32) -> Self {
        self.parent_id = Some(parent_id);

        self
    }

    /// The id of the resource to get comments for
    #[inline]
    pub const fn commentable_id(mut self, commentable_id: u32) -> Self {
        self.commentable_id = Some(commentable_id);

        self
    }

    /// The type of resource to get comments for
    #[inline]
    pub fn commentable_type(mut self, commentable_type: impl Into<String>) -> Self {
        self.commentable_type = Some(commentable_type.into());

        self
    }

    #[inline]
    pub(crate) const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, CommentBundle> {
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetComments, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetComments => CommentBundle);
