/// The provider type for the azurerm package. By default, resources use package-wide configuration
/// settings, however an explicit `Provider` instance may be created and passed during resource
/// construction to achieve fine-grained programmatic control over provider settings. See the
/// [documentation](https://www.pulumi.com/docs/reference/programming-model/#providers) for more information.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[derive(pulumi_gestalt_rust::__private::bon::Builder)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderArgs {
    #[builder(into, default)]
    pub auxiliary_tenant_ids: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    /// Base64 encoded PKCS#12 certificate bundle to use when authenticating as a Service Principal using a Client Certificate
    #[builder(into, default)]
    pub client_certificate: pulumi_gestalt_rust::Input<Option<String>>,
    /// The password associated with the Client Certificate. For use when authenticating as a Service Principal using a Client
    /// Certificate
    #[builder(into, default)]
    pub client_certificate_password: pulumi_gestalt_rust::Input<Option<String>>,
    /// The path to the Client Certificate associated with the Service Principal for use when authenticating as a Service
    /// Principal using a Client Certificate.
    #[builder(into, default)]
    pub client_certificate_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// The Client ID which should be used.
    #[builder(into, default)]
    pub client_id: pulumi_gestalt_rust::Input<Option<String>>,
    /// The path to a file containing the Client ID which should be used.
    #[builder(into, default)]
    pub client_id_file_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// The Client Secret which should be used. For use When authenticating as a Service Principal using a Client Secret.
    #[builder(into, default)]
    pub client_secret: pulumi_gestalt_rust::Input<Option<String>>,
    /// The path to a file containing the Client Secret which should be used. For use When authenticating as a Service Principal
    /// using a Client Secret.
    #[builder(into, default)]
    pub client_secret_file_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// This will disable the x-ms-correlation-request-id header.
    #[builder(into, default)]
    pub disable_correlation_request_id: pulumi_gestalt_rust::Input<Option<bool>>,
    #[builder(into, default)]
    pub disable_terraform_partner_id: pulumi_gestalt_rust::Input<Option<bool>>,
    /// The Cloud Environment which should be used. Possible values are public, usgovernment, and china. Defaults to public. Not
    /// used and should not be specified when `metadata_host` is specified.
    #[builder(into, default)]
    pub environment: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub features: pulumi_gestalt_rust::Input<Option<super::types::ProviderFeatures>>,
    /// The Hostname which should be used for the Azure Metadata Service.
    #[builder(into, default)]
    pub metadata_host: pulumi_gestalt_rust::Input<Option<String>>,
    /// The path to a custom endpoint for Managed Service Identity - in most circumstances this should be detected
    /// automatically.
    #[builder(into, default)]
    pub msi_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    /// The bearer token for the request to the OIDC provider. For use when authenticating as a Service Principal using OpenID
    /// Connect.
    #[builder(into, default)]
    pub oidc_request_token: pulumi_gestalt_rust::Input<Option<String>>,
    /// The URL for the OIDC provider from which to request an ID token. For use when authenticating as a Service Principal
    /// using OpenID Connect.
    #[builder(into, default)]
    pub oidc_request_url: pulumi_gestalt_rust::Input<Option<String>>,
    /// The OIDC ID token for use when authenticating as a Service Principal using OpenID Connect.
    #[builder(into, default)]
    pub oidc_token: pulumi_gestalt_rust::Input<Option<String>>,
    /// The path to a file containing an OIDC ID token for use when authenticating as a Service Principal using OpenID Connect.
    #[builder(into, default)]
    pub oidc_token_file_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// A GUID/UUID that is registered with Microsoft to facilitate partner resource usage attribution.
    #[builder(into, default)]
    pub partner_id: pulumi_gestalt_rust::Input<Option<String>>,
    /// The set of Resource Providers which should be automatically registered for the subscription.
    #[builder(into, default)]
    pub resource_provider_registrations: pulumi_gestalt_rust::Input<Option<String>>,
    /// A list of Resource Providers to explicitly register for the subscription, in addition to those specified by the
    /// `resource_provider_registrations` property.
    #[builder(into, default)]
    pub resource_providers_to_registers: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    /// Should the AzureRM Provider skip registering all of the Resource Providers that it supports, if they're not already
    /// registered?
    #[builder(into, default)]
    pub skip_provider_registration: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Should the AzureRM Provider use Azure AD Authentication when accessing the Storage Data Plane APIs?
    #[builder(into, default)]
    pub storage_use_azuread: pulumi_gestalt_rust::Input<Option<bool>>,
    /// The Subscription ID which should be used.
    #[builder(into, default)]
    pub subscription_id: pulumi_gestalt_rust::Input<Option<String>>,
    /// The Tenant ID which should be used.
    #[builder(into, default)]
    pub tenant_id: pulumi_gestalt_rust::Input<Option<String>>,
    /// Allow Azure AKS Workload Identity to be used for Authentication.
    #[builder(into, default)]
    pub use_aks_workload_identity: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Allow Azure CLI to be used for Authentication.
    #[builder(into, default)]
    pub use_cli: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Allow Managed Service Identity to be used for Authentication.
    #[builder(into, default)]
    pub use_msi: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Allow OpenID Connect to be used for authentication
    #[builder(into, default)]
    pub use_oidc: pulumi_gestalt_rust::Input<Option<bool>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    pub auxiliary_tenant_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    /// Base64 encoded PKCS#12 certificate bundle to use when authenticating as a Service Principal using a Client Certificate
    pub client_certificate: pulumi_gestalt_rust::Output<Option<String>>,
    /// The password associated with the Client Certificate. For use when authenticating as a Service Principal using a Client
    /// Certificate
    pub client_certificate_password: pulumi_gestalt_rust::Output<Option<String>>,
    /// The path to the Client Certificate associated with the Service Principal for use when authenticating as a Service
    /// Principal using a Client Certificate.
    pub client_certificate_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// The Client ID which should be used.
    pub client_id: pulumi_gestalt_rust::Output<Option<String>>,
    /// The path to a file containing the Client ID which should be used.
    pub client_id_file_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// The Client Secret which should be used. For use When authenticating as a Service Principal using a Client Secret.
    pub client_secret: pulumi_gestalt_rust::Output<Option<String>>,
    /// The path to a file containing the Client Secret which should be used. For use When authenticating as a Service Principal
    /// using a Client Secret.
    pub client_secret_file_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// This will disable the x-ms-correlation-request-id header.
    pub disable_correlation_request_id: pulumi_gestalt_rust::Output<Option<bool>>,
    pub disable_terraform_partner_id: pulumi_gestalt_rust::Output<Option<bool>>,
    /// The Cloud Environment which should be used. Possible values are public, usgovernment, and china. Defaults to public. Not
    /// used and should not be specified when `metadata_host` is specified.
    pub environment: pulumi_gestalt_rust::Output<Option<String>>,
    pub features: pulumi_gestalt_rust::Output<Option<super::types::ProviderFeatures>>,
    /// The Hostname which should be used for the Azure Metadata Service.
    pub metadata_host: pulumi_gestalt_rust::Output<Option<String>>,
    /// The path to a custom endpoint for Managed Service Identity - in most circumstances this should be detected
    /// automatically.
    pub msi_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    /// The bearer token for the request to the OIDC provider. For use when authenticating as a Service Principal using OpenID
    /// Connect.
    pub oidc_request_token: pulumi_gestalt_rust::Output<Option<String>>,
    /// The URL for the OIDC provider from which to request an ID token. For use when authenticating as a Service Principal
    /// using OpenID Connect.
    pub oidc_request_url: pulumi_gestalt_rust::Output<Option<String>>,
    /// The OIDC ID token for use when authenticating as a Service Principal using OpenID Connect.
    pub oidc_token: pulumi_gestalt_rust::Output<Option<String>>,
    /// The path to a file containing an OIDC ID token for use when authenticating as a Service Principal using OpenID Connect.
    pub oidc_token_file_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// A GUID/UUID that is registered with Microsoft to facilitate partner resource usage attribution.
    pub partner_id: pulumi_gestalt_rust::Output<Option<String>>,
    /// The set of Resource Providers which should be automatically registered for the subscription.
    pub resource_provider_registrations: pulumi_gestalt_rust::Output<Option<String>>,
    /// A list of Resource Providers to explicitly register for the subscription, in addition to those specified by the
    /// `resource_provider_registrations` property.
    pub resource_providers_to_registers: pulumi_gestalt_rust::Output<
        Option<Vec<String>>,
    >,
    /// Should the AzureRM Provider skip registering all of the Resource Providers that it supports, if they're not already
    /// registered?
    pub skip_provider_registration: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Should the AzureRM Provider use Azure AD Authentication when accessing the Storage Data Plane APIs?
    pub storage_use_azuread: pulumi_gestalt_rust::Output<Option<bool>>,
    /// The Subscription ID which should be used.
    pub subscription_id: pulumi_gestalt_rust::Output<Option<String>>,
    /// The Tenant ID which should be used.
    pub tenant_id: pulumi_gestalt_rust::Output<Option<String>>,
    /// Allow Azure AKS Workload Identity to be used for Authentication.
    pub use_aks_workload_identity: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Allow Azure CLI to be used for Authentication.
    pub use_cli: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Allow Managed Service Identity to be used for Authentication.
    pub use_msi: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Allow OpenID Connect to be used for authentication
    pub use_oidc: pulumi_gestalt_rust::Output<Option<bool>>,
}
impl pulumi_gestalt_rust::Provider for ProviderResult {
    fn get_provider_id(&self) -> pulumi_gestalt_rust::Output<String> {
        self.provider_id.clone()
    }
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
) -> ProviderResult {
    create_with_options(context, name, args, None)
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create_with_options(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
    options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
) -> ProviderResult {
    let auxiliary_tenant_ids_binding = args.auxiliary_tenant_ids.get_output(context);
    let client_certificate_binding = args.client_certificate.get_output(context);
    let client_certificate_password_binding = args
        .client_certificate_password
        .get_output(context);
    let client_certificate_path_binding = args
        .client_certificate_path
        .get_output(context);
    let client_id_binding = args.client_id.get_output(context);
    let client_id_file_path_binding = args.client_id_file_path.get_output(context);
    let client_secret_binding = args.client_secret.get_output(context);
    let client_secret_file_path_binding = args
        .client_secret_file_path
        .get_output(context);
    let disable_correlation_request_id_binding = args
        .disable_correlation_request_id
        .get_output(context);
    let disable_terraform_partner_id_binding = args
        .disable_terraform_partner_id
        .get_output(context);
    let environment_binding = args.environment.get_output(context);
    let features_binding = args.features.get_output(context);
    let metadata_host_binding = args.metadata_host.get_output(context);
    let msi_endpoint_binding = args.msi_endpoint.get_output(context);
    let oidc_request_token_binding = args.oidc_request_token.get_output(context);
    let oidc_request_url_binding = args.oidc_request_url.get_output(context);
    let oidc_token_binding = args.oidc_token.get_output(context);
    let oidc_token_file_path_binding = args.oidc_token_file_path.get_output(context);
    let partner_id_binding = args.partner_id.get_output(context);
    let resource_provider_registrations_binding = args
        .resource_provider_registrations
        .get_output(context);
    let resource_providers_to_registers_binding = args
        .resource_providers_to_registers
        .get_output(context);
    let skip_provider_registration_binding = args
        .skip_provider_registration
        .get_output(context);
    let storage_use_azuread_binding = args.storage_use_azuread.get_output(context);
    let subscription_id_binding = args.subscription_id.get_output(context);
    let tenant_id_binding = args.tenant_id.get_output(context);
    let use_aks_workload_identity_binding = args
        .use_aks_workload_identity
        .get_output(context);
    let use_cli_binding = args.use_cli.get_output(context);
    let use_msi_binding = args.use_msi.get_output(context);
    let use_oidc_binding = args.use_oidc.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:azure".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "auxiliaryTenantIds".into(),
                value: &auxiliary_tenant_ids_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientCertificate".into(),
                value: &client_certificate_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientCertificatePassword".into(),
                value: &client_certificate_password_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientCertificatePath".into(),
                value: &client_certificate_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientId".into(),
                value: &client_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientIdFilePath".into(),
                value: &client_id_file_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientSecret".into(),
                value: &client_secret_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clientSecretFilePath".into(),
                value: &client_secret_file_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "disableCorrelationRequestId".into(),
                value: &disable_correlation_request_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "disableTerraformPartnerId".into(),
                value: &disable_terraform_partner_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "environment".into(),
                value: &environment_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "features".into(),
                value: &features_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "metadataHost".into(),
                value: &metadata_host_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "msiEndpoint".into(),
                value: &msi_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "oidcRequestToken".into(),
                value: &oidc_request_token_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "oidcRequestUrl".into(),
                value: &oidc_request_url_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "oidcToken".into(),
                value: &oidc_token_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "oidcTokenFilePath".into(),
                value: &oidc_token_file_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "partnerId".into(),
                value: &partner_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "resourceProviderRegistrations".into(),
                value: &resource_provider_registrations_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "resourceProvidersToRegisters".into(),
                value: &resource_providers_to_registers_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "skipProviderRegistration".into(),
                value: &skip_provider_registration_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "storageUseAzuread".into(),
                value: &storage_use_azuread_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "subscriptionId".into(),
                value: &subscription_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tenantId".into(),
                value: &tenant_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useAksWorkloadIdentity".into(),
                value: &use_aks_workload_identity_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useCli".into(),
                value: &use_cli_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useMsi".into(),
                value: &use_msi_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useOidc".into(),
                value: &use_oidc_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        auxiliary_tenant_ids: o.get_field("auxiliaryTenantIds"),
        client_certificate: o.get_field("clientCertificate"),
        client_certificate_password: o.get_field("clientCertificatePassword"),
        client_certificate_path: o.get_field("clientCertificatePath"),
        client_id: o.get_field("clientId"),
        client_id_file_path: o.get_field("clientIdFilePath"),
        client_secret: o.get_field("clientSecret"),
        client_secret_file_path: o.get_field("clientSecretFilePath"),
        disable_correlation_request_id: o.get_field("disableCorrelationRequestId"),
        disable_terraform_partner_id: o.get_field("disableTerraformPartnerId"),
        environment: o.get_field("environment"),
        features: o.get_field("features"),
        metadata_host: o.get_field("metadataHost"),
        msi_endpoint: o.get_field("msiEndpoint"),
        oidc_request_token: o.get_field("oidcRequestToken"),
        oidc_request_url: o.get_field("oidcRequestUrl"),
        oidc_token: o.get_field("oidcToken"),
        oidc_token_file_path: o.get_field("oidcTokenFilePath"),
        partner_id: o.get_field("partnerId"),
        resource_provider_registrations: o.get_field("resourceProviderRegistrations"),
        resource_providers_to_registers: o.get_field("resourceProvidersToRegisters"),
        skip_provider_registration: o.get_field("skipProviderRegistration"),
        storage_use_azuread: o.get_field("storageUseAzuread"),
        subscription_id: o.get_field("subscriptionId"),
        tenant_id: o.get_field("tenantId"),
        use_aks_workload_identity: o.get_field("useAksWorkloadIdentity"),
        use_cli: o.get_field("useCli"),
        use_msi: o.get_field("useMsi"),
        use_oidc: o.get_field("useOidc"),
    }
}
