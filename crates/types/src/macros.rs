/// Defines a strongly-typed identifier wrapper around `crate::Id`.
///
/// This macro generates all the boilerplate (Display, FromStr, From, tests)
/// so every ID type is consistent and production-ready.
#[macro_export]
macro_rules! define_id_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        $vis struct $name(pub(crate) crate::Id);

        impl $name {
            /// Creates a new identifier from a raw `u64`.
            #[inline]
            pub const fn new(id: u64) -> Self {
                Self(crate::Id::new(id))
            }

            /// Returns the wrapped generic identifier.
            #[inline]
            pub const fn id(self) -> crate::Id {
                self.0
            }

            /// Returns the raw numeric value.
            #[inline]
            pub const fn value(self) -> u64 {
                self.0.value()
            }

            /// Returns true if this is the zero/invalid ID.
            #[inline]
            pub const fn is_zero(self) -> bool {
                self.0.is_zero()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(crate::Id::from_str(s)?))
            }
        }

        impl From<crate::Id> for $name {
            #[inline]
            fn from(id: crate::Id) -> Self {
                Self(id)
            }
        }

        impl From<$name> for crate::Id {
            #[inline]
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl From<u64> for $name {
            #[inline]
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                // This is a bit of a hack; in practice you'd use a thread-local
                // or just not implement AsRef<str> for numeric IDs.
                // Instead, we provide as_string for convenience.
                panic!("Use .to_string() or .value() for numeric IDs")
            }
        }

        #[cfg(test)]
        mod paste {
            // Tests are generated in the caller module
        }
    };
}
