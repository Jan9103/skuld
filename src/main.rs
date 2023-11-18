use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use std::thread::sleep;
use std::io;
use std::process::Command;
use std::ffi::OsStr;

enum TimeConfig {
    Any,
    Exact(u32), // u8 would work, but chrono uses u32 and conversions are annoying
    DividableBy(u32),
}

impl TimeConfig {
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.is_empty() {
            return Err(String::from(
                "Empty time value config (maybe a accidental double-space?)",
            ));
        }
        if text == "*" {
            return Ok(Self::Any);
        }
        if &text[0..1] == "/" {
            return Ok(Self::DividableBy(text[1..].parse::<u32>().map_err(
                |_| format!("Failed to parse number (division entry): {}", text),
            )?));
        }
        return Ok(Self::Exact(
            text.parse::<u32>()
                .map_err(|_| format!("Failed to parse number: {}", text))?,
        ));
    }

    pub fn matches(&self, time: &u32) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(i) => time == i,
            Self::DividableBy(i) => time % i == 0,
        }
    }
}

struct Entry {
    day_of_month: TimeConfig,
    day_of_week: TimeConfig,
    hour: TimeConfig,
    minute: TimeConfig,
    command: String,
}

impl Entry {
    pub fn parse(line: &String) -> Result<Self, String> {
        let args: Vec<&str> = line.splitn(5, " ").collect();
        if args.len() != 5 {
            return Err(format!("Wrong syntax (only {} args)", args.len()));
        }
        Ok(Self {
            day_of_month: TimeConfig::parse(args[0])?,
            day_of_week: TimeConfig::parse(args[1])?,
            hour: TimeConfig::parse(args[2])?,
            minute: TimeConfig::parse(args[3])?,
            command: args[4].to_string(),
        })
    }

    pub fn run(&self) {
        println!("[SKULD] Running: {}", self.command);
        match Command::new("sh")
            .arg("-c")
            .arg(OsStr::new(self.command.as_str()))
            .spawn() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("[SKULD] ERROR: Failed to spawn `sh -c {}`: {:?}", self.command, err.kind());
            }
        }
    }

    pub fn should_run(&self, time: &DateTime<Utc>) -> bool {
        if !self.day_of_month.matches(&time.day()) {
            return false;
        }
        if !self
            .day_of_week
            .matches(&time.weekday().number_from_monday())
        {
            return false;
        }
        if !self.hour.matches(&time.hour()) {
            return false;
        }
        if !self.minute.matches(&time.minute()) {
            return false;
        }
        return true;
    }
}

fn main() {
    let mut all_entries: Vec<Entry> = Vec::new();
    for line in io::stdin().lines() {
        match line {
            Ok(line) => match Entry::parse(&line) {
                Ok(e) => all_entries.push(e),
                Err(e) => {
                    eprintln!("[SKULD] ERROR: failed to parse line ({}) for: {}", line, e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("[SKULD] ERROR: failed to read stdin lines: {}", e);
                return;
            }
        }
    }
    let mut now: DateTime<Utc> = Utc::now();
    let d1min: Duration = Duration::minutes(1);
    let mut next: DateTime<Utc> = now + d1min;
    loop {
        now = next;
        next = now + d1min;
        let actual_now: DateTime<Utc> = Utc::now();
        let sleep_duration = next - actual_now;
        if sleep_duration.num_seconds() > 0 {
            sleep(
                sleep_duration
                    .to_std()
                    .expect("[SKULD] PANIC: Failed to convert chrono::Duration to std Duration."),
            );
        }

        for entry in &all_entries {
            if entry.should_run(&now) {
                entry.run();
            }
        }
    }
}
