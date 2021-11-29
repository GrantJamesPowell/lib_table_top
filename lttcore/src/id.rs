//! Helpers around ids
//!
//! This module contains the "ID"s for various domain concepts within `lib_table_top`. Higher level
//! systems can use these types to communicate with each other. In practice all the IDs in this
//! module are wrappers around [`Uuid`](uuid::Uuid).


/// Create a wrapper type around [`Uuid`](uuid::Uuid)
///
/// ```
/// use lttcore::uuid_id;
///
/// uuid_id!(Foo);
/// assert!(Foo::new() != Foo::default());
/// ```
#[macro_export]
macro_rules! uuid_id {
    ($id:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialOrd,
            Ord,
            PartialEq,
            Eq,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct $id(::uuid::Uuid);

        impl $id {
            pub fn new() -> Self {
                Self(::uuid::Uuid::new_v4())
            }
        }

        impl From<$id> for ::uuid::Uuid {
            fn from($id(uuid): $id) -> Self {
                uuid
            }
        }

        impl From<::uuid::Uuid> for $id {
            fn from(uuid: ::uuid::Uuid) -> Self {
                Self(uuid)
            }
        }

        impl Default for $id {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

uuid_id!(UserId);
uuid_id!(GameId);
uuid_id!(SettingsId);
uuid_id!(ScenarioId);
