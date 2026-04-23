/// Provides a resource to manage whether default EBS encryption is enabled for your AWS account in the current AWS region. To manage the default KMS key for the region, see the `aws.ebs.DefaultKmsKey` resource.
///
/// > **NOTE:** Removing this resource disables default EBS encryption.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = encryption_by_default::create(
///         "example",
///         EncryptionByDefaultArgs::builder().enabled(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the default EBS encryption state. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/encryptionByDefault:EncryptionByDefault example default
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod encryption_by_default {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EncryptionByDefaultArgs {
        /// Whether or not default EBS encryption is enabled. Valid values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::Input<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EncryptionByDefaultResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether or not default EBS encryption is enabled. Valid values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EncryptionByDefaultArgs,
    ) -> EncryptionByDefaultResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EncryptionByDefaultArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EncryptionByDefaultResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EncryptionByDefaultArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EncryptionByDefaultResult {
        let enabled_binding = args.enabled.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/encryptionByDefault:EncryptionByDefault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EncryptionByDefaultResult {
            id: o.get_id(),
            urn: o.get_urn(),
            enabled: o.get_field("enabled"),
        }
    }
}
