/// Manages a Redis Enterprise Cluster.
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
///             .name("example-redisenterprise")
///             .build_struct(),
///     );
///     let exampleEnterpriseCluster = enterprise_cluster::create(
///         "exampleEnterpriseCluster",
///         EnterpriseClusterArgs::builder()
///             .location("${example.location}")
///             .name("example-redisenterprise")
///             .resource_group_name("${example.name}")
///             .sku_name("EnterpriseFlash_F300-3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Redis Enterprise Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:redis/enterpriseCluster:EnterpriseCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Cache/redisEnterprise/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enterprise_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnterpriseClusterArgs {
        /// The Azure Region where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.2`. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Redis Enterprise Cluster. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The `sku_name` is comprised of two segments separated by a hyphen (e.g. `Enterprise_E10-2`). The first segment of the `sku_name` defines the `name` of the SKU, possible values are `Enterprise_E5`, `Enterprise_E10`, `Enterprise_E20`, `Enterprise_E50`, `Enterprise_E100`, `Enterprise_E200`, `Enterprise_E400`, `EnterpriseFlash_F300`, `EnterpriseFlash_F700` or `EnterpriseFlash_F1500`. The second segment defines the `capacity` of the `sku_name`, possible values for `Enteprise` SKUs are (`2`, `4`, `6`, ...). Possible values for `EnterpriseFlash` SKUs are (`3`, `9`, `15`, ...). Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Redis Enterprise Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Enterprise Cluster should be located. Changing this forces a new Redis Enterprise Cluster to be created.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EnterpriseClusterResult {
        /// DNS name of the cluster endpoint.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The minimum TLS version. Possible values are `1.0`, `1.1` and `1.2`. Defaults to `1.2`. Changing this forces a new Redis Enterprise Cluster to be created.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Redis Enterprise Cluster. Changing this forces a new Redis Enterprise Cluster to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Redis Enterprise Cluster should exist. Changing this forces a new Redis Enterprise Cluster to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The `sku_name` is comprised of two segments separated by a hyphen (e.g. `Enterprise_E10-2`). The first segment of the `sku_name` defines the `name` of the SKU, possible values are `Enterprise_E5`, `Enterprise_E10`, `Enterprise_E20`, `Enterprise_E50`, `Enterprise_E100`, `Enterprise_E200`, `Enterprise_E400`, `EnterpriseFlash_F300`, `EnterpriseFlash_F700` or `EnterpriseFlash_F1500`. The second segment defines the `capacity` of the `sku_name`, possible values for `Enteprise` SKUs are (`2`, `4`, `6`, ...). Possible values for `EnterpriseFlash` SKUs are (`3`, `9`, `15`, ...). Changing this forces a new Redis Enterprise Cluster to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Redis Enterprise Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies a list of Availability Zones in which this Redis Enterprise Cluster should be located. Changing this forces a new Redis Enterprise Cluster to be created.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnterpriseClusterArgs,
    ) -> EnterpriseClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let minimum_tls_version_binding = args.minimum_tls_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:redis/enterpriseCluster:EnterpriseCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: &zones_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnterpriseClusterResult {
            hostname: o.get_field("hostname"),
            location: o.get_field("location"),
            minimum_tls_version: o.get_field("minimumTlsVersion"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
