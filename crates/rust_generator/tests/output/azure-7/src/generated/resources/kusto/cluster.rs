/// Manages a Kusto (also known as Azure Data Explorer) Cluster
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: my-kusto-cluster-rg
///       location: West Europe
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: kustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// Kusto Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/cluster:Cluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// List of allowed FQDNs(Fully Qualified Domain Name) for egress from Cluster.
        #[builder(into, default)]
        pub allowed_fqdns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The list of ips in the format of CIDR allowed to connect to the cluster.
        #[builder(into, default)]
        pub allowed_ip_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies if the cluster could be automatically stopped (due to lack of data or no activity for many days). Defaults to `true`.
        #[builder(into, default)]
        pub auto_stop_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if the cluster's disks are encrypted.
        #[builder(into, default)]
        pub disk_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is the cluster's double encryption enabled? Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub double_encryption_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kusto::ClusterIdentity>,
        >,
        /// An list of `language_extensions` to enable. Valid values are: `PYTHON`, `PYTHON_3.10.8` and `R`. `PYTHON` is used to specify Python 3.6.5 image and `PYTHON_3.10.8` is used to specify Python 3.10.8 image. Note that `PYTHON_3.10.8` is only available in skus which support nested virtualization.
        ///
        /// > **NOTE:** In `v4.0.0` and later version of the AzureRM Provider, `language_extensions` will be changed to a list of `language_extension` block. In each block, `name` and `image` are required. `name` is the name of the language extension, possible values are `PYTHON`, `R`. `image` is the image of the language extension, possible values are `Python3_6_5`, `Python3_10_8` and `R`.
        #[builder(into, default)]
        pub language_extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::kusto::ClusterLanguageExtension>>,
        >,
        /// The location where the Kusto Cluster should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Kusto Cluster to create. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `optimized_auto_scale` block as defined below.
        #[builder(into, default)]
        pub optimized_auto_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kusto::ClusterOptimizedAutoScale>,
        >,
        /// Whether to restrict outbound network access. Value is optional but if passed in, must be `true` or `false`, default is `false`.
        #[builder(into, default)]
        pub outbound_network_access_restricted: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Indicates what public IP type to create - IPv4 (default), or DualStack (both IPv4 and IPv6). Defaults to `IPv4`.
        #[builder(into, default)]
        pub public_ip_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the public network access enabled? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies if the purge operations are enabled.
        #[builder(into, default)]
        pub purge_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::kusto::ClusterSku,
        >,
        /// Specifies if the streaming ingest is enabled.
        #[builder(into, default)]
        pub streaming_ingestion_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of tenant IDs that are trusted by the cluster. Default setting trusts all other tenants. Use `trusted_external_tenants = ["*"]` to explicitly allow all other tenants, `trusted_external_tenants = ["MyTenantOnly"]` for only your tenant or `trusted_external_tenants = ["<tenantId1>", "<tenantIdx>"]` to allow specific other tenants.
        ///
        /// > **NOTE:** In v3.0 of `azurerm` a new or updated Kusto Cluster will only allow your own tenant by default. Explicit configuration of this setting will change from `trusted_external_tenants = ["MyTenantOnly"]` to `trusted_external_tenants = []`.
        #[builder(into, default)]
        pub trusted_external_tenants: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A `virtual_network_configuration` block as defined below.
        ///
        /// > **NOTE:** Currently removing `virtual_network_configuration` sets the `virtual_network_configuration` to `Disabled` state. But any changes to `virtual_network_configuration` in `Disabled` state forces a new resource to be created.
        #[builder(into, default)]
        pub virtual_network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kusto::ClusterVirtualNetworkConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Kusto Cluster should be located. Changing this forces a new Kusto Cluster to be created.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// List of allowed FQDNs(Fully Qualified Domain Name) for egress from Cluster.
        pub allowed_fqdns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The list of ips in the format of CIDR allowed to connect to the cluster.
        pub allowed_ip_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies if the cluster could be automatically stopped (due to lack of data or no activity for many days). Defaults to `true`.
        pub auto_stop_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Kusto Cluster URI to be used for data ingestion.
        pub data_ingestion_uri: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the cluster's disks are encrypted.
        pub disk_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is the cluster's double encryption enabled? Changing this forces a new resource to be created.
        pub double_encryption_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::kusto::ClusterIdentity>,
        >,
        /// An list of `language_extensions` to enable. Valid values are: `PYTHON`, `PYTHON_3.10.8` and `R`. `PYTHON` is used to specify Python 3.6.5 image and `PYTHON_3.10.8` is used to specify Python 3.10.8 image. Note that `PYTHON_3.10.8` is only available in skus which support nested virtualization.
        ///
        /// > **NOTE:** In `v4.0.0` and later version of the AzureRM Provider, `language_extensions` will be changed to a list of `language_extension` block. In each block, `name` and `image` are required. `name` is the name of the language extension, possible values are `PYTHON`, `R`. `image` is the image of the language extension, possible values are `Python3_6_5`, `Python3_10_8` and `R`.
        pub language_extensions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::kusto::ClusterLanguageExtension>>,
        >,
        /// The location where the Kusto Cluster should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Kusto Cluster to create. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `optimized_auto_scale` block as defined below.
        pub optimized_auto_scale: pulumi_gestalt_rust::Output<
            Option<super::super::types::kusto::ClusterOptimizedAutoScale>,
        >,
        /// Whether to restrict outbound network access. Value is optional but if passed in, must be `true` or `false`, default is `false`.
        pub outbound_network_access_restricted: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Indicates what public IP type to create - IPv4 (default), or DualStack (both IPv4 and IPv6). Defaults to `IPv4`.
        pub public_ip_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is the public network access enabled? Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if the purge operations are enabled.
        pub purge_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Resource Group where the Kusto Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::kusto::ClusterSku>,
        /// Specifies if the streaming ingest is enabled.
        pub streaming_ingestion_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of tenant IDs that are trusted by the cluster. Default setting trusts all other tenants. Use `trusted_external_tenants = ["*"]` to explicitly allow all other tenants, `trusted_external_tenants = ["MyTenantOnly"]` for only your tenant or `trusted_external_tenants = ["<tenantId1>", "<tenantIdx>"]` to allow specific other tenants.
        ///
        /// > **NOTE:** In v3.0 of `azurerm` a new or updated Kusto Cluster will only allow your own tenant by default. Explicit configuration of this setting will change from `trusted_external_tenants = ["MyTenantOnly"]` to `trusted_external_tenants = []`.
        pub trusted_external_tenants: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The FQDN of the Azure Kusto Cluster.
        pub uri: pulumi_gestalt_rust::Output<String>,
        /// A `virtual_network_configuration` block as defined below.
        ///
        /// > **NOTE:** Currently removing `virtual_network_configuration` sets the `virtual_network_configuration` to `Disabled` state. But any changes to `virtual_network_configuration` in `Disabled` state forces a new resource to be created.
        pub virtual_network_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::kusto::ClusterVirtualNetworkConfiguration>,
        >,
        /// Specifies a list of Availability Zones in which this Kusto Cluster should be located. Changing this forces a new Kusto Cluster to be created.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_fqdns_binding = args.allowed_fqdns.get_output(context);
        let allowed_ip_ranges_binding = args.allowed_ip_ranges.get_output(context);
        let auto_stop_enabled_binding = args.auto_stop_enabled.get_output(context);
        let disk_encryption_enabled_binding = args
            .disk_encryption_enabled
            .get_output(context);
        let double_encryption_enabled_binding = args
            .double_encryption_enabled
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let language_extensions_binding = args.language_extensions.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let optimized_auto_scale_binding = args.optimized_auto_scale.get_output(context);
        let outbound_network_access_restricted_binding = args
            .outbound_network_access_restricted
            .get_output(context);
        let public_ip_type_binding = args.public_ip_type.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let purge_enabled_binding = args.purge_enabled.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let streaming_ingestion_enabled_binding = args
            .streaming_ingestion_enabled
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trusted_external_tenants_binding = args
            .trusted_external_tenants
            .get_output(context);
        let virtual_network_configuration_binding = args
            .virtual_network_configuration
            .get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:kusto/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedFqdns".into(),
                    value: &allowed_fqdns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedIpRanges".into(),
                    value: &allowed_ip_ranges_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoStopEnabled".into(),
                    value: &auto_stop_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskEncryptionEnabled".into(),
                    value: &disk_encryption_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "doubleEncryptionEnabled".into(),
                    value: &double_encryption_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageExtensions".into(),
                    value: &language_extensions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optimizedAutoScale".into(),
                    value: &optimized_auto_scale_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outboundNetworkAccessRestricted".into(),
                    value: &outbound_network_access_restricted_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpType".into(),
                    value: &public_ip_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purgeEnabled".into(),
                    value: &purge_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingIngestionEnabled".into(),
                    value: &streaming_ingestion_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustedExternalTenants".into(),
                    value: &trusted_external_tenants_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkConfiguration".into(),
                    value: &virtual_network_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            allowed_fqdns: o.get_field("allowedFqdns"),
            allowed_ip_ranges: o.get_field("allowedIpRanges"),
            auto_stop_enabled: o.get_field("autoStopEnabled"),
            data_ingestion_uri: o.get_field("dataIngestionUri"),
            disk_encryption_enabled: o.get_field("diskEncryptionEnabled"),
            double_encryption_enabled: o.get_field("doubleEncryptionEnabled"),
            identity: o.get_field("identity"),
            language_extensions: o.get_field("languageExtensions"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            optimized_auto_scale: o.get_field("optimizedAutoScale"),
            outbound_network_access_restricted: o
                .get_field("outboundNetworkAccessRestricted"),
            public_ip_type: o.get_field("publicIpType"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            purge_enabled: o.get_field("purgeEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            streaming_ingestion_enabled: o.get_field("streamingIngestionEnabled"),
            tags: o.get_field("tags"),
            trusted_external_tenants: o.get_field("trustedExternalTenants"),
            uri: o.get_field("uri"),
            virtual_network_configuration: o.get_field("virtualNetworkConfiguration"),
            zones: o.get_field("zones"),
        }
    }
}
