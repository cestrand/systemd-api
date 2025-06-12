use std::{self, io::{self, BufRead}, process::Output};


#[derive(Debug)]
pub struct CallSystemctlVersionOutput {
    pub version_line: String,
    pub feature_list: String,
}
#[derive(Debug)]
pub enum CallSystemctlError {
    CommandFailed(io::Error),
    NonZeroExit(Option<i32>),
    MissingOutputLine(&'static str),
    InvalidUtf8(std::string::FromUtf8Error),
}
pub fn call_systemctl_version() -> Result<CallSystemctlVersionOutput, CallSystemctlError> {
    let out = std::process::Command::new("systemctl")
    .arg("--version")
    .output().map_err(CallSystemctlError::CommandFailed)?;

    if !out.status.success() {
        return Err(CallSystemctlError::NonZeroExit(out.status.code()))
    }

    // Example output:
    // systemd 257 (257.5)
    // +PAM -AUDIT -SELINUX -APPARMOR +IMA +IPE +SMACK +SECCOMP +GCRYPT -GNUTLS +OPENSSL +ACL +BLKID +CURL -ELFUTILS -FIDO2 +IDN2 -IDN -IPTC +KMOD -LIBCRYPTSETUP -LIBCRYPTSETUP_PLUGINS +LIBFDISK +PCRE2 -PWQUALITY -P11KIT -QRENCODE -TPM2 -BZIP2 +LZ4 -XZ -ZLIB +ZSTD -BPF_FRAMEWORK -BTF -XKBCOMMON +UTMP -SYSVINIT +LIBARCHIVE
    let stdout = String::from_utf8(out.stdout).map_err(CallSystemctlError::InvalidUtf8)?;

    let mut lines = stdout.lines();
    let version_line = lines.next().ok_or(CallSystemctlError::MissingOutputLine("version_line"))?.to_string();
    let feature_list = lines.next().ok_or(CallSystemctlError::MissingOutputLine("feature_list"))?.to_string();
    
    Ok(CallSystemctlVersionOutput {
        version_line, feature_list
    })
}
#[test]
fn run_call_systemctl_version() {
    println!("{:?}", call_systemctl_version());
}

fn extract_version(s: &str) -> Option<&str> {
    let start = s.find('(')? + 1;
    let end = s[start..].find(')')?;
    Some(&s[start..start+end])
}
fn parse_version(s: &str) -> Option<(u32, u32)> {
    let (a,b) = s.split_once('.')?;
    let version = a.parse().ok()?;
    let patch = b.parse().ok()?;
    Some((version, patch))
}
#[derive(Debug)]
pub struct Version {
    pub version: u32,
    pub patch: u32,
}
#[derive(Debug)]
pub enum SystemctlVersionError {
    Call(CallSystemctlError),
    WrongVersionLine(String),
    VersionParseError(String),
}
pub fn systemctl_version() -> Result<Version, SystemctlVersionError> {
    use SystemctlVersionError::*;

    let ver = call_systemctl_version().map_err(|e| Call(e))?;
    let extracted = extract_version(&ver.version_line).ok_or(WrongVersionLine(ver.version_line.clone()))?;

    let (version, patch) = parse_version(extracted).ok_or(VersionParseError(extracted.to_string()))?;
    Ok(Version {version, patch})
}
#[test]
fn run_systemctl_version() {
    println!("{:?}", systemctl_version());
}