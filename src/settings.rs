/// Application settings
#[derive(Clone)]
pub struct Settings {
    /// How much note sticks to just notes
    /// 0 means no autotune.
    pub autotune_strength: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            autotune_strength: 2,
        }
    }
}
