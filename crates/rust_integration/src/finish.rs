use crate::Context;
use pulumi_gestalt_core::NativeFunctionRequest;
use serde_json::Value;

pub async fn finish_lambdas_sequentially<F: Fn(Value) -> Value + ?Sized>(ctx: &Context<Box<F>>) {
    loop {
        let result = ctx.finish().await;

        match result {
            None => break,
            Some(NativeFunctionRequest {
                context,
                data,
                return_mailbox,
            }) => {
                return_mailbox.send(context(data)).unwrap();
            }
        }
    }
}
