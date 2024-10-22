use super::WebsocketHandlingError;
use crate::handlers::{errors::ParseVariableError, utils::get_channel_addr};
use artnet::ArtNetClient;
use config::{Binding, Fixture};
use std::{collections::HashMap, sync::Arc};

pub fn handle(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let binding = bindings
        .get(msg)
        .ok_or_else(|| WebsocketHandlingError::UnknownMessage(msg.to_string()))?;

    for action in binding.get_actions() {
        let channel_addr = get_channel_addr(&action[0], &fixtures)?;
        let value: u8 = action[1]
            .parse()
            .map_err(|e| WebsocketHandlingError::VariableParseError(ParseVariableError::from(e)))?;
        client.set_single(channel_addr, value);
    }

    client.commit().map_err(WebsocketHandlingError::IoError)
}