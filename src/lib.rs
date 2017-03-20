// Copyright ⓒ 2017 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 (see LICENSE-APACHE or
// <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT license (see<LICENSE-MIT or
// <http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! `features` is a small library that implements runtime [feature toggles][fowler_toggles] for
//! your library or program allowing behavior to be changed on boot or dynamically at runtime using
//! the same compiled binary artifact. This is different from cargo's [feature][cargo_feature]
//! support which uses conditional compilation.
//!
//! At its core, it is a macro (`features!`) that takes a collection of feature flag names which it
//! uses to generate a module containing a function to enable a feature toggle (`::enable()`), a
//! function to disable a feature toggle (`::disable()`) and a function to check if a feature
//! toggle is enabled (`::is_enabled()`).
//!
//! ## Example
//!
//! Basic example:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//! #[macro_use]
//! extern crate features;
//!
//! features! {
//!     pub mod feature {
//!         const Alpha = 0b00000001,
//!         const Beta = 0b00000010
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(false, feature::is_enabled(feature::Alpha));
//!     assert_eq!(false, feature::is_enabled(feature::Beta));
//!
//!     feature::enable(feature::Beta);
//!     assert_eq!(false, feature::is_enabled(feature::Alpha));
//!     assert_eq!(true, feature::is_enabled(feature::Beta));
//! }
//! ```
//!
//! Multiple feature sets:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//! #[macro_use]
//! extern crate features;
//!
//! features! {
//!     pub mod ux {
//!         const JsonOutput = 0b10000000,
//!         const VerboseOutput = 0b01000000
//!     }
//! }
//!
//! features! {
//!     pub mod srv {
//!         const Http2Downloading = 0b10000000,
//!         const BitTorrentDownloading = 0b01000000
//!     }
//! }
//!
//! fn main() {
//!     // Parse CLI args, environment, read config file etc...
//!     srv::enable(srv::BitTorrentDownloading);
//!     ux::enable(ux::JsonOutput);
//!
//!     if srv::is_enabled(srv::Http2Downloading) {
//!         println!("Downloading via http2...");
//!     } else if srv::is_enabled(srv::BitTorrentDownloading) {
//!         println!("Downloading via bit torrent...");
//!     } else {
//!         println!("Downloading the old fashioned way...");
//!     }
//!
//!     if ux::is_enabled(ux::VerboseOutput) {
//!         println!("COOL");
//!     }
//! }
//! ```
//!
//! ## Feature Toggle References
//!
//! Here are some articles and projects which insipred the implementation of `features`:
//!
//! * [Feature Toggles](https://martinfowler.com/articles/feature-toggles.html) (Martin Fowler's
//! blog)
//! * [Using Feature Flags to Ship Changes with
//! Confidence](https://blog.travis-ci.com/2014-03-04-use-feature-flags-to-ship-changes-with-confidence/)
//! (TravisCI's blog)
//! * [Feature Toggles are one of the worst kinds of Technical
//! Debt](http://swreflections.blogspot.ca/2014/08/feature-toggles-are-one-of-worst-kinds.html)
//! (excellent cautionary tale and warning)
//! * [Feature toggle](https://en.wikipedia.org/wiki/Feature_toggle) (Wikipedia article)
//! * [Ruby feature gem](https://github.com/mgsnova/feature) (nice prior art)
//!
//! [fowler_toggles]: https://martinfowler.com/articles/feature-toggles.html
//! [cargo_feature]: http://doc.crates.io/manifest.html#the-features-section

#[macro_use]
extern crate bitflags;

