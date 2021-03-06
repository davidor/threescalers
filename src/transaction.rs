use std::prelude::v1::*;

use super::{application::Application, usage::Usage, user::User, ToParams};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction<'a> {
    application: &'a Application,
    user: Option<&'a User>,
    usage: Option<&'a Usage<'a>>,
    timestamp: Option<String>,
}

impl<'a> Transaction<'a> {
    pub fn new(
        application: &'a Application,
        user: Option<&'a User>,
        usage: Option<&'a Usage>,
        timestamp: Option<i64>,
    ) -> Self {
        Self {
            application,
            user,
            usage,
            timestamp: timestamp.map(|tsi64| tsi64.to_string()),
        }
    }

    pub fn application(&self) -> &Application {
        self.application
    }

    pub fn user(&self) -> Option<&User> {
        self.user
    }

    pub fn usage(&self) -> Option<&Usage> {
        self.usage
    }

    pub fn timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Transaction<'_>
where
    'this: 'k + 'v,
    E: Extend<(Cow<'k, str>, &'v str)>,
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(
        &'this self,
        extendable: &mut E,
        key_mangling: &mut F,
    ) {
        if let Some(ts) = self.timestamp() {
            let field = key_mangling("timestamp".into());
            extendable.extend([(field, ts)].iter().cloned());
        }

        self.application
            .to_params_with_mangling(extendable, key_mangling);

        if let Some(user_params) = self.user {
            user_params.to_params_with_mangling(extendable, key_mangling);
        }

        if let Some(usage_params) = self.usage {
            usage_params.to_params_with_mangling(extendable, key_mangling);
        }
    }
}
