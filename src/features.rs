//! Contains code for selecting features

#![deny(missing_docs)]
#![deny(warnings)]
#![deny(unused_extern_crates)]

use std::io;

/// Define RustTarget struct definition, Default impl, and conversions
/// between RustTarget and String.
macro_rules! rust_target_def {
    ( $( $release:ident => $value:expr => $attrs:meta; )* ) => {
        /// Represents the version of the Rust language to target.
        ///
        /// To support a beta release, use the corresponding stable release.
        ///
        /// This enum will have more variants added as necessary.
        #[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
        #[allow(non_camel_case_types)]
        pub enum RustTarget {
            $(
                #[$attrs]
                $release,
            )*
        }

        impl Default for RustTarget {
            /// Gives the latest stable Rust version
            fn default() -> RustTarget {
                LATEST_STABLE_RUST
            }
        }

        impl RustTarget {
            #[allow(dead_code)]
            /// Create a `RustTarget` from a string.
            ///
            /// * The stable/beta versions of Rust are of the form "1.0",
            /// "1.19", etc.
            /// * The nightly version should be specified with "nightly".
            pub fn from_str<'a>(s: &'a str) -> io::Result<RustTarget> {
                match s.as_ref() {
                    $(
                        stringify!($value) => Ok(RustTarget::$release),
                    )*
                    _ => Err(
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            concat!(
                                "Got an invalid rust target. Accepted values ",
                                "are of the form ",
                                "\"1.0\" or \"nightly\"."))),
                }
            }
        }

        impl From<RustTarget> for String {
            fn from(target: RustTarget) -> Self {
                match target {
                    $(
                        RustTarget::$release => stringify!($value),
                    )*
                }.into()
            }
        }
    }
}

rust_target_def!(
    Stable_1_0 => 1.0 => doc="Rust stable 1.0";
    Stable_1_19 => 1.19 => doc="Rust stable 1.19";
    Nightly => nightly => doc="Nightly rust";
);

/// Latest stable release of Rust
pub const LATEST_STABLE_RUST: RustTarget = RustTarget::Stable_1_19;

/// Create RustFeatures struct definition, new(), and a getter for each field
macro_rules! rust_feature_def {
    ( $( $feature:ident => $attrs:meta; )* ) => {
        /// Features supported by a rust target
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct RustFeatures {
            $(
                $feature: bool,
            )*
        }

        impl RustFeatures {
            /// Gives a RustFeatures struct with all features disabled
            fn new() -> Self {
                RustFeatures {
                    $(
                        $feature: false,
                    )*
                }
            }

            $(
                #[$attrs]
                pub fn $feature(&self) -> bool {
                    self.$feature
                }
            )*
        }
    }
}

rust_feature_def!(
    untagged_union => doc="Untagged unions ([RFC 1444](https://github.com/rust-lang/rfcs/blob/master/text/1444-union.md))";
    const_fn => doc="Constant function ([RFC 911](https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md))";
);

impl From<RustTarget> for RustFeatures {
    fn from(rust_target: RustTarget) -> Self {
        let mut features = RustFeatures::new();

        if rust_target >= RustTarget::Stable_1_19 {
            features.untagged_union = true;
        }

        if rust_target >= RustTarget::Nightly {
            features.const_fn = true;
        }

        features
    }
}

impl Default for RustFeatures {
    fn default() -> Self {
        let default_rust_target: RustTarget = Default::default();
        Self::from(default_rust_target)
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;

    fn test_target(target_str: &str, target: RustTarget) {
        let target_string: String = target.into();
        assert_eq!(target_str, target_string);
        assert_eq!(target, RustTarget::from_str(target_str).unwrap());
    }

    #[test]
    fn str_to_target() {
        test_target("1.0", RustTarget::Stable_1_0);
        test_target("1.19", RustTarget::Stable_1_19);
        test_target("nightly", RustTarget::Nightly);
    }
}
