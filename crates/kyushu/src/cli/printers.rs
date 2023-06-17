use chrono::{DateTime, Local};
use owo_colors::OwoColorize;
use redmaple::{event_group::EventGroup, id::IDGiver, RedMaple, RedMapleProjector};
use whirlybird::journey::{JournalEventWrapper, JourneyEvent};

///  EntryPrinter struct
pub struct EntryPrinter {
    show_time: bool,
    show_id: bool,
    time_format: String,
}

impl EntryPrinter {
    pub fn new(show_time: bool, show_id: bool, time_format: String) -> Self {
        Self {
            show_time,
            show_id,
            time_format,
        }
    }
}

impl RedMapleProjector for EntryPrinter {
    type EventType = JournalEventWrapper;

    fn projector(&self, data: &RedMaple<Self::EventType>) -> String {
        let id = match self.show_id {
            true => data.id().inner().inner().to_string(),
            false => "".to_string(),
        };

        let date = match self.show_time {
            true => data
                .events()
                .first()
                .map(|x| {
                    let a: DateTime<Local> = x.time().to_owned().into();
                    a.format(&self.time_format).to_string()
                })
                .unwrap_or(String::from("____-__-__ __:__:__")),
            false => "".to_string(),
        };

        let body = data
            .events()
            .first()
            .map(|x| match x.data() {
                JourneyEvent::MapleCreated(e) => Some(e.body().inner().to_owned()),
                _ => None,
            })
            .flatten()
            .unwrap_or_else(|| "".to_string());

        format!(
            "{} {} {}\n{}\n",
            date.bold().reversed(),
            "=>".bold(),
            body,
            id.dimmed().italic(),
        )
    }
}
