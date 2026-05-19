use anyhow::Error;
use pulumi_gestalt_providers_random::random_bytes;
use pulumi_gestalt_rust::{Context, pulumi_combine};

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

fn pulumi_main(ctx: &Context) -> Result<(), Error> {
    let custom_secret = ctx.new_secret(&10);
    let non_secret = ctx.new_output(&1);

    let secret = random_bytes::create(
        ctx,
        "secret",
        random_bytes::RandomBytesArgs::builder()
            .length(custom_secret.clone())
            .build_struct(),
    );

    let secret_mapped = secret.base64.map(|_| "mapped_secret".to_string());
    let combined_with_secret = pulumi_combine!(non_secret, secret_mapped.clone())
        .map(|(a, b)| format!("[{}, \"{}\"]", a, b));

    ctx.add_export("secret_output", &secret.base64);
    ctx.add_export("secret_output_mapped", &secret_mapped);
    ctx.add_export("combined_with_secret", &combined_with_secret);
    ctx.add_export("custom_secret", &custom_secret);
    Ok(())
}
