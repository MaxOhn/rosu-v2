use crate::{
    model::{BeatmapScores, GameMode, GameMods},
    request::{Pending, Query, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get top scores of a beatmap by its id.
pub struct GetBeatmapScores<'a> {
    fut: Option<Pending<'a, BeatmapScores>>,
    osu: &'a Osu,
    map_id: u32,
    score_type: Option<&'static str>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
}

impl<'a> GetBeatmapScores<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            score_type: None, // TODO
            mode: None,
            mods: None,
        }
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods.replace(mods);

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", mode.to_string());
        }

        if let Some(_mods) = self.mods {
            // TODO
        }

        if let Some(_score_type) = self.score_type {
            // TODO
        }

        let req = Request::from((
            query,
            Route::GetBeatmapScores {
                map_id: self.map_id,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetBeatmapScores<'_>, BeatmapScores);
