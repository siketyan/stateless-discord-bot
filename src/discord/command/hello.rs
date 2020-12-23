use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn hello() -> InteractionResponse {
    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: "Hello, world!".to_string(),
        }),
    }
}
