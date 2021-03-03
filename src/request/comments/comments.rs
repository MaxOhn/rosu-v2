use crate::{
    model::{CommentBundle, CommentBundleCursor, CommentSort},
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

// TODO: Test
/// Get a list of comments and their replies up to two levels deep
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

    fn start(&mut self) -> OsuResult<()> {
        let mut form = Form::new();

        if let Some(sort) = self.sort {
            form = form.text("sort", sort.to_string());
        }

        if let Some(parent) = self.parent_id {
            form = form.text("parent_id", parent.to_string());
        }

        if let Some(commentable) = self.commentable_id {
            form = form.text("commentable_id", commentable.to_string());
        }

        if let Some(commentable) = self.commentable_type.take() {
            form = form.text("commentable_type", commentable);
        }

        let req = Request::from((
            form,
            Route::GetComments {
                cursor: self.cursor.take(),
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetComments<'_>, CommentBundle);
