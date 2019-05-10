/// Allows you to pull the version from your Cargo.toml at compile time as
/// `MAJOR.MINOR.PATCH_PKGVERSION_PRE`
///
/// # Examples
///
/// ```no_run
/// # use metacrate::crate_version;
/// # fn main() {
///     println!(crate_version!())
/// # }
/// ```
#[cfg(not(feature = "no_cargo"))]
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Allows you to pull the authors for the app from your Cargo.toml at
/// compile time in the form:
/// `"author1 lastname <author1@example.com>:author2 lastname <author2@example.com>"`
///
/// You can replace the colons with a custom separator by supplying a
/// replacement string, so, for example,
/// `crate_authors!(",\n")` would become
/// `"author1 lastname <author1@example.com>,\nauthor2 lastname <author2@example.com>,\nauthor3 lastname <author3@example.com>"`
///
/// # Examples
///
/// ```no_run
/// # use metacrate::crate_authors;
/// # fn main() {
///     println!(crate_authors!())
/// # }
/// ```
#[cfg(not(feature = "no_cargo"))]
#[macro_export]
macro_rules! crate_authors {
    ($sep:expr) => {{
        use std::ops::Deref;
        use std::sync::{ONCE_INIT, Once};

        #[allow(missing_copy_implementations)]
        #[allow(dead_code)]
        struct CargoAuthors { __private_field: () };

        impl Deref for CargoAuthors {
            type Target = str;

            #[allow(unsafe_code)]
            fn deref(&self) -> &'static str {
                static ONCE: Once = ONCE_INIT;
                static mut VALUE: *const String = 0 as *const String;

                unsafe {
                    ONCE.call_once(|| {
                        let s = env!("CARGO_PKG_AUTHORS").replace(':', $sep);
                        VALUE = Box::into_raw(Box::new(s));
                    });

                    &(*VALUE)[..]
                }
            }
        }

        &*CargoAuthors { __private_field: () }
    }};
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

/// Allows you to pull the description from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```no_run
/// # use metacrate::crate_description;
/// # fn main() {
///     println!(crate_description!())
/// # }
/// ```
#[cfg(not(feature = "no_cargo"))]
#[macro_export]
macro_rules! crate_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    };
}

/// Allows you to pull the name from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```no_run
/// # use metacrate::crate_name;
/// # fn main() {
///     println!(crate_name!())
/// # }
/// ```
#[cfg(not(feature = "no_cargo"))]
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
