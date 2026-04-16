use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("empty", &pulumi_gestalt_rust::pulumi_any!({}));
    ctx.add_export(
        "strings",
        &pulumi_gestalt_rust::pulumi_any!(
            { "greeting" : ("Hello, world!"), "farewell" : ("Goodbye, world!") }
        ),
    );
    ctx.add_export("numbers", &pulumi_gestalt_rust::pulumi_any!({ "2" : 2, "1" : 1 }));
    ctx.add_export(
        "keys",
        &pulumi_gestalt_rust::pulumi_any!(
            { "MY_KEY" : 4, "mykey" : 5, "my_key" : 3, "my-key" : 2, "my.key" : 1,
            "MYKEY" : 6 }
        ),
    );
    Ok(())
}
