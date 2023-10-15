use crate::{
    model::{
        comments_::{CommentBundle, CommentSort},
        Cursor,
    },
    request::{
        serialize::{maybe_comment_sort, maybe_cursor},
        Pending, Query, Request,
    },
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
    #[serde(flatten, serialize_with = "maybe_cursor")]
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
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetComments, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetComments => CommentBundle);
