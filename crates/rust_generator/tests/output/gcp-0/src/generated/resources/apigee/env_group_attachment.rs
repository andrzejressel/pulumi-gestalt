/// An `Environment Group attachment` in Apigee.
///
///
/// To get more information about EnvgroupAttachment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.envgroups.attachments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ## Import
///
/// EnvgroupAttachment can be imported using any of these accepted formats:
///
/// * `{{envgroup_id}}/attachments/{{name}}`
///
/// * `{{envgroup_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvgroupAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/attachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/{{name}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod env_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub envgroup_id: pulumi_gestalt_rust::Input<String>,
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        pub envgroup_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the environment.
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// The name of the newly created  attachment (output parameter).
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvGroupAttachmentArgs,
    ) -> EnvGroupAttachmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvGroupAttachmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EnvGroupAttachmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvGroupAttachmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EnvGroupAttachmentResult {
        let envgroup_id_binding = args.envgroup_id.get_output(ctx);
        let environment_binding = args.environment.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/envGroupAttachment:EnvGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envgroupId".into(),
                    value: &envgroup_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EnvGroupAttachmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            envgroup_id: o.get_field("envgroupId"),
            environment: o.get_field("environment"),
            name: o.get_field("name"),
        }
    }
}
