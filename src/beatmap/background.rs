/// A background of [`Beatmap`](crate::beatmap::Background)
#[derive(Debug, Default, Clone)]
pub struct Background {
    /// Start timestamp of the background
    pub start_time: f64,
    /// Filename of the background
    pub filename: String,
    /// x offset of the background
    pub x_offset: i32,
    /// y offset of the background
    pub y_offset: i32 
}
