use anyhow::Result;
use pulumi_gestalt_providers_test::combine_string::CombineStringArgs;
use pulumi_gestalt_rust::{ConfigValue, Context, Output};
use pulumi_gestalt_rust::{GestaltContext, add_export, pulumi_combine, pulumi_format};
use pulumi_gestalt_rust::{GestaltOutput, ToOutput};

pulumi_gestalt_rust::pulumi_main!();

fn pulumi_main(context: &Context) -> Result<()> {
    let combine_string_res = pulumi_gestalt_providers_test::combine_string::create(
        context,
        "ABC",
        CombineStringArgs::builder().suffix("SUFFIX").build_struct(),
    );

    add_export("result", &combine_string_res.result);

    Ok(())
}
