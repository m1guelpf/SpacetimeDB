use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::directories::Layout;
use crate::errors::ErrorPlatform;
use crate::toml::{read_toml, write_toml};
use spacetimedb_lib::Address;

/// Enum representing different binaries.
pub enum Bin {
    Spacetime,
    StandAlone,
    Cloud,
    Cli,
    Update,
}

impl Bin {
    pub fn name_unix(&self) -> &'static str {
        match self {
            Bin::Spacetime => "spacetime",
            Bin::StandAlone => "spacetimedb-standalone",
            Bin::Cloud => "spacetimedb-cloud",
            Bin::Cli => "spacetimedb-cli",
            Bin::Update => "spacetimedb-update",
        }
    }
    pub fn name_windows(&self) -> &'static str {
        match self {
            Bin::Spacetime => "spacetime.exe",
            Bin::StandAlone => "spacetimedb-standalone.exe",
            Bin::Cloud => "spacetimedb-cloud.exe",
            Bin::Cli => "spacetimedb-cli.exe",
            Bin::Update => "spacetimedb-update.exe",
        }
    }
    /// Get the name of the binary according to the platform inferred from the [Layout].
    pub fn name(&self, layout: Layout) -> &'static str {
        if layout.platform().nix_like() {
            self.name_unix()
        } else {
            self.name_windows()
        }
    }
}

/// Version of the executable.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = ErrorPlatform;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('.');
        match (iter.next(), iter.next(), iter.next()) {
            (Some(major), Some(minor), Some(patch)) => {
                let parse_u64 = |s: &str| -> Result<u64, ErrorPlatform> {
                    s.parse()
                        .map_err(|_| ErrorPlatform::ParseVersion { version: s.to_string() })
                };

                let major = parse_u64(major)?;
                let minor = parse_u64(minor)?;
                let patch = parse_u64(patch)?;
                Ok(Version::new(major, minor, patch))
            }
            _ => Err(ErrorPlatform::ParseVersion { version: s.to_string() }),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum EditionKind {
    StandAlone,
    Cloud,
}

/// Edition of the executable.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Edition {
    pub kind: EditionKind,
    pub version: Version,
}

impl Edition {
    pub fn cloud(major: u64, minor: u64, patch: u64) -> Self {
        Edition {
            kind: EditionKind::Cloud,
            version: Version::new(major, minor, patch),
        }
    }

    pub fn standalone(major: u64, minor: u64, patch: u64) -> Self {
        Edition {
            kind: EditionKind::StandAlone,
            version: Version::new(major, minor, patch),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EditionOptionsRaw {
    StandAlone {
        major: u64,
        minor: u64,
        patch: u64,
    },
    Cloud {
        major: u64,
        minor: u64,
        patch: u64,
        /// The client address.
        address: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum EditionOptions {
    StandAlone {
        major: u64,
        minor: u64,
        patch: u64,
    },
    Cloud {
        major: u64,
        minor: u64,
        patch: u64,
        /// The client address.
        address: Address,
    },
}

impl EditionOptions {
    pub fn kind(&self) -> EditionKind {
        match self {
            EditionOptions::StandAlone { .. } => EditionKind::StandAlone,
            EditionOptions::Cloud { .. } => EditionKind::Cloud,
        }
    }
    pub fn version(&self) -> Version {
        match self {
            EditionOptions::StandAlone { major, minor, patch } => Version::new(*major, *minor, *patch),
            EditionOptions::Cloud {
                major, minor, patch, ..
            } => Version::new(*major, *minor, *patch),
        }
    }
}

/// Raw representation of the metadata, for serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawMetadata {
    pub edition: EditionOptionsRaw,
}

/// Store the metadata of the executable(ie: edition, version, client-address * -if cloud-) that created the directory.
///
/// Defined in the `metadata.toml` file.
#[derive(Debug, Clone)]
pub struct Metadata {
    pub edition: EditionOptions,
}

impl Metadata {
    /// Read the [Metadata] from the given `path`.
    pub fn read(path: &PathBuf) -> Result<Self, ErrorPlatform> {
        let config: RawMetadata = read_toml(path)?;

        let edition = match config.edition {
            EditionOptionsRaw::StandAlone { major, minor, patch } => EditionOptions::StandAlone { major, minor, patch },
            EditionOptionsRaw::Cloud {
                major,
                minor,
                patch,
                address,
            } => EditionOptions::Cloud {
                major,
                minor,
                patch,
                address: Address::from_hex(&address)?,
            },
        };

        Ok(Metadata { edition })
    }

    /// Write the [Metadata] to the given `path`.
    pub fn write(&self, path: PathBuf) -> Result<(), ErrorPlatform> {
        let edition = match &self.edition {
            EditionOptions::StandAlone { major, minor, patch } => EditionOptionsRaw::StandAlone {
                major: *major,
                minor: *minor,
                patch: *patch,
            },
            EditionOptions::Cloud {
                major,
                minor,
                patch,
                address,
            } => EditionOptionsRaw::Cloud {
                major: *major,
                minor: *minor,
                patch: *patch,
                address: address.to_string(),
            },
        };
        let raw = RawMetadata { edition };
        write_toml(path, &raw)
    }
}
