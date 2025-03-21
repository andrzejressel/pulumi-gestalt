/// This certificate can be used to secure custom domains on App Services (Windows and Linux) hosted on an App Service Plan of Basic and above (free and shared tiers are not supported).
///
/// > NOTE: A certificate is valid for six months, and about a month before the certificate’s expiration date, App Services renews/rotates the certificate. This is managed by Azure and doesn't require this resource to be changed or reprovisioned. It will change the `thumbprint` computed attribute the next time the resource is refreshed after rotation occurs, so keep that in mind if you have any dependencies on this attribute directly.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: example-plan
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       kind: Linux
///       reserved: true
///       sku:
///         tier: Basic
///         size: B1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: example-app
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       appServicePlanId: ${examplePlan.id}
///   exampleTxtRecord:
///     type: azure:dns:TxtRecord
///     name: example
///     properties:
///       name: asuid.mycustomhost.contoso.com
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 300
///       records:
///         - value: ${exampleAppService.customDomainVerificationId}
///   exampleCNameRecord:
///     type: azure:dns:CNameRecord
///     name: example
///     properties:
///       name: example-adcr
///       zoneName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       ttl: 300
///       record: ${exampleAppService.defaultSiteHostname}
///   exampleCustomHostnameBinding:
///     type: azure:appservice:CustomHostnameBinding
///     name: example
///     properties:
///       hostname:
///         fn::invoke:
///           function: std:join
///           arguments:
///             separator: .
///             input:
///               - ${exampleCNameRecord.name}
///               - ${exampleCNameRecord.zoneName}
///           return: result
///       appServiceName: ${exampleAppService.name}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleManagedCertificate:
///     type: azure:appservice:ManagedCertificate
///     name: example
///     properties:
///       customHostnameBindingId: ${exampleCustomHostnameBinding.id}
///   exampleCertificateBinding:
///     type: azure:appservice:CertificateBinding
///     name: example
///     properties:
///       hostnameBindingId: ${exampleCustomHostnameBinding.id}
///       certificateId: ${exampleManagedCertificate.id}
///       sslState: SniEnabled
/// variables:
///   example:
///     fn::invoke:
///       function: azure:dns:getZone
///       arguments:
///         name: mydomain.com
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// App Service Managed Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/managedCertificate:ManagedCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Web/certificates/customhost.contoso.com
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedCertificateArgs {
        /// The ID of the App Service Custom Hostname Binding for the Certificate. Changing this forces a new App Service Managed Certificate to be created.
        #[builder(into)]
        pub custom_hostname_binding_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the App Service Managed Certificate.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedCertificateResult {
        /// The Canonical Name of the Certificate.
        pub canonical_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the App Service Custom Hostname Binding for the Certificate. Changing this forces a new App Service Managed Certificate to be created.
        pub custom_hostname_binding_id: pulumi_gestalt_rust::Output<String>,
        /// The expiration date of the Certificate.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The friendly name of the Certificate.
        pub friendly_name: pulumi_gestalt_rust::Output<String>,
        /// The list of Host Names for the Certificate.
        pub host_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Start date for the Certificate.
        pub issue_date: pulumi_gestalt_rust::Output<String>,
        /// The issuer of the Certificate.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// The Subject Name for the Certificate.
        pub subject_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the App Service Managed Certificate.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Thumbprint.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedCertificateArgs,
    ) -> ManagedCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_hostname_binding_id_binding = args
            .custom_hostname_binding_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/managedCertificate:ManagedCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customHostnameBindingId".into(),
                    value: &custom_hostname_binding_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedCertificateResult {
            canonical_name: o.get_field("canonicalName"),
            custom_hostname_binding_id: o.get_field("customHostnameBindingId"),
            expiration_date: o.get_field("expirationDate"),
            friendly_name: o.get_field("friendlyName"),
            host_names: o.get_field("hostNames"),
            issue_date: o.get_field("issueDate"),
            issuer: o.get_field("issuer"),
            subject_name: o.get_field("subjectName"),
            tags: o.get_field("tags"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
