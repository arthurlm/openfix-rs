pub mod ze_corp {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/ZE_CORP_fields.rs"));
    }
    pub mod messages {
        // Keep header / trailer from FIX 4.4
        pub use openfix_messages::fix44::fields::*;
        pub use openfix_messages::fix44::messages::{MessageHeader, MessageTrailer};
        include!(concat!(env!("OUT_DIR"), "/ZE_CORP_messages.rs"));
    }
}

pub mod prelude {
    // Use common library fields
    pub use openfix_messages::prelude::*;
}
