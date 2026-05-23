use anyhow::Result;

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let res = pulumi_ref-ref::resource::create(&ctx, "res", pulumi_ref-ref::resource::ResourceArgs::builder().data(pulumi_gestalt_rust::pulumi_any!({"token": "ref-ref:index:Data", "properties": {"boolArray": [], "boolean": true, "float": 4.5, "innerData": {"token": "ref-ref:index:InnerData", "properties": {"boolArray": [false, true], "boolean": false, "float": 2.17, "integer": -12, "string": ("Goodbye"), "stringMap": {"three": ("french hens"), "two": ("turtle doves")}}}, "integer": 1024, "string": ("Hello"), "stringMap": {"x": ("100"), "y": ("200")}}})).build_struct());
    Ok(())
}