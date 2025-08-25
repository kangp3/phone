use std::collections::HashMap;
use std::sync::LazyLock;

use rsip::typed::To;
use rsip::{Auth, Scheme, Uri};

use crate::sip::{SERVER_NAME, SERVER_PORT};

pub static CONTACTS: LazyLock<HashMap<String, To>> = LazyLock::new(|| {
    ["1100", "1101", "1102", "1103"]
        .into_iter()
        .map(|uname| {
            (
                uname.to_string(),
                To {
                    display_name: Some(uname.into()),
                    uri: Uri {
                        scheme: Some(Scheme::Sips),
                        auth: Some(Auth {
                            user: uname.into(),
                            password: None,
                        }),
                        host_with_port: (SERVER_NAME, SERVER_PORT).into(),
                        ..Default::default()
                    },
                    params: vec![],
                },
            )
        })
        .collect()
});
