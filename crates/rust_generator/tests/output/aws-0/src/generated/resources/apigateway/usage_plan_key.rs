/// Provides an API Gateway Usage Plan Key.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = usage_plan_key::create(
///         "main",
///         UsagePlanKeyArgs::builder()
///             .key_id("${mykey.id}")
///             .key_type("API_KEY")
///             .usage_plan_id("${myusageplan.id}")
///             .build_struct(),
///     );
///     let mykey = api_key::create(
///         "mykey",
///         ApiKeyArgs::builder().name("my_key").build_struct(),
///     );
///     let myusageplan = usage_plan::create(
///         "myusageplan",
///         UsagePlanArgs::builder()
///             .api_stages(
///                 vec![
///                     UsagePlanApiStage::builder().apiId("${test.id}")
///                     .stage("${foo.stageName}").build_struct(),
///                 ],
///             )
///             .name("my_usage_plan")
///             .build_struct(),
///     );
///     let test = rest_api::create(
///         "test",
///         RestApiArgs::builder().name("MyDemoAPI").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS API Gateway Usage Plan Key using the `USAGE-PLAN-ID/USAGE-PLAN-KEY-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/usagePlanKey:UsagePlanKey key 12345abcde/zzz
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod usage_plan_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsagePlanKeyArgs {
        /// Identifier of the API key resource.
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the API key resource. Currently, the valid key type is API_KEY.
        #[builder(into)]
        pub key_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the usage plan resource representing to associate the key to.
        #[builder(into)]
        pub usage_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UsagePlanKeyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the API key resource.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the API key resource. Currently, the valid key type is API_KEY.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// Name of a usage plan key.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Id of the usage plan resource representing to associate the key to.
        pub usage_plan_id: pulumi_gestalt_rust::Output<String>,
        /// Value of a usage plan key.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UsagePlanKeyArgs,
    ) -> UsagePlanKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_id_binding = args.key_id.get_output(context);
        let key_type_binding = args.key_type.get_output(context);
        let usage_plan_id_binding = args.usage_plan_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/usagePlanKey:UsagePlanKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyType".into(),
                    value: &key_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usagePlanId".into(),
                    value: &usage_plan_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UsagePlanKeyResult {
            id: o.get_field("id"),
            key_id: o.get_field("keyId"),
            key_type: o.get_field("keyType"),
            name: o.get_field("name"),
            usage_plan_id: o.get_field("usagePlanId"),
            value: o.get_field("value"),
        }
    }
}
