//! printers are the projectors of events.
//! they are the things that are used to interpret the data

use std::fmt::Display;

use owo_colors::OwoColorize;
use whirlybird::journey::{Body, ValidMapleID};

/// creates a new printer for each maple
pub struct MaplePrinter {
    id: ValidMapleID,
    body: Body,
    time_string: String,
}

impl MaplePrinter {
    /// Creates a new printer
    pub fn new_with_local_offset<'a>(
        id: ValidMapleID,
        body: Body,
        time_created: time::OffsetDateTime,
        time_format: &Vec<time::format_description::FormatItem<'a>>,
    ) -> Result<Self, NewPrinterError> {
        let time_offset = time::UtcOffset::current_local_offset()?; // may be we should get this from higher up?
        let time_string = time_created.to_offset(time_offset).format(time_format)?;

        Ok(Self {
            id,
            body,
            time_string,
        })
    }
}

/// errors that may arise while making a [`Title`]
#[derive(Debug, thiserror::Error)]
pub enum NewPrinterError {
    /// indeterminate local time
    #[error("local time could not gotten: {0}")]
    FailedToDetermineLocalTime(#[from] time::error::IndeterminateOffset),

    /// formatting issue
    #[error("got some formatting issue: {0}")]
    FailedToFormatTime(#[from] time::error::Format),
}

impl Display for MaplePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\n {} \n",
            self.time_string.bold().reversed(),
            "=>".bold(),
            self.body,
            self.id.dimmed().italic()
        )
    }
}