/// The `features!` macro generates a module to contain all feature toggling logic.
///
/// # Examples
///
/// Basic example:
///
/// ```
/// #[macro_use]
/// extern crate bitflags;
/// #[macro_use]
/// extern crate features;
///
/// features! {
///     pub mod feature {
///         const Alpha = 0b00000001,
///         const Beta = 0b00000010
///     }
/// }
///
/// fn main() {
///     assert_eq!(false, feature::is_enabled(feature::Alpha));
///     assert_eq!(false, feature::is_enabled(feature::Beta));
///
///     feature::enable(feature::Beta);
///     assert_eq!(false, feature::is_enabled(feature::Alpha));
///     assert_eq!(true, feature::is_enabled(feature::Beta));
/// }
/// ```
///
/// Multiple feature sets:
///
/// ```
/// #[macro_use]
/// extern crate bitflags;
/// #[macro_use]
/// extern crate features;
///
/// features! {
///     pub mod ux {
///         const JsonOutput = 0b10000000,
///         const VerboseOutput = 0b01000000
///     }
/// }
///
/// features! {
///     pub mod srv {
///         const Http2Downloading = 0b10000000,
///         const BitTorrentDownloading = 0b01000000
///     }
/// }
///
/// fn main() {
///     // Parse CLI args, environment, read config file etc...
///     srv::enable(srv::BitTorrentDownloading);
///     ux::enable(ux::JsonOutput);
///
///     if srv::is_enabled(srv::Http2Downloading) {
///         println!("Downloading via http2...");
///     } else if srv::is_enabled(srv::BitTorrentDownloading) {
///         println!("Downloading via bit torrent...");
///     } else {
///         println!("Downloading the old fashioned way...");
///     }
///
///     if ux::is_enabled(ux::VerboseOutput) {
///         println!("COOL");
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! features {
    (mod $mod_name:ident {
        $($(#[$flag_attr:meta])* const $flag:ident = $value:expr),+
    }) => {
        #[allow(non_upper_case_globals)]
        mod $mod_name {
            features! {
                @_impl mod $mod_name {
                    $($(#[$flag_attr])* const $flag = $value),+
                }
            }
        }
    };
    (pub mod $mod_name:ident {
        $($(#[$flag_attr:meta])* const $flag:ident = $value:expr),+
    }) => {
        #[allow(non_upper_case_globals)]
        pub mod $mod_name {
            features! {
                @_impl mod $mod_name {
                    $($(#[$flag_attr])* const $flag = $value),+
                }
            }
        }
    };
    (@_impl mod $mod_name:ident {
        $($(#[$flag_attr:meta])* const $flag:ident = $value:expr),+
    }) => {
        use std::sync::atomic;

        bitflags! {
            pub flags Flags: usize {
                $($(#[$flag_attr])* const $flag = $value),+
            }
        }

        static mut FEATURES: atomic::AtomicUsize = atomic::ATOMIC_USIZE_INIT;

        #[allow(dead_code)]
        pub fn enable(flag: Flags) {
            let mut features = unsafe { FEATURES.get_mut() };
            let mut flags = Flags::from_bits_truncate(*features);
            flags.insert(flag);
            *features = flags.bits();
        }

        #[allow(dead_code)]
        pub fn disable(flag: Flags) {
            let mut features = unsafe { FEATURES.get_mut() };
            let mut flags = Flags::from_bits_truncate(*features);
            flags.remove(flag);
            *features = flags.bits();
        }

        #[allow(dead_code)]
        pub fn is_enabled(flag: Flags) -> bool {
            flags().contains(flag)
        }

        #[allow(dead_code)]
        pub fn flags() -> Flags {
            unsafe { Flags::from_bits_truncate(FEATURES.load(atomic::Ordering::Relaxed)) }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn enabling() {
        features! {
            pub mod f {
                const Alpha = 0b00000001,
                const Beta = 0b00000010
            }
        }

        assert_eq!(false, f::is_enabled(f::Alpha));
        assert_eq!(false, f::is_enabled(f::Beta));

        f::enable(f::Alpha);
        assert_eq!(true, f::is_enabled(f::Alpha));
        assert_eq!(false, f::is_enabled(f::Beta));

        // Enable again
        f::enable(f::Alpha);
        assert_eq!(true, f::is_enabled(f::Alpha));
        assert_eq!(false, f::is_enabled(f::Beta));

        f::enable(f::Beta);
        assert_eq!(true, f::is_enabled(f::Alpha));
        assert_eq!(true, f::is_enabled(f::Beta));
    }

    #[test]
    fn disabling() {
        features! {
            pub mod f {
                const Cool = 0b01000000,
                const Beans = 0b10000000
            }
        }

        f::enable(f::Cool);
        f::enable(f::Beans);
        assert_eq!(true, f::is_enabled(f::Cool));
        assert_eq!(true, f::is_enabled(f::Beans));

        f::disable(f::Cool);
        assert_eq!(false, f::is_enabled(f::Cool));
        assert_eq!(true, f::is_enabled(f::Beans));

        // Disable again
        f::disable(f::Cool);
        assert_eq!(false, f::is_enabled(f::Cool));
        assert_eq!(true, f::is_enabled(f::Beans));

        f::disable(f::Beans);
        assert_eq!(false, f::is_enabled(f::Cool));
        assert_eq!(false, f::is_enabled(f::Beans));
    }
}
