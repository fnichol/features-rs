# Features

## About

`features` is a small library that implements runtime [feature toggles][fowler_toggles] for
your library or program allowing behavior to be changed on boot or dynamically at runtime using
the same compiled binary artifact. This is different from cargo's [feature][cargo_feature]
support which uses conditional compilation.

At its core, it is a macro (`features!`) that takes a collection of feature flag names which it
uses to generate a module containing a function to enable a feature toggle (`::enable()`), a
function to disable a feature toggle (`::disable()`) and a function to check if a feature
toggle is enabled (`::is_enabled()`).

[fowler_toggles]: https://martinfowler.com/articles/feature-toggles.html
[cargo_feature]: http://doc.crates.io/manifest.html#the-features-section

## Example

Basic example:

```rust
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate features;

features! {
    pub mod feature {
        const Alpha = 0b00000001,
        const Beta = 0b00000010
    }
}

fn main() {
    assert_eq!(false, feature::is_enabled(feature::Alpha));
    assert_eq!(false, feature::is_enabled(feature::Beta));

    feature::enable(feature::Beta);
    assert_eq!(false, feature::is_enabled(feature::Alpha));
    assert_eq!(true, feature::is_enabled(feature::Beta));
}
```

Multiple feature sets:

```rust
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate features;

features! {
    pub mod ux {
        const JsonOutput = 0b10000000,
        const VerboseOutput = 0b01000000
    }
}

features! {
    pub mod srv {
        const Http2Downloading = 0b10000000,
        const BitTorrentDownloading = 0b01000000
    }
}

fn main() {
    // Parse CLI args, environment, read config file etc...
    srv::enable(srv::BitTorrentDownloading);
    ux::enable(ux::JsonOutput);

    if srv::is_enabled(srv::Http2Downloading) {
        println!("Downloading via http2...");
    } else if srv::is_enabled(srv::BitTorrentDownloading) {
        println!("Downloading via bit torrent...");
    } else {
        println!("Downloading the old fashioned way...");
    }

    if ux::is_enabled(ux::VerboseOutput) {
        println!("COOL");
    }
}
```

## Feature Toggle References

Here are some articles and projects which insipred the implementation of `features`:

* [Feature Toggles](https://martinfowler.com/articles/feature-toggles.html) (Martin Fowler's
blog)
* [Using Feature Flags to Ship Changes with
Confidence](https://blog.travis-ci.com/2014-03-04-use-feature-flags-to-ship-changes-with-confidence/)
(TravisCI's blog)
* [Feature Toggles are one of the worst kinds of Technical
Debt](http://swreflections.blogspot.ca/2014/08/feature-toggles-are-one-of-worst-kinds.html)
(excellent cautionary tale and warning)
* [Feature toggle](https://en.wikipedia.org/wiki/Feature_toggle) (Wikipedia article)
* [Ruby feature gem](https://github.com/mgsnova/feature) (nice prior art)

## License

Features is licensed under the Apache License, Version 2.0 and the MIT license. Please read the [LICENSE-APACHE] and [LICENSE-MIT] for details

[LICENSE-APACHE]: https://github.com/fnichol/features-rs/blob/master/LICENSE-MIT
[LICENSE-MIT]: https://github.com/fnichol/features-rs/blob/master/MIT-MIT
