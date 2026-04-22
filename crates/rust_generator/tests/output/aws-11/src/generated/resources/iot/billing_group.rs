/// Manages an AWS IoT Billing Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:BillingGroup
///     properties:
///       name: example
///       properties:
///         description: This is my billing group
///       tags:
///         terraform: 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT Billing Groups using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/billingGroup:BillingGroup example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod billing_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BillingGroupArgs {
        /// The name of the Billing Group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The Billing Group properties. Defined below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::Input<
            Option<super::super::types::iot::BillingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct BillingGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Billing Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::iot::BillingGroupMetadata>,
        >,
        /// The name of the Billing Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Billing Group properties. Defined below.
        pub properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::BillingGroupProperties>,
        >,
        /// Key-value mapping of resource tags
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the Billing Group record in the registry.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BillingGroupArgs,
    ) -> BillingGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BillingGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> BillingGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BillingGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> BillingGroupResult {
        let name_binding = args.name.get_output(ctx);
        let properties_binding = args.properties.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/billingGroup:BillingGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        BillingGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            metadatas: o.get_field("metadatas"),
            name: o.get_field("name"),
            properties: o.get_field("properties"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
