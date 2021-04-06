pub mod ze_corp {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/ZE_CORP_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/ZE_CORP_messages.rs"));
    }
}

pub mod prelude {
    pub use quickfix_messages::prelude::*;
}
