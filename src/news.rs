pub struct News {
    pub title: &'static str,
    pub version: &'static str,
}

// Edit these values to publish the single latest launcher announcement.
pub const LATEST_NEWS: News = News {
    title: "NanoMC Reworked",
    version: "1.1",
};
