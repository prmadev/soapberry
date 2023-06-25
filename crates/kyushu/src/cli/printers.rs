//! printers are the projectors of events.
//! they are the things that are used to interpret the data

use owo_colors::OwoColorize;
use redmaple::{event_group::EventGroup, RedMaple, RedMapleProjector};
use whirlybird::journey::{Event, EventWrapper, ValidMapleID};

///  `EntryPrinter` struct
pub struct EntryPrinter<'a> {
    show_time: bool,
    show_id: bool,
    time_format: Vec<time::format_description::FormatItem<'a>>,
}

impl<'a> EntryPrinter<'a> {
    /// creates a new `EntryPrinter`
    #[must_use]
    pub fn new(
        show_time: bool,
        show_id: bool,
        time_format: Vec<time::format_description::FormatItem<'a>>,
    ) -> Self {
        Self {
            show_time,
            show_id,
            time_format,
        }
    }
}

impl<'a> RedMapleProjector for EntryPrinter<'a> {
    type EventType = EventWrapper;

    fn projector(&self, data: &RedMaple<Self::EventType>) -> String {
        let id = if self.show_id {
            match ValidMapleID::try_from(data) {
                Ok(o) => o.inner().inner().to_string(),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        };

        let date_string = if self.show_time {
            data.events()
                .first()
                .map_or(String::from("____-__-__ __:__:__"), |x| {
                    // let a: DateTime<Local> = x.time().to_owned().into();
                    let time_offset =
                        time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);

                    x.time()
                        .to_offset(time_offset)
                        .format(&self.time_format)
                        .unwrap_or_default()
                })
        } else {
            String::new()
        };

        let body = data
            .events()
            .first()
            .and_then(|x| match x.data() {
                Event::MapleCreated(e) => Some(e.body().inner().clone()),
                Event::MapleBodyUpdated(_, _) => None,
            })
            .unwrap_or_default();

        format!(
            "{} {} {}\n{}\n",
            date_string.bold().reversed(),
            "=>".bold(),
            body,
            id.dimmed().italic(),
        )
    }
}
