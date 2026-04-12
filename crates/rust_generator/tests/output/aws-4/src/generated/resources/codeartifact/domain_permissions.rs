/// Provides a CodeArtifact Domains Permissions Policy Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: domain key
///   exampleDomain:
///     type: aws:codeartifact:Domain
///     name: example
///     properties:
///       domain: example
///       encryptionKey: ${example.arn}
///   testDomainPermissions:
///     type: aws:codeartifact:DomainPermissions
///     name: test
///     properties:
///       domain: ${exampleDomain.domain}
///       policyDocument: ${test.json}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - codeartifact:CreateRepository
///             resources:
///               - ${exampleDomain.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Domain Permissions Policies using the CodeArtifact Domain ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/domainPermissions:DomainPermissions example arn:aws:codeartifact:us-west-2:012345678912:domain/tf-acc-test-1928056699409417367
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod domain_permissions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainPermissionsArgs {
        /// The name of the domain on which to set the resource policy.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        #[builder(into)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        #[builder(into, default)]
        pub policy_revision: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DomainPermissionsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the domain on which to set the resource policy.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_gestalt_rust::Output<String>,
        /// A JSON policy string to be set as the access control resource policy on the provided domain.
        pub policy_document: pulumi_gestalt_rust::Output<String>,
        /// The current revision of the resource policy to be set. This revision is used for optimistic locking, which prevents others from overwriting your changes to the domain's resource policy.
        pub policy_revision: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the resource associated with the resource policy.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainPermissionsArgs,
    ) -> DomainPermissionsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainPermissionsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DomainPermissionsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainPermissionsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DomainPermissionsResult {
        let domain_binding = args.domain.get_output(ctx);
        let domain_owner_binding = args.domain_owner.get_output(ctx);
        let policy_document_binding = args.policy_document.get_output(ctx);
        let policy_revision_binding = args.policy_revision.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codeartifact/domainPermissions:DomainPermissions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyRevision".into(),
                    value: &policy_revision_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DomainPermissionsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            domain: o.get_field("domain"),
            domain_owner: o.get_field("domainOwner"),
            policy_document: o.get_field("policyDocument"),
            policy_revision: o.get_field("policyRevision"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}
