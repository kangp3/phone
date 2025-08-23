use std::collections::HashMap;
use std::sync::LazyLock;

use rsip::{Auth, Scheme, Uri};

use crate::sip::SERVER_ADDR;

pub static CONTACTS: LazyLock<HashMap<String, Uri>> = LazyLock::new(|| {
    ["1100", "1101", "1102", "1103"]
        .into_iter()
        .map(|uname| {
            (
                uname.to_string(),
                Uri {
                    scheme: Some(Scheme::Sip),
                    auth: Some(Auth {
                        user: uname.into(),
                        password: None,
                    }),
                    host_with_port: (*SERVER_ADDR).into(),
                    ..Default::default()
                },
            )
        })
        .collect()
});
