/// Manages an Elasticsearch in Elastic Cloud.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = resource_group::create(
///         "test",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let testElasticsearch = elasticsearch::create(
///         "testElasticsearch",
///         ElasticsearchArgs::builder()
///             .elastic_cloud_email_address("user@example.com")
///             .location("${test.location}")
///             .name("example-elasticsearch")
///             .resource_group_name("${test.name}")
///             .sku_name("ess-consumption-2024_Monthly")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Elasticsearch's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:elasticcloud/elasticsearch:Elasticsearch example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Elastic/monitors/monitor1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod elasticsearch {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ElasticsearchArgs {
        /// Specifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
        #[builder(into)]
        pub elastic_cloud_email_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticcloud::ElasticsearchLogs>,
        >,
        /// Specifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub monitoring_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.
        ///
        /// > **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
        /// Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
        /// You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Elasticsearch resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ElasticsearchResult {
        /// The ID of the Deployment within Elastic Cloud.
        pub elastic_cloud_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
        pub elastic_cloud_email_address: pulumi_gestalt_rust::Output<String>,
        /// The Default URL used for Single Sign On (SSO) to Elastic Cloud.
        pub elastic_cloud_sso_default_url: pulumi_gestalt_rust::Output<String>,
        /// The ID of the User Account within Elastic Cloud.
        pub elastic_cloud_user_id: pulumi_gestalt_rust::Output<String>,
        /// The URL to the Elasticsearch Service associated with this Elasticsearch.
        pub elasticsearch_service_url: pulumi_gestalt_rust::Output<String>,
        /// The URL to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_service_url: pulumi_gestalt_rust::Output<String>,
        /// The URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
        pub kibana_sso_uri: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticcloud::ElasticsearchLogs>,
        >,
        /// Specifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
        pub monitoring_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.
        ///
        /// > **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
        /// Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
        /// You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Elasticsearch resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ElasticsearchArgs,
    ) -> ElasticsearchResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let elastic_cloud_email_address_binding = args
            .elastic_cloud_email_address
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let monitoring_enabled_binding = args.monitoring_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:elasticcloud/elasticsearch:Elasticsearch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticCloudEmailAddress".into(),
                    value: &elastic_cloud_email_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoringEnabled".into(),
                    value: &monitoring_enabled_binding.drop_type(),
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
            ],
        };
        let o = context.register_resource(request);
        ElasticsearchResult {
            elastic_cloud_deployment_id: o.get_field("elasticCloudDeploymentId"),
            elastic_cloud_email_address: o.get_field("elasticCloudEmailAddress"),
            elastic_cloud_sso_default_url: o.get_field("elasticCloudSsoDefaultUrl"),
            elastic_cloud_user_id: o.get_field("elasticCloudUserId"),
            elasticsearch_service_url: o.get_field("elasticsearchServiceUrl"),
            kibana_service_url: o.get_field("kibanaServiceUrl"),
            kibana_sso_uri: o.get_field("kibanaSsoUri"),
            location: o.get_field("location"),
            logs: o.get_field("logs"),
            monitoring_enabled: o.get_field("monitoringEnabled"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
