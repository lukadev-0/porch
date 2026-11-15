macro_rules! id_type {
    {
        $($pub:vis struct $name:ident($generate:path);)*
    } => {
        $(
            #[derive(
                Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
                ::serde::Serialize, ::serde::Deserialize,
                ::schemars::JsonSchema
            )]
            #[serde(transparent)]
            $pub struct $name(pub ::uuid::Uuid);

            impl $name {
                pub fn generate() -> Self {
                    Self($generate())
                }
            }

            impl From<::uuid::Uuid> for $name {
                fn from(value: ::uuid::Uuid) -> Self {
                    Self(value)
                }
            }

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        )*
    }
}

pub(crate) use id_type;
