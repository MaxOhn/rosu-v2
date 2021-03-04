use crate::{
    model::Beatmap,
    request::{Pending, Query, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get a beatmap by its id.
pub struct GetBeatmap<'a> {
    fut: Option<Pending<'a, Beatmap>>,
    osu: &'a Osu,
    checksum: Option<String>,
    filename: Option<String>,
    map_id: Option<u32>,
}

impl<'a> GetBeatmap<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            checksum: None,
            filename: None,
            map_id: None,
        }
    }

    #[inline]
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum.replace(checksum.into());

        self
    }

    #[inline]
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename.replace(filename.into());

        self
    }

    #[inline]
    pub fn map_id(mut self, map_id: u32) -> Self {
        self.map_id.replace(map_id);

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let mut query = Query::new();

        if let Some(checksum) = self.checksum.take() {
            query.push("checksum", checksum);
        }

        if let Some(filename) = self.filename.take() {
            query.push("filename", filename);
        }

        if let Some(map_id) = self.map_id {
            query.push("id", map_id.to_string());
        }

        let req = Request::from((query, Route::GetBeatmap));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetBeatmap<'_>, Beatmap);
