// constants define constants that can be configured by a user of this integration test
pub(crate) mod constants;
pub(crate) mod service;
pub(crate) use constants::common as common_constants;
pub(crate) use constants::postgresql as postgresql_constants;
pub(crate) use constants::service as service_constants;
