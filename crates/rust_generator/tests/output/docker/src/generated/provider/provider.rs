/// The provider type for the docker package. By default, resources use package-wide configuration
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
    /// PEM-encoded content of Docker host CA certificate
    #[builder(into, default)]
    pub ca_material: pulumi_gestalt_rust::Input<Option<String>>,
    /// PEM-encoded content of Docker client certificate
    #[builder(into, default)]
    pub cert_material: pulumi_gestalt_rust::Input<Option<String>>,
    /// Path to directory with Docker TLS config
    #[builder(into, default)]
    pub cert_path: pulumi_gestalt_rust::Input<Option<String>>,
    /// The Docker daemon address
    #[builder(into, default)]
    pub host: pulumi_gestalt_rust::Input<Option<String>>,
    /// PEM-encoded content of Docker client private key
    #[builder(into, default)]
    pub key_material: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub registry_auth: pulumi_gestalt_rust::Input<
        Option<Vec<super::types::ProviderRegistryAuth>>,
    >,
    /// Additional SSH option flags to be appended when using `ssh://` protocol
    #[builder(into, default)]
    pub ssh_opts: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    /// PEM-encoded content of Docker host CA certificate
    pub ca_material: pulumi_gestalt_rust::Output<Option<String>>,
    /// PEM-encoded content of Docker client certificate
    pub cert_material: pulumi_gestalt_rust::Output<Option<String>>,
    /// Path to directory with Docker TLS config
    pub cert_path: pulumi_gestalt_rust::Output<Option<String>>,
    /// The Docker daemon address
    pub host: pulumi_gestalt_rust::Output<Option<String>>,
    /// PEM-encoded content of Docker client private key
    pub key_material: pulumi_gestalt_rust::Output<Option<String>>,
    pub registry_auth: pulumi_gestalt_rust::Output<
        Option<Vec<super::types::ProviderRegistryAuth>>,
    >,
    /// Additional SSH option flags to be appended when using `ssh://` protocol
    pub ssh_opts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
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
    let ca_material_binding = args.ca_material.get_output(context);
    let cert_material_binding = args.cert_material.get_output(context);
    let cert_path_binding = args.cert_path.get_output(context);
    let host_binding = args.host.get_output(context);
    let key_material_binding = args.key_material.get_output(context);
    let registry_auth_binding = args.registry_auth.get_output(context);
    let ssh_opts_binding = args.ssh_opts.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:docker".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "caMaterial".into(),
                value: &ca_material_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "certMaterial".into(),
                value: &cert_material_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "certPath".into(),
                value: &cert_path_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "host".into(),
                value: &host_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "keyMaterial".into(),
                value: &key_material_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "registryAuth".into(),
                value: &registry_auth_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "sshOpts".into(),
                value: &ssh_opts_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        ca_material: o.get_field("caMaterial"),
        cert_material: o.get_field("certMaterial"),
        cert_path: o.get_field("certPath"),
        host: o.get_field("host"),
        key_material: o.get_field("keyMaterial"),
        registry_auth: o.get_field("registryAuth"),
        ssh_opts: o.get_field("sshOpts"),
    }
}
