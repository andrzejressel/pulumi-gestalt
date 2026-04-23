/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:verifiedpermissions:Schema
///     properties:
///       policyStoreId: ${exampleAwsVerifiedpermissionsPolicyStore.policyStoreId}
///       definition:
///         - value:
///             fn::toJSON:
///               Namespace:
///                 entityTypes: {}
///                 actions: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store Schema using the `policy_store_id`. For example:
///
/// console
///
///  % pulumi import aws_verifiedpermissions_schema.example DxQg2j8xvXJQ1tQCYNWj9T
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The definition of the schema.
        #[builder(into, default)]
        pub definition: pulumi_gestalt_rust::Input<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The definition of the schema.
        pub definition: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedpermissions::SchemaDefinition>,
        >,
        /// (Optional) Identifies the namespaces of the entities referenced by this schema.
        pub namespaces: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SchemaResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SchemaResult {
        let definition_binding = args.definition.get_output(ctx);
        let policy_store_id_binding = args.policy_store_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SchemaResult {
            id: o.get_id(),
            urn: o.get_urn(),
            definition: o.get_field("definition"),
            namespaces: o.get_field("namespaces"),
            policy_store_id: o.get_field("policyStoreId"),
        }
    }
}
