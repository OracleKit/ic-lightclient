use serde::{de::DeserializeOwned, Serialize};

pub trait WireProtocol {
    type StatePayload: Serialize + DeserializeOwned + 'static;
    type UpdatePayload: Serialize + DeserializeOwned + 'static;
}
