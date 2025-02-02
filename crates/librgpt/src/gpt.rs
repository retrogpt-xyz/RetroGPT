use std::borrow::Cow;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BackendQueryMsg<'a> {
    pub text: Cow<'a, str>,
    pub chatId: Option<i32>,
    pub sessionToken: Cow<'a, str>,
}
