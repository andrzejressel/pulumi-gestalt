use crate::Context;
use pulumi_gestalt_model::PulumiValue;

pub async fn finish_lambdas_sequentially<F: Fn(PulumiValue) -> PulumiValue + ?Sized>(
    ctx: &Context<Box<F>>,
) {
    loop {
        let result = ctx.finish().await;

        match result {
            None => break,
            Some(request) => {
                // Native callbacks currently round-trip through core's JSON mailbox, so callback
                // output metadata (secret/dependencies) is not propagated beyond the content.
                let response = (request.context)(request.data);
                request
                    .return_mailbox
                    .send(crate::engine::pulumi_value_to_json_value(response))
                    .unwrap();
            }
        }
    }
}
