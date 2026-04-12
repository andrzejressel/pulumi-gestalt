use anyhow::Result;
use pulumi_gestalt_providers_test::combine_string::CombineStringArgs;
use pulumi_gestalt_providers_test::provider::ProviderArgs;
use pulumi_gestalt_rust::{Context, CustomResourceOptions, add_export};

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &Context) -> Result<()> {
    let combine_string_res = pulumi_gestalt_providers_test::combine_string::create(
        ctx,
        "ABC",
        CombineStringArgs::builder().suffix("SUFFIX").build_struct(),
    );

    let provider = pulumi_gestalt_providers_test::provider::create(
        ctx,
        "test_provider",
        ProviderArgs::builder().prefix("MY_PREFIX").build_struct(),
    );

    let combine_string_with_provider =
        pulumi_gestalt_providers_test::combine_string::create_with_options(
            ctx,
            "DEF",
            CombineStringArgs::builder().suffix("SUFFIX").build_struct(),
            CustomResourceOptions::builder().provider(&provider).build(),
        );

    add_export("provider_prefix", &provider.prefix);
    add_export("result", &combine_string_res.result);
    add_export("result_with_provider", &combine_string_with_provider.result);
    add_export("provider_urn", &provider.urn);

    Ok(())
}
