use std::fs;
use serde_json::{Value};

struct NodeModule {
    name: String,
    resolved_path: String,
    version: Version,
}

fn resolve(package_name: &str, location: &str, root_package: Option<&str>) -> Option<NodeModule> {
    let root_package = match root_package {
        Some(rp) => rp,
        None => package_name,
    };

    match fs::read_to_string(format!("{}/{}/package.json", location, root_package)) {
        Ok(package) => {
            let package_json: Value = serde_json::from_str(&package).unwrap();
            
            
            let version = match &package_json["version"] {
                Value::String(v) => v,
                _ => return None,
            };

            Some(NodeModule {
                name: String::from("test"),
                version: Version::new(version),
                resolved_path: String::from("")
            })
        }
        Err(_) => None,
    }
}

/// Converts the specified string `a` to non-negative integer.
/// Returns -1 if the result is NaN.
fn parse_non_negative_int(a: &str) -> i32 {
    match a.parse::<u32>() {
        Ok(i) => i as i32,
        Err(_) => -1,
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    version_str: String,
}

impl Version {
    pub fn new(version_str: &str) -> Self {
        let version = Version::parse_version_str(version_str);

        Self {
            version_str: String::from(version_str),
            major: version.0,
            minor: version.1,
            patch: version.2,
        }
    }

    pub fn greater_than_or_equal(&self, other: &Version) -> bool {
        if self.major < other.major {
            return false;
        }
        if self.major > other.major {
            return true;
        }
        if self.minor < other.minor {
            return false;
        }
        if self.minor > other.minor {
            return true;
        }
        return self.patch >= other.patch;
    }

    pub fn to_string(&self) -> String {
        self.version_str.clone()
    }

    /// Converts the specified `version_str` to its number constituents. Invalid
    /// number value is represented as negative number.
    pub fn parse_version_str(version_str: &str) -> (i32, i32, i32) {
        let version = version_str
            .split('.')
            .map(parse_non_negative_int)
            .collect::<Vec<i32>>();

        (
            match version.get(0) {
                Some(&major) => major,
                None => 0,
            },
            match version.get(1) {
                Some(&minor) => minor,
                None => 0,
            },
            match version.get(2) {
                Some(&patch) => patch,
                None => 0,
            },
        )
    }
}
