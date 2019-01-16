use crate::ToParams;
use crate::credentials::{Credentials, ServiceId};

#[derive(Debug)]
pub struct Service {
    service_id: ServiceId,
    creds: Credentials,
}

impl Service {
    /// Creates a new `Service` from a `ServiceId` and `Credentials`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::credentials::*;
    /// use threescalers::service::*;
    ///
    /// let creds = Credentials::from_token("my_token");
    /// let service = Service::new("my_service_id", creds);
    /// ```
    pub fn new<T: Into<ServiceId>>(service_id: T, creds: Credentials) -> Self {
        Self { service_id: service_id.into(), creds }
    }
}

impl<'k, 'v, E> ToParams<'k, 'v, E> for Service where E: Extend<(&'k str, &'v str)> {
    fn to_params<'s: 'k + 'v>(&'s self, extendable: &mut E) {
        extendable.extend([("service_id", self.service_id.as_ref())].iter().cloned());
        self.creds.to_params(extendable);
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;

    #[test]
    fn transforms_service_id_and_key_into_params() {
        let service_id = "my_service_id";
        let provider_key = "my_provider_key";
        let creds = Credentials::from_key(provider_key);
        let service = Service::new(service_id, creds);

        let mut result = Vec::new();
        service.to_params(&mut result);

        let expected = vec![("service_id", service_id),
                            ("provider_key", provider_key)];
        assert_eq!(expected, result);
    }

    #[test]
    fn transforms_service_id_and_token_into_params() {
        let service_id = "my_service_id";
        let token = "my_token";
        let creds = Credentials::from_token(token);
        let service = Service::new(service_id, creds);

        let mut result = Vec::new();
        service.to_params(&mut result);

        let expected = vec![("service_id", service_id),
                            ("service_token", token)];
        assert_eq!(expected, result);
    }
}
