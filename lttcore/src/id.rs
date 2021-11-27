/// Create Uuid `NewType` wrappers with all the fixin's
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
