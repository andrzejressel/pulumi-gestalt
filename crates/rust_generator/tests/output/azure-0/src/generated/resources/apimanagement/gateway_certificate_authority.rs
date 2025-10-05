/// Manages an API Management Gateway Certificate Authority.
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
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       publisherName: pub1
///       publisherEmail: pub1@email.com
///       skuName: Consumption_0
///   exampleGateway:
///     type: azure:apimanagement:Gateway
///     name: example
///     properties:
///       name: example-gateway
///       apiManagementId: ${exampleService.id}
///       description: Example API Management gateway
///       locationData:
///         name: example name
///         city: example city
///         district: example district
///         region: example region
///   exampleCertificate:
///     type: azure:apimanagement:Certificate
///     name: example
///     properties:
///       name: example-cert
///       apiManagementName: ${exampleService.name}
///       resourceGroupName: ${example.name}
///       data:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: example.pfx
///           return: result
///   exampleGatewayCertificateAuthority:
///     type: azure:apimanagement:GatewayCertificateAuthority
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       certificateName: ${exampleCertificate.name}
///       gatewayName: ${exampleGateway.name}
///       isTrusted: true
/// ```
///
/// ## Import
///
/// API Management Gateway Certificate Authority can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/gatewayCertificateAuthority:GatewayCertificateAuthority example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/gateways/gateway1/certificateAuthorities/cert1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway_certificate_authority {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayCertificateAuthorityArgs {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub certificate_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the API Management Gateway Certificate Authority is trusted.
        #[builder(into, default)]
        pub is_trusted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GatewayCertificateAuthorityResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        pub certificate_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        pub gateway_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the API Management Gateway Certificate Authority is trusted.
        pub is_trusted: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GatewayCertificateAuthorityArgs,
    ) -> GatewayCertificateAuthorityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let certificate_name_binding = args.certificate_name.get_output(context);
        let gateway_name_binding = args.gateway_name.get_output(context);
        let is_trusted_binding = args.is_trusted.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/gatewayCertificateAuthority:GatewayCertificateAuthority"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isTrusted".into(),
                    value: &is_trusted_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayCertificateAuthorityResult {
            id: o.get_field("id"),
            api_management_id: o.get_field("apiManagementId"),
            certificate_name: o.get_field("certificateName"),
            gateway_name: o.get_field("gatewayName"),
            is_trusted: o.get_field("isTrusted"),
        }
    }
}
