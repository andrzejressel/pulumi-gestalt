use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("empty", &pulumi_gestalt_rust::pulumi_any!({}));
    ctx.add_export(
        "strings",
        &pulumi_gestalt_rust::pulumi_any!(
            { "farewell" : ("Goodbye, world!"), "greeting" : ("Hello, world!") }
        ),
    );
    ctx.add_export("numbers", &pulumi_gestalt_rust::pulumi_any!({ "1" : 1, "2" : 2 }));
    ctx.add_export(
        "keys",
        &pulumi_gestalt_rust::pulumi_any!(
            { "MYKEY" : 6, "MY_KEY" : 4, "my-key" : 2, "my.key" : 1, "my_key" : 3,
            "mykey" : 5 }
        ),
    );
    Ok(())
}
