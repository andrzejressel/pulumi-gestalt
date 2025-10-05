/// Allows you to manage an Azure SQL Elastic Pool.
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
///             .name("my-resource-group")
///             .build_struct(),
///     );
///     let exampleElasticPool = elastic_pool::create(
///         "exampleElasticPool",
///         ElasticPoolArgs::builder()
///             .license_type("LicenseIncluded")
///             .location("${example.location}")
///             .max_size_gb(756)
///             .name("test-epool")
///             .per_database_settings(
///                 ElasticPoolPerDatabaseSettings::builder()
///                     .maxCapacity(4)
///                     .minCapacity(0.25)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .sku(
///                 ElasticPoolSku::builder()
///                     .capacity(4)
///                     .family("Gen4")
///                     .name("BasicPool")
///                     .tier("Basic")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("my-sql-server")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Elastic Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/elasticPool:ElasticPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver/elasticPools/myelasticpoolname
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod elastic_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticPoolArgs {
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the elastic pool. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** All databases that are added to the elastic pool must have the same `enclave_type` as the elastic pool.
        ///
        /// > **NOTE:** `enclave_type` is not supported for DC-series SKUs.
        ///
        /// > **NOTE:** The default value for `enclave_type` field is unset not `Default`.
        #[builder(into, default)]
        pub enclave_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Public Maintenance Configuration window to apply to the elastic pool. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`, `SQL_SouthAfricaNorth_DB_1`, `SQL_SouthAfricaNorth_DB_2`, `SQL_WestUS3_DB_1`, `SQL_WestUS3_DB_2`. Defaults to `SQL_Default`.
        #[builder(into, default)]
        pub maintenance_configuration_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The max data size of the elastic pool in bytes. Conflicts with `max_size_gb`.
        ///
        /// > **NOTE:** One of either `max_size_gb` or `max_size_bytes` must be specified.
        #[builder(into, default)]
        pub max_size_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The max data size of the elastic pool in gigabytes. Conflicts with `max_size_bytes`.
        #[builder(into, default)]
        pub max_size_gb: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The name of the elastic pool. This needs to be globally unique. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `per_database_settings` block as defined below.
        #[builder(into)]
        pub per_database_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mssql::ElasticPoolPerDatabaseSettings,
        >,
        /// The name of the resource group in which to create the elastic pool. This must be the same as the resource group of the underlying SQL server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SQL Server on which to create the elastic pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mssql::ElasticPoolSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether or not this elastic pool is zone redundant. `tier` needs to be `Premium` for `DTU` based or `BusinessCritical` for `vCore` based `sku`.
        #[builder(into, default)]
        pub zone_redundant: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ElasticPoolResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of enclave to be used by the elastic pool. When `enclave_type` is not specified (e.g., the default) enclaves are not enabled on the elastic pool. <!-- TODO: Uncomment in 4.0: Once enabled (e.g., by specifying `Default` or `VBS`) removing the `enclave_type` field from the configuration file will force the creation of a new resource.-> Possible values are `Default` or `VBS`.
        ///
        /// > **NOTE:** All databases that are added to the elastic pool must have the same `enclave_type` as the elastic pool.
        ///
        /// > **NOTE:** `enclave_type` is not supported for DC-series SKUs.
        ///
        /// > **NOTE:** The default value for `enclave_type` field is unset not `Default`.
        pub enclave_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the license type applied to this database. Possible values are `LicenseIncluded` and `BasePrice`.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Public Maintenance Configuration window to apply to the elastic pool. Valid values include `SQL_Default`, `SQL_EastUS_DB_1`, `SQL_EastUS2_DB_1`, `SQL_SoutheastAsia_DB_1`, `SQL_AustraliaEast_DB_1`, `SQL_NorthEurope_DB_1`, `SQL_SouthCentralUS_DB_1`, `SQL_WestUS2_DB_1`, `SQL_UKSouth_DB_1`, `SQL_WestEurope_DB_1`, `SQL_EastUS_DB_2`, `SQL_EastUS2_DB_2`, `SQL_WestUS2_DB_2`, `SQL_SoutheastAsia_DB_2`, `SQL_AustraliaEast_DB_2`, `SQL_NorthEurope_DB_2`, `SQL_SouthCentralUS_DB_2`, `SQL_UKSouth_DB_2`, `SQL_WestEurope_DB_2`, `SQL_AustraliaSoutheast_DB_1`, `SQL_BrazilSouth_DB_1`, `SQL_CanadaCentral_DB_1`, `SQL_CanadaEast_DB_1`, `SQL_CentralUS_DB_1`, `SQL_EastAsia_DB_1`, `SQL_FranceCentral_DB_1`, `SQL_GermanyWestCentral_DB_1`, `SQL_CentralIndia_DB_1`, `SQL_SouthIndia_DB_1`, `SQL_JapanEast_DB_1`, `SQL_JapanWest_DB_1`, `SQL_NorthCentralUS_DB_1`, `SQL_UKWest_DB_1`, `SQL_WestUS_DB_1`, `SQL_AustraliaSoutheast_DB_2`, `SQL_BrazilSouth_DB_2`, `SQL_CanadaCentral_DB_2`, `SQL_CanadaEast_DB_2`, `SQL_CentralUS_DB_2`, `SQL_EastAsia_DB_2`, `SQL_FranceCentral_DB_2`, `SQL_GermanyWestCentral_DB_2`, `SQL_CentralIndia_DB_2`, `SQL_SouthIndia_DB_2`, `SQL_JapanEast_DB_2`, `SQL_JapanWest_DB_2`, `SQL_NorthCentralUS_DB_2`, `SQL_UKWest_DB_2`, `SQL_WestUS_DB_2`, `SQL_WestCentralUS_DB_1`, `SQL_FranceSouth_DB_1`, `SQL_WestCentralUS_DB_2`, `SQL_FranceSouth_DB_2`, `SQL_SwitzerlandNorth_DB_1`, `SQL_SwitzerlandNorth_DB_2`, `SQL_BrazilSoutheast_DB_1`, `SQL_UAENorth_DB_1`, `SQL_BrazilSoutheast_DB_2`, `SQL_UAENorth_DB_2`, `SQL_SouthAfricaNorth_DB_1`, `SQL_SouthAfricaNorth_DB_2`, `SQL_WestUS3_DB_1`, `SQL_WestUS3_DB_2`. Defaults to `SQL_Default`.
        pub maintenance_configuration_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The max data size of the elastic pool in bytes. Conflicts with `max_size_gb`.
        ///
        /// > **NOTE:** One of either `max_size_gb` or `max_size_bytes` must be specified.
        pub max_size_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The max data size of the elastic pool in gigabytes. Conflicts with `max_size_bytes`.
        pub max_size_gb: pulumi_gestalt_rust::Output<f64>,
        /// The name of the elastic pool. This needs to be globally unique. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `per_database_settings` block as defined below.
        pub per_database_settings: pulumi_gestalt_rust::Output<
            super::super::types::mssql::ElasticPoolPerDatabaseSettings,
        >,
        /// The name of the resource group in which to create the elastic pool. This must be the same as the resource group of the underlying SQL server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the SQL Server on which to create the elastic pool. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_gestalt_rust::Output<super::super::types::mssql::ElasticPoolSku>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether or not this elastic pool is zone redundant. `tier` needs to be `Premium` for `DTU` based or `BusinessCritical` for `vCore` based `sku`.
        pub zone_redundant: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ElasticPoolArgs,
    ) -> ElasticPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enclave_type_binding = args.enclave_type.get_output(context);
        let license_type_binding = args.license_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let maintenance_configuration_name_binding = args
            .maintenance_configuration_name
            .get_output(context);
        let max_size_bytes_binding = args.max_size_bytes.get_output(context);
        let max_size_gb_binding = args.max_size_gb.get_output(context);
        let name_binding = args.name.get_output(context);
        let per_database_settings_binding = args
            .per_database_settings
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_name_binding = args.server_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_redundant_binding = args.zone_redundant.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/elasticPool:ElasticPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enclaveType".into(),
                    value: &enclave_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfigurationName".into(),
                    value: &maintenance_configuration_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSizeBytes".into(),
                    value: &max_size_bytes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSizeGb".into(),
                    value: &max_size_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "perDatabaseSettings".into(),
                    value: &per_database_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneRedundant".into(),
                    value: &zone_redundant_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ElasticPoolResult {
            id: o.get_field("id"),
            enclave_type: o.get_field("enclaveType"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            maintenance_configuration_name: o.get_field("maintenanceConfigurationName"),
            max_size_bytes: o.get_field("maxSizeBytes"),
            max_size_gb: o.get_field("maxSizeGb"),
            name: o.get_field("name"),
            per_database_settings: o.get_field("perDatabaseSettings"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_name: o.get_field("serverName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            zone_redundant: o.get_field("zoneRedundant"),
        }
    }
}
