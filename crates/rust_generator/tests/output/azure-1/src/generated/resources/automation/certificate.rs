/// Manages an Automation Certificate.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: account1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleCertificate:
///     type: azure:automation:Certificate
///     name: example
///     properties:
///       name: certificate1
///       resourceGroupName: ${example.name}
///       automationAccountName: ${exampleAccount.name}
///       description: This is an example certificate
///       base64:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.pfx
///           return: result
///       exportable: true
/// ```
///
/// ## Import
///
/// Automation Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/certificate:Certificate certificate1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/certificates/certificate1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::Input<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub base64: pulumi_gestalt_rust::Input<String>,
        /// The description of this Automation Certificate.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The is exportable flag of the certificate.
        #[builder(into, default)]
        pub exportable: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the automation account in which the Certificate is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// Base64 encoded value of the certificate. Changing this forces a new resource to be created.
        pub base64: pulumi_gestalt_rust::Output<String>,
        /// The description of this Automation Certificate.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The is exportable flag of the certificate.
        pub exportable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Certificate is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint for the certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CertificateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CertificateResult {
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(ctx);
        let base64_binding = args.base64.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let exportable_binding = args.exportable.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "base64".into(),
                    value: &base64_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exportable".into(),
                    value: &exportable_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CertificateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            automation_account_name: o.get_field("automationAccountName"),
            base64: o.get_field("base64"),
            description: o.get_field("description"),
            exportable: o.get_field("exportable"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
