use crate::Context;
use pulumi_gestalt_core::NativeFunctionRequest;
use pulumi_gestalt_model::PulumiValue;

pub async fn finish_lambdas_sequentially<F: Fn(PulumiValue) -> PulumiValue + ?Sized>(
    ctx: &Context<Box<F>>,
) {
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
