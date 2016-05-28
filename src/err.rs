
use std::error;
use std::fmt;
use std::io::Error as ioError;

#[derive(Debug)]
/// Represents potential errors that may occur when profiling
pub enum NBAError {
    RegexError,
    InvalidProfiler,
    InvalidBinary,
    InvalidNum,
    InvalidSortMetric,
    /// Wraps a std::io::Error
    IOError(ioError),
    UTF8Error,
    MisalignedData,
    CompilationError(String, String),
    TomlError,
    ReadManifestError,
    NoNameError,
    NoTargetDirectory,
    OutOfMemoryError,
    CliError,
}

impl fmt::Display for NBAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NBAError::RegexError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mRegex error -- please file a bug. In bug report, \
                        please include the original output file from profiler, e.g. from \
                        valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.txt")
            }
            NBAError::InvalidProfiler => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mInvalid profiler. cargo profiler currently \
                        supports callgrind and cachegrind.")
            }
            NBAError::InvalidBinary => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mInvalid binary. make sure binary exists.")
            }
            NBAError::InvalidNum => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mInvalid number. make sure number is a positive \
                        integer.")
            }
            NBAError::InvalidSortMetric => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mInvalid metric to sort on. available cachegrind \
                        metrics are \nir, i1mr, ilmr, dr, d1mr, dlmr, dw, d1mw, and dlmw. Check \
                        README for details on these metrics.")
            }
            NBAError::IOError(ref err) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mIO error: {} -- please file a bug.",
                       err)
            }
            NBAError::UTF8Error => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mCLI Utf8 error -- please file a bug.")
            }
            NBAError::MisalignedData => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mMisaligned data arrays due to regex error -- \
                        please file a bug.")
            }
            NBAError::CompilationError(ref package_name, ref stderr) => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mFailed to compile {}.\n\n{}",
                       package_name,
                       stderr)
            }
            NBAError::TomlError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mError in parsing Cargo.toml to derive package \
                        name. Make sure package name is directly under [package] tag.")
            }
            NBAError::ReadManifestError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mCargo.toml missing. Are you sure you're in a Rust \
                        project?")
            }

            NBAError::NoNameError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mNo package name found in Cargo.toml. Run \
                        cargo read-manifest to make sure everything looks okay. Otherwise please \
                        submit bug.")
            }

            NBAError::NoTargetDirectory => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mNo target output directory found in project. \
                        Binary must be in target/debug/ or target/release/, or specify binary \
                        path explicitly with --bin argument.")
            }
            NBAError::OutOfMemoryError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mValgrind's memory management: out of memory. \
                        Valgrind cannot continue. Sorry. ")
            }
            NBAError::CliError => {
                write!(f,
                       "\x1b[1;31merror: \x1b[0mError in valgrind cli call. Make sure valgrind is \
                        installed properly.")
            }
        }
    }
}

impl error::Error for NBAError {
    fn description(&self) -> &str {
        match *self {
            NBAError::RegexError => "Regex error. file bug.",
            NBAError::InvalidProfiler => "Invalid Profiler.",
            NBAError::InvalidBinary => "Invalid Binary.",
            NBAError::InvalidNum => "Invalid number.",
            NBAError::InvalidSortMetric => "Invalid sort metric.",
            NBAError::MisalignedData => "Misaligned Data. File bug.",
            NBAError::CompilationError(_, _) => {
                "Failed to compile. Run cargo build to get compilation error."
            }
            NBAError::TomlError => "Error in parsing Cargo.toml.",
            NBAError::ReadManifestError => "Error in reading the manifest of this crate.",
            NBAError::NoNameError => "No package name found in Cargo.toml",
            NBAError::NoTargetDirectory => "No target output directory found in project.",
            NBAError::IOError(ref err) => err.description(),
            NBAError::OutOfMemoryError => "out of memory.",
            NBAError::CliError => "make sure valgrind is installed properly.",
            NBAError::UTF8Error => "utf8 error. file bug.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            NBAError::RegexError => None,
            NBAError::InvalidProfiler => None,
            NBAError::InvalidBinary => None,
            NBAError::InvalidNum => None,
            NBAError::InvalidSortMetric => None,
            NBAError::MisalignedData => None,
            NBAError::TomlError => None,
            NBAError::IOError(ref err) => Some(err),
            NBAError::CompilationError(_, _) => None,
            NBAError::ReadManifestError => None,
            NBAError::NoNameError => None,
            NBAError::NoTargetDirectory => None,
            NBAError::OutOfMemoryError => None,
            NBAError::CliError => None,
            NBAError::UTF8Error => None,
        }
    }
}

impl From<ioError> for NBAError {
    fn from(err: ioError) -> NBAError {
        NBAError::IOError(err)
    }
}
