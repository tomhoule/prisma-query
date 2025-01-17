#[cfg(feature = "uuid-0_7")]
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Id {
    String(String),
    Int(usize),
    #[cfg(feature = "uuid-0_7")]
    UUID(Uuid),
}

impl From<usize> for Id {
    fn from(u: usize) -> Self {
        Id::Int(u)
    }
}

impl From<u64> for Id {
    fn from(u: u64) -> Self {
        Id::Int(u as usize)
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

#[cfg(feature = "uuid-0_7")]
impl From<Uuid> for Id {
    fn from(u: Uuid) -> Self {
        Id::UUID(u)
    }
}
