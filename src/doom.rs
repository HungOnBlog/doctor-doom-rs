use std::env;
use std::fmt::{Debug, Formatter, Result};

/// DoomOptions is a struct that contains all the options for the program.
///
/// # Examples
///
/// ```rust
/// use doctor_doom_rs::doom;
/// let doom_options = doom::DoomOptions::new_and_assign_from_env();
/// println!("{:?}", doom_options);
///
/// ```
///
pub struct DoomRule {
    /// age is the age of the file to be deleted.
    /// It must be in the format of a number followed by a unit.
    /// The unit can be one of the following:
    /// - s: seconds
    /// - m: minutes
    /// - h: hours
    /// - d: days
    /// - w: weeks
    /// - M: months
    pub age: String,

    /// size is the size of the file to be deleted.
    /// It must be in the format of a number followed by a unit.
    /// The unit can be one of the following:
    /// - B: bytes
    /// - K: kilobytes
    /// - M: megabytes
    /// - G: gigabytes
    /// - T: terabytes
    /// - P: petabytes
    pub size: String,

    /// name is the name of the file to be deleted.
    /// It must be in the format of a glob.
    /// The glob can be one of the following:
    /// - *: matches everything
    /// - "": matches nothing
    /// - regex: matches the regex
    pub name: String,
}

/// DoomOptions is a struct that contains all the options for the program.
///
/// # Examples
///
/// ```rust
/// use doctor_doom_rs::doom;
/// let doom_options = doom::DoomOptions::new_and_assign_from_env();
/// println!("{:?}", doom_options);
///
/// ```
///
pub struct DoomOptions {
    /// doom_path is the path to the directory that will be cleaned.
    pub doom_path: String,

    /// doom_export is the path to the directory that will be exported logs.
    pub doom_export: String,

    /// circle is the cron expression that will be used to schedule the program.
    pub circle: String,

    /// rule is the rule that will be used to clean the directory.
    pub rule: DoomRule,
}

impl DoomOptions {
    pub fn new() -> DoomOptions {
        DoomOptions {
            doom_path: String::from("./"),
            doom_export: String::from("/var/log"),
            circle: String::from("0 0 * * 0"),
            rule: DoomRule {
                age: String::from("7d"),
                size: String::from("100M"),
                name: String::from("*"),
            },
        }
    }

    pub fn assign(
        &mut self,
        doom_path: String,
        doom_export: String,
        circle: String,
        rule: DoomRule,
    ) {
        self.doom_path = doom_path;
        self.doom_export = doom_export;
        self.circle = circle;
        self.rule = rule;
    }

    pub fn assign_from_env(&mut self) {
        self.doom_path = env::var("DOOM_PATH").unwrap_or(String::from("./"));
        self.doom_export = env::var("DOOM_EXPORT").unwrap_or(String::from("/var/log"));
        self.circle = env::var("DOOM_CIRCLE").unwrap_or(String::from("0 0 * * 0"));
        self.rule = DoomRule {
            age: env::var("RULE_AGE").unwrap_or(String::from("7d")),
            size: env::var("RULE_SIZE").unwrap_or(String::from("100M")),
            name: env::var("RULE_NAME").unwrap_or(String::from("*")),
        };
    }

    pub fn new_and_assign_from_env() -> DoomOptions {
        let mut doom_options = DoomOptions::new();
        doom_options.assign_from_env();
        doom_options
    }
}

impl Debug for DoomOptions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "DoomOptions {{ doom_path: {}, doom_export: {}, circle: {}, rule: {:?} }}",
            self.doom_path, self.doom_export, self.circle, self.rule
        )
    }
}

impl Debug for DoomRule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "DoomRule {{ age: {}, size: {}, name: {} }}",
            self.age, self.size, self.name
        )
    }
}
