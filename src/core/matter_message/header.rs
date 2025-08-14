use crate::constants::MATTER_MESSAGE_EXTENSION_BYTES;

pub struct MatterMessageHeader {
    pub message_extensions: Option<[u8; MATTER_MESSAGE_EXTENSION_BYTES]>, // TODO: change in the future, when it actually starts being used.
}
