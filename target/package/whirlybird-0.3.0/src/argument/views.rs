//! `views` implement `ViewMode`s that are suitable for `Argument` implementation of `RedMaple`

use std::fmt::Display;

use redmaple::view_mode::ViewMode;

/// Holds the different view modes that the `RedMaple` could present
#[derive(Clone, Debug)]
#[must_use]
pub enum Views {
    /// means one big post up, and editions for that post + comments and replies
    Blog(BlogMode),
    /// conversation means a series of talks that two or more people could have, responding to
    /// each other
    Conversation,
    /// a series of links that reflect a replied conversations. reflecting and responding to each
    /// other.
    ResponseLinks,
    /// a curated list of links that hold the same theme
    CuratedList,
    /// a list of todo items representing a task progress
    TodoList,
}

// implementation of `Display` is neccessary for later implementation of `ViewMode` trait
impl Display for Views {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blog(m) => match m {
                    BlogMode::Text => "BlogText",
                    BlogMode::PhotoSlide => "BlogPhotoSlide",
                    BlogMode::Video => "BlogVideo",
                },
                Self::Conversation => "Conversation",
                Self::ResponseLinks => "ResponseLinks",
                Self::CuratedList => "CuratedList",
                Self::TodoList => "TodoList",
            }
        )
    }
}

impl ViewMode for Views {
    type Identifier = Self;

    fn get(&self) -> &Self::Identifier {
        self
    }
}

/// blogs could form different views, on could stress on the text while the other could present a
/// series of photos or a video
#[derive(Clone, Debug)]
pub enum BlogMode {
    /// Text is the traditional essay blogging form
    Text,
    /// PhotoSlide does not neccessarily means that the post should have an slider, rather, the
    /// focus is the photos taken.
    PhotoSlide,
    /// Video represent a video stream as the main post
    Video,
}

impl Display for BlogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text => "BlogMode(Text)",
                Self::PhotoSlide => "BlogMode(PhotoSlide)",
                Self::Video => "BlogMode(Video)",
            }
        )
    }
}
impl ViewMode for BlogMode {
    type Identifier = Self;

    fn get(&self) -> &Self::Identifier {
        self
    }
}
