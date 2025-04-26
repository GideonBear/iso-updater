use crate::iso_file::IsoFile;
use crate::iso_source::IsoSource;
use crate::utils::download;
use color_eyre::eyre::{OptionExt, eyre};
use color_eyre::{Report, Result};
use command_error::CommandExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::process::Command;
use std::str::FromStr;
use tempdir::TempDir;
use url::Url;

fn parse_sha256sum_file(s: &str) -> Result<HashMap<Edition, String>> {
    s.split('\n')
        .filter_map(|line| line.split_once(' '))
        .map(|(hash, filename)| {
            Ok::<(Edition, String), Report>((
                filename
                    .split('-')
                    .nth(2)
                    .ok_or_eyre("Filename had less than 3 parts (separated by -)")?
                    .parse::<Edition>()?,
                hash.to_string(),
            ))
        })
        .collect()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
enum Edition {
    Cinnamon,
    Mate,
    Xfce,
}

impl Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edition::Cinnamon => write!(f, "cinnamon"),
            Edition::Mate => write!(f, "mate"),
            Edition::Xfce => write!(f, "xfce"),
        }
    }
}

impl FromStr for Edition {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "cinnamon" => Edition::Cinnamon,
            "mate" => Edition::Mate,
            "xfce" => Edition::Xfce,
            _ => return Err(eyre!("Invalid edition")),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Version {
    major: u32,
    minor: u32,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let major_ord = self.major.cmp(&other.major);
        match major_ord {
            Ordering::Equal => self.minor.cmp(&other.minor),
            major_ord => major_ord,
        }
    }
}

impl FromStr for Version {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.split_once('.') {
            Some((major, minor)) => Self {
                major: major.parse()?,
                minor: minor.parse()?,
            },
            None => Self {
                major: s.parse()?,
                minor: 0,
            },
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

const URL: &str = "https://mirrors.edge.kernel.org/linuxmint/stable/";

fn get_files() -> Result<Vec<Url>, io::Error> {
    // SAFETY: This is a valid URL
    let url = Url::parse(URL).unwrap();
    httpdir::Crawler::new().walk(url.clone()).collect()
}

fn get_versions(files: &Vec<Url>) -> Result<Vec<Version>> {
    let mut versions: Vec<Version> = files
        .iter()
        .map(|url| url.path_segments()?.nth(2).map(|x| x.parse()))
        .collect::<Option<Result<Vec<Version>>>>()
        .ok_or_eyre("Url path has less than three segments")??;
    versions.sort();
    versions.dedup();
    Ok(versions)
}

fn get_latest_version() -> Result<Version> {
    let files = get_files()?;
    let versions = get_versions(&files)?;
    versions.into_iter().max().ok_or_eyre("No versions found")
}

fn download_version(version: &Version, edition: &Edition, temp: &TempDir) -> Result<IsoFile> {
    let base_url = format!("{URL}{version}");
    let iso_url = format!("{base_url}/linuxmint-{version}-{edition}-64bit.iso");
    let hash_url = format!("{base_url}/sha256sum.txt");
    let gpg_url = format!("{hash_url}.gpg");

    // gpg --keyserver hkp://keys.openpgp.org:80 --recv-key 27DEB15644C6B3CF3BD7D291300F846BA25BAE09
    Command::new("gpg")
        .arg("--keyserver")
        .arg("hkp://keys.openpgp.org:80")
        .arg("--recv-key")
        .arg("27DEB15644C6B3CF3BD7D291300F846BA25BAE09")
        .status_checked()?;

    let iso_path = temp.path().join("linuxmint.iso");
    let hash_path = temp.path().join("sha256sum.txt");
    let gpg_path = temp.path().join("sha256sum.txt.gpg");

    download(&hash_url, &hash_path)?;
    download(&gpg_url, &gpg_path)?;

    // gpg will exit with code 1 if the signature is invalid
    // gpg --verify sha256sum.txt.gpg sha256sum.txt
    Command::new("gpg")
        .arg("--verify")
        .arg(&gpg_path)
        .arg(&hash_path)
        .status_checked()?;

    let mut sha256sum_file = parse_sha256sum_file(&std::fs::read_to_string(&hash_path)?)?;
    let sha256sum = sha256sum_file
        .remove(edition) // .remove instead of .get so we get ownership. We don't use the value afterwards.
        .ok_or_else(|| eyre!("Could not find sha256sum for edition {edition} in {hash_path:?}"))?;

    download(&iso_url, &iso_path)?;

    println!("Hashing...");
    let iso_file = IsoFile::new(&iso_path, Some(version.to_string()))?;

    if iso_file.hash != *sha256sum {
        return Err(eyre!(
            "Hash mismatch! Expected {sha256sum}, got {}",
            iso_file.hash
        ));
    } else {
        println!("Hash matches: {sha256sum}")
    }

    Ok(iso_file)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinuxMint {
    edition: Edition,
}

impl IsoSource<'_> for LinuxMint {
    fn latest(&self, temp: &TempDir) -> Result<IsoFile> {
        let latest_version = get_latest_version()?;

        download_version(&latest_version, &self.edition, temp)
    }

    fn updated(&self, existing: &IsoFile, temp: &TempDir) -> Result<Option<IsoFile>> {
        let latest_version = get_latest_version()?;

        if latest_version
            > existing
                .version
                .clone()
                .ok_or_eyre("IsoFile doesn't have version")?
                .parse()?
        {
            Ok(Some(download_version(
                &latest_version,
                &self.edition,
                temp,
            )?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_ordering() {
        assert!(
            Version {
                major: 20,
                minor: 1
            } < Version {
                major: 21,
                minor: 0
            }
        );
        assert!(
            Version {
                major: 20,
                minor: 1
            } < Version {
                major: 20,
                minor: 2
            }
        );
        assert_eq!(
            Version {
                major: 20,
                minor: 1
            },
            Version {
                major: 20,
                minor: 1
            }
        );
    }

    #[test]
    fn test_version_parse() {
        assert_eq!(
            "20.1".parse::<Version>().unwrap(),
            Version {
                major: 20,
                minor: 1
            }
        );
        assert_eq!(
            "20".parse::<Version>().unwrap(),
            Version {
                major: 20,
                minor: 0
            }
        );
    }

    #[test]
    fn test_version_parse_error() {
        assert!("invalid".parse::<Version>().is_err());
        assert!("20.invalid".parse::<Version>().is_err());
        assert!("invalid.1".parse::<Version>().is_err());
    }

    #[test]
    fn test_get_versions() {
        let files = get_files().unwrap();
        let versions = get_versions(&files).unwrap();
        assert_eq!(
            versions[0..11], // Slicing to take into account new versions
            [
                "19.3", "20.0", "20.1", "20.2", "20.3", "21.0", "21.1", "21.2", "21.3", "22.0",
                "22.1"
            ]
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<Version>>()
        )
    }

    #[test]
    #[ignore]
    fn test_latest_downloads() {
        let temp = TempDir::new("iso_updater_test").unwrap();
        let source = LinuxMint {
            edition: Edition::Cinnamon,
        };
        let iso = source.latest(&temp).unwrap();
        assert!(
            iso.version.unwrap().parse::<Version>().unwrap() >= "22.1".parse::<Version>().unwrap()
        );
    }
}
