extern crate csv;

use std::error::Error;
use std::path::{Path, PathBuf};

/// A data error or warning message that should be reported.
pub enum Message {
    Error(String),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
pub struct Report {
    pub path: PathBuf,
    pub messages: Vec<Message>,
}

impl Report {
    /// Creates a new Report.
    pub fn new(path: PathBuf) -> Self {
        Report { path, messages: Vec::new() }
    }

    /// Reports an error, which causes checks to fail.
    pub fn error(&mut self, message: impl ToString) {
        self.messages.push(Message::Error(message.to_string()));
    }

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, message: impl ToString) {
        self.messages.push(Message::Warning(message.to_string()));
    }
}

pub fn check_entries(entries_csv: PathBuf) -> Result<Report, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(entries_csv);

    // The entries.csv file must exist to continue on.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(report);
    }

    let mut rdr = csv::Reader::from_path(&report.path)?;
    for result in rdr.records() {
        result?;
    }

    Ok(report)
}

/// Checks a directory with meet data.
pub fn check(meetdir: &Path) -> Result<Vec<Report>, Box<Error>> {
    let mut acc = Vec::new();

    let report = check_entries(meetdir.join("entries.csv"))?;
    if !report.messages.is_empty() {
        acc.push(report);
    }

    Ok(acc)
}