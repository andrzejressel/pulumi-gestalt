use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aNumber = ctx
        .require_config_deserialize::<f64>(None, "aNumber")
        .expect("Expected config [aNumber] to exist");
    ctx.add_export("theNumber", &(aNumber + 1.25));
    let optionalNumber = ctx
        .require_config_deserialize::<f64>(None, "optionalNumber")
        .unwrap_or(41.5);
    ctx.add_export("defaultNumber", &(optionalNumber + 1.2));
    let anInt = ctx
        .require_config_deserialize::<i64>(None, "anInt")
        .expect("Expected config [anInt] to exist");
    ctx.add_export("theInteger", &(anInt + 4));
    let optionalInt = ctx
        .require_config_deserialize::<i64>(None, "optionalInt")
        .unwrap_or(1);
    ctx.add_export("defaultInteger", &(optionalInt + 2));
    let aString = ctx
        .require_config(None, "aString")
        .expect("Expected config [aString] to exist");
    ctx.add_export("theString", &format!("{}{}", aString, " World"));
    let optionalString = ctx
        .require_config(None, "optionalString")
        .unwrap_or_else(|_| "defaultStringValue".to_string());
    ctx.add_export("defaultString", &optionalString);
    let aBool = ctx
        .require_config_deserialize::<bool>(None, "aBool")
        .expect("Expected config [aBool] to exist");
    ctx.add_export("theBool", &(!aBool && true));
    let optionalBool = ctx
        .require_config_deserialize::<bool>(None, "optionalBool")
        .unwrap_or(false);
    ctx.add_export("defaultBool", &optionalBool);
    Ok(())
}
