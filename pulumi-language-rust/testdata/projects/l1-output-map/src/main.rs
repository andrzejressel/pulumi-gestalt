use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("empty", &pulumi_gestalt_rust::pulumi_any_v2!({}));
    ctx.add_export(
        "strings",
        &pulumi_gestalt_rust::pulumi_any_v2!(
            { "farewell" : ("Goodbye, world!"), "greeting" : ("Hello, world!") }
        ),
    );
    ctx.add_export(
        "adversarialStrings",
        &pulumi_gestalt_rust::pulumi_any_v2!(
            { "" : ("empty key"),
            "Some ${common} \"characters\" 'that' need escaping: \\ (backslash), \t (tab), \u{1b} (escape), \u{7} (bell), \0 (null), \u{e0021} (tag space)"
            :
            ("Some ${common} \"characters\" 'that' need escaping: \\ (backslash), \t (tab), \u{1b} (escape), \u{7} (bell), \0 (null), \u{e0021} (tag space)"),
            "__internal" : ("dunder internal"), "__provider" : ("dunder provider"),
            "__type" : ("dunder type"), "__version" : ("dunder version"), "dunder value"
            : ("__dunder"), "empty value" : ("") }
        ),
    );
    ctx.add_export(
        "numbers",
        &pulumi_gestalt_rust::pulumi_any_v2!({ "1" : 1, "2" : 2 }),
    );
    ctx.add_export(
        "keys",
        &pulumi_gestalt_rust::pulumi_any_v2!(
            { "MYKEY" : 6, "MY_KEY" : 4, "my-key" : 2, "my.key" : 1, "my_key" : 3,
            "mykey" : 5 }
        ),
    );
    Ok(())
}
