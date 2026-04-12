/// Resource for managing an AWS EventBridge Schemas Registry Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleRegistryPolicy:
///     type: aws:schemas:RegistryPolicy
///     name: example
///     properties:
///       registryName: example
///       policy: ${example.json}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: example
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '109876543210'
///             actions:
///               - schemas:*
///             resources:
///               - arn:aws:schemas:us-east-1:123456789012:registry/example
///               - arn:aws:schemas:us-east-1:123456789012:schema/example*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge Schema Registry Policy using the `registry_name`. For example:
///
/// ```sh
/// $ pulumi import aws:schemas/registryPolicy:RegistryPolicy example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod registry_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryPolicyArgs {
        /// Resource Policy for EventBridge Schema Registry
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of EventBridge Schema Registry
        #[builder(into)]
        pub registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Resource Policy for EventBridge Schema Registry
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// Name of EventBridge Schema Registry
        pub registry_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryPolicyArgs,
    ) -> RegistryPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RegistryPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RegistryPolicyResult {
        let policy_binding = args.policy.get_output(ctx);
        let registry_name_binding = args.registry_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:schemas/registryPolicy:RegistryPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryName".into(),
                    value: &registry_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RegistryPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy: o.get_field("policy"),
            registry_name: o.get_field("registryName"),
        }
    }
}
