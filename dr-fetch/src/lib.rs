use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_messaging::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct DrFetchActor {}

#[async_trait]
impl MessageSubscriber for DrFetchActor {
    /// handle subscription response
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        

        MessagingSender::new()
        .publish(
            ctx,
            &PubMessage {
                body: msg.body.clone(),
                reply_to: None,
                subject: "app.drwrite.fetch".to_string(),
            },
        )
        .await?;

        Ok(())
    }
}
