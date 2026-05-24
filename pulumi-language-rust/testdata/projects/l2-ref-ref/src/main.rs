use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![pulumi_ref_ref::package()], pulumi_main)
        .unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let res = pulumi_ref_ref::resource::create(
        &ctx,
        "res",
        pulumi_ref_ref::resource::ResourceArgs::builder()
            .data(
                pulumi_ref_ref::types::Data::builder()
                    .bool_array(Vec::new())
                    .boolean(true)
                    .float(4.5)
                    .inner_data(
                        pulumi_ref_ref::types::InnerData::builder()
                            .bool_array(vec!(false, true))
                            .boolean(false)
                            .float(2.17)
                            .integer(-12)
                            .string("Goodbye")
                            .string_map(
                                std::collections::BTreeMap::from([
                                    (("three").to_string(), ("french hens").to_string()),
                                    (("two").to_string(), ("turtle doves").to_string()),
                                ]),
                            )
                            .build_struct(),
                    )
                    .integer(1024)
                    .string("Hello")
                    .string_map(
                        std::collections::BTreeMap::from([
                            (("x").to_string(), ("100").to_string()),
                            (("y").to_string(), ("200").to_string()),
                        ]),
                    )
                    .build_struct(),
            )
            .build_struct(),
    );
    Ok(())
}
