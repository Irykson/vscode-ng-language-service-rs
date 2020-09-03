use node_resolve;
use serde_json::Value;
use std::{fs, path::PathBuf};


static MIN_TS_VERSION: &str = "3.9";
static MIN_NG_VERSION: &str = "10.0";

pub struct NodeModule {
    name: String,
    resolved_path: String,
    version: Version,
}

fn resolve(package_name: &str, location: &str, root_package: Option<&str>) -> Option<NodeModule> {
    let root_package = match root_package {
        Some(rp) => rp,
        None => package_name,
    };

    // TODO:
    // - Pruefen, ob das ohne die viele match geht
    // - Pruefen, ob resolve_from sematisch gleich so require.resolve(package, {paths: [location]}) ist
    let package_json_path = match node_resolve::resolve_from(
        &format!("{}/package.json", root_package),
        PathBuf::from(location),
    ) {
        Ok(path) => String::from(path.to_str().unwrap()),
        Err(_) => return None,
    };

    let package_json: Value = match fs::read_to_string(package_json_path) {
        Ok(package) => serde_json::from_str(&package).unwrap(),
        Err(_) => return None,
    };

    let resolved_path = match node_resolve::resolve_from(package_name, PathBuf::from(location)) {
        Ok(rpath) => String::from(rpath.to_str().unwrap()),
        Err(_) => return None,
    };

    Some(NodeModule {
        name: String::from(package_name),
        resolved_path: String::from(resolved_path),
        version: Version::new(package_json["version"].as_str().unwrap()),
    })
}

fn resolve_with_min_version(
    package_name: &str,
    min_version_str: &str,
    probe_locations: Vec<&str>,
    root_package: &str,
) -> NodeModule {
    if !package_name.starts_with(root_package) {
        panic!(format!("{} must be in the root package", package_name));
    }

    let min_version = Version::new(min_version_str);
    for location in probe_locations.iter() {
        let node_module = resolve(package_name, location, Some(root_package));

        match node_module {
            Some(nm) => {
                if nm.version.greater_than_or_equal(&min_version) {
                    return nm;
                }
            }
            None => continue,
        }
    }

    panic!(format!(
        "Failed to resolve '{}' with minimum version '{}' from {}",
        package_name,
        min_version.to_string(),
        "TODO: print string array" /*probe_locations*/
    ));
}

pub fn resolve_ts_server(probe_locations: Vec<&str>) -> NodeModule {
    let tsserver = "typescript/lib/tsserverlibrary";

    resolve_with_min_version(tsserver, MIN_TS_VERSION, probe_locations, "typescript")
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
