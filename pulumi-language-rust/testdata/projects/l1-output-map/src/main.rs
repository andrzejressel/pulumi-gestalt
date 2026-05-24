use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![], pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("empty", &pulumi_gestalt_rust::pulumi_any!({}));
    ctx.add_export(
        "strings",
        &pulumi_gestalt_rust::pulumi_any!(
            { "farewell" : (("Goodbye, world!").clone()), "greeting" : (("Hello, world!")
            .clone()) }
        ),
    );
    ctx.add_export(
        "adversarialStrings",
        &pulumi_gestalt_rust::pulumi_any!(
            { "" : (("empty key").clone()),
            "Some ${common} \"characters\" 'that' need escaping: \\ (backslash), \t (tab), \u{1b} (escape), \u{7} (bell), \0 (null), \u{e0021} (tag space)"
            :
            (("Some ${common} \"characters\" 'that' need escaping: \\ (backslash), \t (tab), \u{1b} (escape), \u{7} (bell), \0 (null), \u{e0021} (tag space)")
            .clone()), "__internal" : (("dunder internal").clone()), "__provider" :
            (("dunder provider").clone()), "__type" : (("dunder type").clone()),
            "__version" : (("dunder version").clone()), "dunder value" : (("__dunder")
            .clone()), "empty value" : (("").clone()) }
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
