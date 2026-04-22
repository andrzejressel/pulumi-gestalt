/// Manages a Federated Identity Credential.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleFederatedIdentityCredential = federated_identity_credential::create(
///         "exampleFederatedIdentityCredential",
///         FederatedIdentityCredentialArgs::builder()
///             .audience("foo")
///             .issuer("https://foo")
///             .name("example")
///             .parent_id("${exampleUserAssignedIdentity.id}")
///             .resource_group_name("${example.name}")
///             .subject("foo")
///             .build_struct(),
///     );
///     let exampleUserAssignedIdentity = user_assigned_identity::create(
///         "exampleUserAssignedIdentity",
///         UserAssignedIdentityArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Federated Identity Credential can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:armmsi/federatedIdentityCredential:FederatedIdentityCredential example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{parentIdentityName}/federatedIdentityCredentials/{resourceName}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod federated_identity_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialArgs {
        /// Specifies the audience for this Federated Identity Credential.
        #[builder(into)]
        pub audience: pulumi_gestalt_rust::Input<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        #[builder(into)]
        pub issuer: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Specifies the subject for this Federated Identity Credential.
        #[builder(into)]
        pub subject: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the audience for this Federated Identity Credential.
        pub audience: pulumi_gestalt_rust::Output<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        pub parent_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the subject for this Federated Identity Credential.
        pub subject: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FederatedIdentityCredentialArgs,
    ) -> FederatedIdentityCredentialResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FederatedIdentityCredentialArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FederatedIdentityCredentialResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FederatedIdentityCredentialArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FederatedIdentityCredentialResult {
        let audience_binding = args.audience.get_output(ctx);
        let issuer_binding = args.issuer.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let parent_id_binding = args.parent_id.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let subject_binding = args.subject.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:armmsi/federatedIdentityCredential:FederatedIdentityCredential"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "audience".into(),
                    value: &audience_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "issuer".into(),
                    value: &issuer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subject".into(),
                    value: &subject_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FederatedIdentityCredentialResult {
            id: o.get_id(),
            urn: o.get_urn(),
            audience: o.get_field("audience"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            parent_id: o.get_field("parentId"),
            resource_group_name: o.get_field("resourceGroupName"),
            subject: o.get_field("subject"),
        }
    }
}
