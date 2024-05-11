use core::fmt;

/// Error type returned by _try methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternError {
	InvalidPatternCapture,
	InvalidCaptureIndex(Option<i8>),
	EndsWithEscape,
	UnfinishedCharClass,
	MalformedBalance,
	MalformedFrontier,
	TooManyCaptures,
	MatchDepthExceeded,
	UnfinishedCapture,
	NoOpenCapture,
	NoCaptureLength,
}

impl fmt::Display for PatternError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InvalidPatternCapture => write!(f, "invalid pattern capture"),
			Self::InvalidCaptureIndex(None) => write!(f, "invalid capture index"),
			Self::InvalidCaptureIndex(Some(idx)) => write!(f, "invalid capture index %{}", (*idx as usize) + 1),
			Self::EndsWithEscape => write!(f, "malformed pattern (ends with '%')"),
			Self::UnfinishedCharClass => write!(f, "malformed pattern (missing ']')"),
			Self::MalformedBalance => write!(f, "malformed pattern (missing arguments to '%b')"),
			Self::MalformedFrontier => write!(f, "malformed pattern (missing '[' after '%f' in pattern)"),
			Self::TooManyCaptures => write!(f, "too many captures"),
			Self::MatchDepthExceeded => write!(f, "pattern too complex"),
			Self::UnfinishedCapture => write!(f, "unfinished capture"),
			Self::NoOpenCapture => write!(f, "no open capture"),
			Self::NoCaptureLength => write!(f, "capture was unfinished or positional"),
		}
	}
}

#[cfg(feature = "std")]
use std::error::Error;

#[cfg(feature = "std")]
impl Error for PatternError { }

