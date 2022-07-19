use super::GameMod;

impl GameMod {
    /// The clock rate of the [`GameMod`]
    ///
    /// Returns `None` if there is no single clock rate.
    pub const fn clock_rate(&self) -> Option<f32> {
        // TODO: replace with `Option::unwrap_or` when it's const stable
        const fn unwrap_or(opt: Option<f32>, default: f32) -> f32 {
            match opt {
                Some(n) => n,
                None => default,
            }
        }

        match self {
            Self::DoubleTimeOsu(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::DoubleTimeTaiko(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::DoubleTimeCatch(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::DoubleTimeMania(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::NightcoreOsu(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::NightcoreTaiko(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::NightcoreCatch(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::NightcoreMania(m) => Some(unwrap_or(m.speed_change, 1.5)),
            Self::HalfTimeOsu(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::HalfTimeTaiko(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::HalfTimeCatch(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::HalfTimeMania(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::DaycoreOsu(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::DaycoreTaiko(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::DaycoreCatch(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::DaycoreMania(m) => Some(unwrap_or(m.speed_change, 0.75)),
            Self::WindUpOsu(_) => None,
            Self::WindUpTaiko(_) => None,
            Self::WindUpCatch(_) => None,
            Self::WindUpMania(_) => None,
            Self::WindDownOsu(_) => None,
            Self::WindDownTaiko(_) => None,
            Self::WindDownCatch(_) => None,
            Self::WindDownMania(_) => None,
            Self::AdaptiveSpeedOsu(_) => None,
            Self::AdaptiveSpeedTaiko(_) => None,
            Self::AdaptiveSpeedMania(_) => None,
            _ => Some(1.0),
        }
    }
}
