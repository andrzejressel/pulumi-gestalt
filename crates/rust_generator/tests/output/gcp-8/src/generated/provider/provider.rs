/// The provider type for the google-beta package. By default, resources use package-wide configuration
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
    pub access_approval_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub access_context_manager_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub access_token: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub active_directory_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub add_pulumi_attribution_label: pulumi_gestalt_rust::Input<Option<bool>>,
    #[builder(into, default)]
    pub alloydb_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub api_gateway_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub apigee_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub apikeys_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub app_engine_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub apphub_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub artifact_registry_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub assured_workloads_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub backup_dr_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub batching: pulumi_gestalt_rust::Input<Option<super::types::ProviderBatching>>,
    #[builder(into, default)]
    pub beyondcorp_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub big_query_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub biglake_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub bigquery_analytics_hub_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub bigquery_connection_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub bigquery_data_transfer_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub bigquery_datapolicy_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub bigquery_reservation_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub bigtable_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub billing_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub billing_project: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub binary_authorization_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub blockchain_node_engine_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub certificate_manager_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_asset_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_billing_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_build_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_build_worker_pool_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub cloud_functions_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_identity_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_ids_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_quotas_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_resource_manager_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub cloud_run_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_run_v2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_scheduler_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloud_tasks_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloudbuildv2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub clouddeploy_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub clouddomains_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub cloudfunctions2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub composer_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub compute_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub container_analysis_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub container_attached_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub container_aws_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub container_azure_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub container_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub core_billing_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub credentials: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub data_catalog_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub data_fusion_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub data_loss_prevention_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub data_pipeline_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub database_migration_service_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub dataflow_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dataform_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dataplex_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dataproc_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dataproc_gdc_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dataproc_metastore_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub datastream_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub default_labels: pulumi_gestalt_rust::Input<
        Option<std::collections::HashMap<String, String>>,
    >,
    #[builder(into, default)]
    pub deployment_manager_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub developer_connect_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dialogflow_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dialogflow_cx_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub disable_google_partner_name: pulumi_gestalt_rust::Input<Option<bool>>,
    #[builder(into, default)]
    pub discovery_engine_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub dns_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub document_ai_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub document_ai_warehouse_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub edgecontainer_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub edgenetwork_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub essential_contacts_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub eventarc_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub filestore_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_app_check_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_database_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_extensions_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_hosting_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebase_storage_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firebaserules_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub firestore_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gemini_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gke_backup_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gke_hub2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gke_hub_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gkehub_feature_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub gkeonprem_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub google_partner_name: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub healthcare_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam3_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam_beta_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam_credentials_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iam_workforce_pool_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub iap_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub identity_platform_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub impersonate_service_account: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub impersonate_service_account_delegates: pulumi_gestalt_rust::Input<
        Option<Vec<String>>,
    >,
    #[builder(into, default)]
    pub integration_connectors_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub integrations_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub kms_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub logging_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub looker_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub managed_kafka_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub memcache_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub memorystore_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub migration_center_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub ml_engine_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub monitoring_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub netapp_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub network_connectivity_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub network_management_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub network_security_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub network_services_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub notebooks_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub oracle_database_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub org_policy_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub os_config_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub os_login_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub parallelstore_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub privateca_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub privileged_access_manager_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub project: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub public_ca_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub pubsub_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub pubsub_lite_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub pulumi_attribution_label_addition_strategy: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub recaptcha_enterprise_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub redis_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub region: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub request_reason: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub request_timeout: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub resource_manager_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub resource_manager_v3_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub runtime_config_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub runtimeconfig_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub scopes: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    #[builder(into, default)]
    pub secret_manager_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub secret_manager_regional_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub secure_source_manager_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub security_center_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub security_center_management_custom_endpoint: pulumi_gestalt_rust::Input<
        Option<String>,
    >,
    #[builder(into, default)]
    pub security_center_v2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub security_scanner_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub securityposture_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub service_directory_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub service_management_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub service_networking_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub service_usage_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub site_verification_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub source_repo_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub spanner_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub sql_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub storage_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub storage_insights_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub storage_transfer_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub tags_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub tags_location_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub tpu_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub tpu_v2_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub transcoder_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub universe_domain: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub user_project_override: pulumi_gestalt_rust::Input<Option<bool>>,
    #[builder(into, default)]
    pub vertex_ai_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub vmwareengine_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub vpc_access_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub workbench_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub workflows_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub workstations_custom_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub zone: pulumi_gestalt_rust::Input<Option<String>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    pub access_approval_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub access_context_manager_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub access_token: pulumi_gestalt_rust::Output<Option<String>>,
    pub active_directory_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub add_pulumi_attribution_label: pulumi_gestalt_rust::Output<Option<bool>>,
    pub alloydb_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub api_gateway_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub apigee_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub apikeys_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub app_engine_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub apphub_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub artifact_registry_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub assured_workloads_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub backup_dr_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub batching: pulumi_gestalt_rust::Output<Option<super::types::ProviderBatching>>,
    pub beyondcorp_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub big_query_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub biglake_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub bigquery_analytics_hub_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub bigquery_connection_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub bigquery_data_transfer_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub bigquery_datapolicy_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub bigquery_reservation_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub bigtable_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub billing_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub billing_project: pulumi_gestalt_rust::Output<Option<String>>,
    pub binary_authorization_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub blockchain_node_engine_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub certificate_manager_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_asset_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_billing_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_build_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_build_worker_pool_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub cloud_functions_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_identity_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_ids_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_quotas_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_resource_manager_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub cloud_run_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_run_v2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_scheduler_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloud_tasks_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloudbuildv2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub clouddeploy_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub clouddomains_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub cloudfunctions2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub composer_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub compute_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub container_analysis_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub container_attached_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub container_aws_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub container_azure_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub container_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub core_billing_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub credentials: pulumi_gestalt_rust::Output<Option<String>>,
    pub data_catalog_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub data_fusion_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub data_loss_prevention_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub data_pipeline_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub database_migration_service_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub dataflow_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dataform_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dataplex_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dataproc_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dataproc_gdc_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dataproc_metastore_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub datastream_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub default_labels: pulumi_gestalt_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    pub deployment_manager_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub developer_connect_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dialogflow_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dialogflow_cx_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub disable_google_partner_name: pulumi_gestalt_rust::Output<Option<bool>>,
    pub discovery_engine_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub dns_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub document_ai_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub document_ai_warehouse_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub edgecontainer_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub edgenetwork_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub essential_contacts_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub eventarc_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub filestore_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_app_check_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_database_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_extensions_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_hosting_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebase_storage_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firebaserules_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub firestore_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gemini_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gke_backup_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gke_hub2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gke_hub_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gkehub_feature_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub gkeonprem_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub google_partner_name: pulumi_gestalt_rust::Output<Option<String>>,
    pub healthcare_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam3_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam_beta_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam_credentials_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iam_workforce_pool_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub iap_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub identity_platform_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub impersonate_service_account: pulumi_gestalt_rust::Output<Option<String>>,
    pub impersonate_service_account_delegates: pulumi_gestalt_rust::Output<
        Option<Vec<String>>,
    >,
    pub integration_connectors_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub integrations_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub kms_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub logging_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub looker_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub managed_kafka_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub memcache_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub memorystore_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub migration_center_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub ml_engine_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub monitoring_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub netapp_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub network_connectivity_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub network_management_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub network_security_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub network_services_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub notebooks_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub oracle_database_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub org_policy_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub os_config_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub os_login_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub parallelstore_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub privateca_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub privileged_access_manager_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub project: pulumi_gestalt_rust::Output<Option<String>>,
    pub public_ca_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub pubsub_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub pubsub_lite_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub pulumi_attribution_label_addition_strategy: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub recaptcha_enterprise_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub redis_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub region: pulumi_gestalt_rust::Output<Option<String>>,
    pub request_reason: pulumi_gestalt_rust::Output<Option<String>>,
    pub request_timeout: pulumi_gestalt_rust::Output<Option<String>>,
    pub resource_manager_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub resource_manager_v3_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub runtime_config_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub runtimeconfig_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    pub secret_manager_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub secret_manager_regional_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub secure_source_manager_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub security_center_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub security_center_management_custom_endpoint: pulumi_gestalt_rust::Output<
        Option<String>,
    >,
    pub security_center_v2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub security_scanner_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub securityposture_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub service_directory_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub service_management_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub service_networking_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub service_usage_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub site_verification_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub source_repo_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub spanner_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub sql_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub storage_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub storage_insights_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub storage_transfer_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub tags_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub tags_location_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub tpu_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub tpu_v2_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub transcoder_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub universe_domain: pulumi_gestalt_rust::Output<Option<String>>,
    pub user_project_override: pulumi_gestalt_rust::Output<Option<bool>>,
    pub vertex_ai_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub vmwareengine_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub vpc_access_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub workbench_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub workflows_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub workstations_custom_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    pub zone: pulumi_gestalt_rust::Output<Option<String>>,
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
    let access_approval_custom_endpoint_binding = args
        .access_approval_custom_endpoint
        .get_output(context);
    let access_context_manager_custom_endpoint_binding = args
        .access_context_manager_custom_endpoint
        .get_output(context);
    let access_token_binding = args.access_token.get_output(context);
    let active_directory_custom_endpoint_binding = args
        .active_directory_custom_endpoint
        .get_output(context);
    let add_pulumi_attribution_label_binding = args
        .add_pulumi_attribution_label
        .get_output(context);
    let alloydb_custom_endpoint_binding = args
        .alloydb_custom_endpoint
        .get_output(context);
    let api_gateway_custom_endpoint_binding = args
        .api_gateway_custom_endpoint
        .get_output(context);
    let apigee_custom_endpoint_binding = args.apigee_custom_endpoint.get_output(context);
    let apikeys_custom_endpoint_binding = args
        .apikeys_custom_endpoint
        .get_output(context);
    let app_engine_custom_endpoint_binding = args
        .app_engine_custom_endpoint
        .get_output(context);
    let apphub_custom_endpoint_binding = args.apphub_custom_endpoint.get_output(context);
    let artifact_registry_custom_endpoint_binding = args
        .artifact_registry_custom_endpoint
        .get_output(context);
    let assured_workloads_custom_endpoint_binding = args
        .assured_workloads_custom_endpoint
        .get_output(context);
    let backup_dr_custom_endpoint_binding = args
        .backup_dr_custom_endpoint
        .get_output(context);
    let batching_binding = args.batching.get_output(context);
    let beyondcorp_custom_endpoint_binding = args
        .beyondcorp_custom_endpoint
        .get_output(context);
    let big_query_custom_endpoint_binding = args
        .big_query_custom_endpoint
        .get_output(context);
    let biglake_custom_endpoint_binding = args
        .biglake_custom_endpoint
        .get_output(context);
    let bigquery_analytics_hub_custom_endpoint_binding = args
        .bigquery_analytics_hub_custom_endpoint
        .get_output(context);
    let bigquery_connection_custom_endpoint_binding = args
        .bigquery_connection_custom_endpoint
        .get_output(context);
    let bigquery_data_transfer_custom_endpoint_binding = args
        .bigquery_data_transfer_custom_endpoint
        .get_output(context);
    let bigquery_datapolicy_custom_endpoint_binding = args
        .bigquery_datapolicy_custom_endpoint
        .get_output(context);
    let bigquery_reservation_custom_endpoint_binding = args
        .bigquery_reservation_custom_endpoint
        .get_output(context);
    let bigtable_custom_endpoint_binding = args
        .bigtable_custom_endpoint
        .get_output(context);
    let billing_custom_endpoint_binding = args
        .billing_custom_endpoint
        .get_output(context);
    let billing_project_binding = args.billing_project.get_output(context);
    let binary_authorization_custom_endpoint_binding = args
        .binary_authorization_custom_endpoint
        .get_output(context);
    let blockchain_node_engine_custom_endpoint_binding = args
        .blockchain_node_engine_custom_endpoint
        .get_output(context);
    let certificate_manager_custom_endpoint_binding = args
        .certificate_manager_custom_endpoint
        .get_output(context);
    let cloud_asset_custom_endpoint_binding = args
        .cloud_asset_custom_endpoint
        .get_output(context);
    let cloud_billing_custom_endpoint_binding = args
        .cloud_billing_custom_endpoint
        .get_output(context);
    let cloud_build_custom_endpoint_binding = args
        .cloud_build_custom_endpoint
        .get_output(context);
    let cloud_build_worker_pool_custom_endpoint_binding = args
        .cloud_build_worker_pool_custom_endpoint
        .get_output(context);
    let cloud_functions_custom_endpoint_binding = args
        .cloud_functions_custom_endpoint
        .get_output(context);
    let cloud_identity_custom_endpoint_binding = args
        .cloud_identity_custom_endpoint
        .get_output(context);
    let cloud_ids_custom_endpoint_binding = args
        .cloud_ids_custom_endpoint
        .get_output(context);
    let cloud_quotas_custom_endpoint_binding = args
        .cloud_quotas_custom_endpoint
        .get_output(context);
    let cloud_resource_manager_custom_endpoint_binding = args
        .cloud_resource_manager_custom_endpoint
        .get_output(context);
    let cloud_run_custom_endpoint_binding = args
        .cloud_run_custom_endpoint
        .get_output(context);
    let cloud_run_v2_custom_endpoint_binding = args
        .cloud_run_v2_custom_endpoint
        .get_output(context);
    let cloud_scheduler_custom_endpoint_binding = args
        .cloud_scheduler_custom_endpoint
        .get_output(context);
    let cloud_tasks_custom_endpoint_binding = args
        .cloud_tasks_custom_endpoint
        .get_output(context);
    let cloudbuildv2_custom_endpoint_binding = args
        .cloudbuildv2_custom_endpoint
        .get_output(context);
    let clouddeploy_custom_endpoint_binding = args
        .clouddeploy_custom_endpoint
        .get_output(context);
    let clouddomains_custom_endpoint_binding = args
        .clouddomains_custom_endpoint
        .get_output(context);
    let cloudfunctions2_custom_endpoint_binding = args
        .cloudfunctions2_custom_endpoint
        .get_output(context);
    let composer_custom_endpoint_binding = args
        .composer_custom_endpoint
        .get_output(context);
    let compute_custom_endpoint_binding = args
        .compute_custom_endpoint
        .get_output(context);
    let container_analysis_custom_endpoint_binding = args
        .container_analysis_custom_endpoint
        .get_output(context);
    let container_attached_custom_endpoint_binding = args
        .container_attached_custom_endpoint
        .get_output(context);
    let container_aws_custom_endpoint_binding = args
        .container_aws_custom_endpoint
        .get_output(context);
    let container_azure_custom_endpoint_binding = args
        .container_azure_custom_endpoint
        .get_output(context);
    let container_custom_endpoint_binding = args
        .container_custom_endpoint
        .get_output(context);
    let core_billing_custom_endpoint_binding = args
        .core_billing_custom_endpoint
        .get_output(context);
    let credentials_binding = args.credentials.get_output(context);
    let data_catalog_custom_endpoint_binding = args
        .data_catalog_custom_endpoint
        .get_output(context);
    let data_fusion_custom_endpoint_binding = args
        .data_fusion_custom_endpoint
        .get_output(context);
    let data_loss_prevention_custom_endpoint_binding = args
        .data_loss_prevention_custom_endpoint
        .get_output(context);
    let data_pipeline_custom_endpoint_binding = args
        .data_pipeline_custom_endpoint
        .get_output(context);
    let database_migration_service_custom_endpoint_binding = args
        .database_migration_service_custom_endpoint
        .get_output(context);
    let dataflow_custom_endpoint_binding = args
        .dataflow_custom_endpoint
        .get_output(context);
    let dataform_custom_endpoint_binding = args
        .dataform_custom_endpoint
        .get_output(context);
    let dataplex_custom_endpoint_binding = args
        .dataplex_custom_endpoint
        .get_output(context);
    let dataproc_custom_endpoint_binding = args
        .dataproc_custom_endpoint
        .get_output(context);
    let dataproc_gdc_custom_endpoint_binding = args
        .dataproc_gdc_custom_endpoint
        .get_output(context);
    let dataproc_metastore_custom_endpoint_binding = args
        .dataproc_metastore_custom_endpoint
        .get_output(context);
    let datastream_custom_endpoint_binding = args
        .datastream_custom_endpoint
        .get_output(context);
    let default_labels_binding = args.default_labels.get_output(context);
    let deployment_manager_custom_endpoint_binding = args
        .deployment_manager_custom_endpoint
        .get_output(context);
    let developer_connect_custom_endpoint_binding = args
        .developer_connect_custom_endpoint
        .get_output(context);
    let dialogflow_custom_endpoint_binding = args
        .dialogflow_custom_endpoint
        .get_output(context);
    let dialogflow_cx_custom_endpoint_binding = args
        .dialogflow_cx_custom_endpoint
        .get_output(context);
    let disable_google_partner_name_binding = args
        .disable_google_partner_name
        .get_output(context);
    let discovery_engine_custom_endpoint_binding = args
        .discovery_engine_custom_endpoint
        .get_output(context);
    let dns_custom_endpoint_binding = args.dns_custom_endpoint.get_output(context);
    let document_ai_custom_endpoint_binding = args
        .document_ai_custom_endpoint
        .get_output(context);
    let document_ai_warehouse_custom_endpoint_binding = args
        .document_ai_warehouse_custom_endpoint
        .get_output(context);
    let edgecontainer_custom_endpoint_binding = args
        .edgecontainer_custom_endpoint
        .get_output(context);
    let edgenetwork_custom_endpoint_binding = args
        .edgenetwork_custom_endpoint
        .get_output(context);
    let essential_contacts_custom_endpoint_binding = args
        .essential_contacts_custom_endpoint
        .get_output(context);
    let eventarc_custom_endpoint_binding = args
        .eventarc_custom_endpoint
        .get_output(context);
    let filestore_custom_endpoint_binding = args
        .filestore_custom_endpoint
        .get_output(context);
    let firebase_app_check_custom_endpoint_binding = args
        .firebase_app_check_custom_endpoint
        .get_output(context);
    let firebase_custom_endpoint_binding = args
        .firebase_custom_endpoint
        .get_output(context);
    let firebase_database_custom_endpoint_binding = args
        .firebase_database_custom_endpoint
        .get_output(context);
    let firebase_extensions_custom_endpoint_binding = args
        .firebase_extensions_custom_endpoint
        .get_output(context);
    let firebase_hosting_custom_endpoint_binding = args
        .firebase_hosting_custom_endpoint
        .get_output(context);
    let firebase_storage_custom_endpoint_binding = args
        .firebase_storage_custom_endpoint
        .get_output(context);
    let firebaserules_custom_endpoint_binding = args
        .firebaserules_custom_endpoint
        .get_output(context);
    let firestore_custom_endpoint_binding = args
        .firestore_custom_endpoint
        .get_output(context);
    let gemini_custom_endpoint_binding = args.gemini_custom_endpoint.get_output(context);
    let gke_backup_custom_endpoint_binding = args
        .gke_backup_custom_endpoint
        .get_output(context);
    let gke_hub2_custom_endpoint_binding = args
        .gke_hub2_custom_endpoint
        .get_output(context);
    let gke_hub_custom_endpoint_binding = args
        .gke_hub_custom_endpoint
        .get_output(context);
    let gkehub_feature_custom_endpoint_binding = args
        .gkehub_feature_custom_endpoint
        .get_output(context);
    let gkeonprem_custom_endpoint_binding = args
        .gkeonprem_custom_endpoint
        .get_output(context);
    let google_partner_name_binding = args.google_partner_name.get_output(context);
    let healthcare_custom_endpoint_binding = args
        .healthcare_custom_endpoint
        .get_output(context);
    let iam2_custom_endpoint_binding = args.iam2_custom_endpoint.get_output(context);
    let iam3_custom_endpoint_binding = args.iam3_custom_endpoint.get_output(context);
    let iam_beta_custom_endpoint_binding = args
        .iam_beta_custom_endpoint
        .get_output(context);
    let iam_credentials_custom_endpoint_binding = args
        .iam_credentials_custom_endpoint
        .get_output(context);
    let iam_custom_endpoint_binding = args.iam_custom_endpoint.get_output(context);
    let iam_workforce_pool_custom_endpoint_binding = args
        .iam_workforce_pool_custom_endpoint
        .get_output(context);
    let iap_custom_endpoint_binding = args.iap_custom_endpoint.get_output(context);
    let identity_platform_custom_endpoint_binding = args
        .identity_platform_custom_endpoint
        .get_output(context);
    let impersonate_service_account_binding = args
        .impersonate_service_account
        .get_output(context);
    let impersonate_service_account_delegates_binding = args
        .impersonate_service_account_delegates
        .get_output(context);
    let integration_connectors_custom_endpoint_binding = args
        .integration_connectors_custom_endpoint
        .get_output(context);
    let integrations_custom_endpoint_binding = args
        .integrations_custom_endpoint
        .get_output(context);
    let kms_custom_endpoint_binding = args.kms_custom_endpoint.get_output(context);
    let logging_custom_endpoint_binding = args
        .logging_custom_endpoint
        .get_output(context);
    let looker_custom_endpoint_binding = args.looker_custom_endpoint.get_output(context);
    let managed_kafka_custom_endpoint_binding = args
        .managed_kafka_custom_endpoint
        .get_output(context);
    let memcache_custom_endpoint_binding = args
        .memcache_custom_endpoint
        .get_output(context);
    let memorystore_custom_endpoint_binding = args
        .memorystore_custom_endpoint
        .get_output(context);
    let migration_center_custom_endpoint_binding = args
        .migration_center_custom_endpoint
        .get_output(context);
    let ml_engine_custom_endpoint_binding = args
        .ml_engine_custom_endpoint
        .get_output(context);
    let monitoring_custom_endpoint_binding = args
        .monitoring_custom_endpoint
        .get_output(context);
    let netapp_custom_endpoint_binding = args.netapp_custom_endpoint.get_output(context);
    let network_connectivity_custom_endpoint_binding = args
        .network_connectivity_custom_endpoint
        .get_output(context);
    let network_management_custom_endpoint_binding = args
        .network_management_custom_endpoint
        .get_output(context);
    let network_security_custom_endpoint_binding = args
        .network_security_custom_endpoint
        .get_output(context);
    let network_services_custom_endpoint_binding = args
        .network_services_custom_endpoint
        .get_output(context);
    let notebooks_custom_endpoint_binding = args
        .notebooks_custom_endpoint
        .get_output(context);
    let oracle_database_custom_endpoint_binding = args
        .oracle_database_custom_endpoint
        .get_output(context);
    let org_policy_custom_endpoint_binding = args
        .org_policy_custom_endpoint
        .get_output(context);
    let os_config_custom_endpoint_binding = args
        .os_config_custom_endpoint
        .get_output(context);
    let os_login_custom_endpoint_binding = args
        .os_login_custom_endpoint
        .get_output(context);
    let parallelstore_custom_endpoint_binding = args
        .parallelstore_custom_endpoint
        .get_output(context);
    let privateca_custom_endpoint_binding = args
        .privateca_custom_endpoint
        .get_output(context);
    let privileged_access_manager_custom_endpoint_binding = args
        .privileged_access_manager_custom_endpoint
        .get_output(context);
    let project_binding = args.project.get_output(context);
    let public_ca_custom_endpoint_binding = args
        .public_ca_custom_endpoint
        .get_output(context);
    let pubsub_custom_endpoint_binding = args.pubsub_custom_endpoint.get_output(context);
    let pubsub_lite_custom_endpoint_binding = args
        .pubsub_lite_custom_endpoint
        .get_output(context);
    let pulumi_attribution_label_addition_strategy_binding = args
        .pulumi_attribution_label_addition_strategy
        .get_output(context);
    let recaptcha_enterprise_custom_endpoint_binding = args
        .recaptcha_enterprise_custom_endpoint
        .get_output(context);
    let redis_custom_endpoint_binding = args.redis_custom_endpoint.get_output(context);
    let region_binding = args.region.get_output(context);
    let request_reason_binding = args.request_reason.get_output(context);
    let request_timeout_binding = args.request_timeout.get_output(context);
    let resource_manager_custom_endpoint_binding = args
        .resource_manager_custom_endpoint
        .get_output(context);
    let resource_manager_v3_custom_endpoint_binding = args
        .resource_manager_v3_custom_endpoint
        .get_output(context);
    let runtime_config_custom_endpoint_binding = args
        .runtime_config_custom_endpoint
        .get_output(context);
    let runtimeconfig_custom_endpoint_binding = args
        .runtimeconfig_custom_endpoint
        .get_output(context);
    let scopes_binding = args.scopes.get_output(context);
    let secret_manager_custom_endpoint_binding = args
        .secret_manager_custom_endpoint
        .get_output(context);
    let secret_manager_regional_custom_endpoint_binding = args
        .secret_manager_regional_custom_endpoint
        .get_output(context);
    let secure_source_manager_custom_endpoint_binding = args
        .secure_source_manager_custom_endpoint
        .get_output(context);
    let security_center_custom_endpoint_binding = args
        .security_center_custom_endpoint
        .get_output(context);
    let security_center_management_custom_endpoint_binding = args
        .security_center_management_custom_endpoint
        .get_output(context);
    let security_center_v2_custom_endpoint_binding = args
        .security_center_v2_custom_endpoint
        .get_output(context);
    let security_scanner_custom_endpoint_binding = args
        .security_scanner_custom_endpoint
        .get_output(context);
    let securityposture_custom_endpoint_binding = args
        .securityposture_custom_endpoint
        .get_output(context);
    let service_directory_custom_endpoint_binding = args
        .service_directory_custom_endpoint
        .get_output(context);
    let service_management_custom_endpoint_binding = args
        .service_management_custom_endpoint
        .get_output(context);
    let service_networking_custom_endpoint_binding = args
        .service_networking_custom_endpoint
        .get_output(context);
    let service_usage_custom_endpoint_binding = args
        .service_usage_custom_endpoint
        .get_output(context);
    let site_verification_custom_endpoint_binding = args
        .site_verification_custom_endpoint
        .get_output(context);
    let source_repo_custom_endpoint_binding = args
        .source_repo_custom_endpoint
        .get_output(context);
    let spanner_custom_endpoint_binding = args
        .spanner_custom_endpoint
        .get_output(context);
    let sql_custom_endpoint_binding = args.sql_custom_endpoint.get_output(context);
    let storage_custom_endpoint_binding = args
        .storage_custom_endpoint
        .get_output(context);
    let storage_insights_custom_endpoint_binding = args
        .storage_insights_custom_endpoint
        .get_output(context);
    let storage_transfer_custom_endpoint_binding = args
        .storage_transfer_custom_endpoint
        .get_output(context);
    let tags_custom_endpoint_binding = args.tags_custom_endpoint.get_output(context);
    let tags_location_custom_endpoint_binding = args
        .tags_location_custom_endpoint
        .get_output(context);
    let tpu_custom_endpoint_binding = args.tpu_custom_endpoint.get_output(context);
    let tpu_v2_custom_endpoint_binding = args.tpu_v2_custom_endpoint.get_output(context);
    let transcoder_custom_endpoint_binding = args
        .transcoder_custom_endpoint
        .get_output(context);
    let universe_domain_binding = args.universe_domain.get_output(context);
    let user_project_override_binding = args.user_project_override.get_output(context);
    let vertex_ai_custom_endpoint_binding = args
        .vertex_ai_custom_endpoint
        .get_output(context);
    let vmwareengine_custom_endpoint_binding = args
        .vmwareengine_custom_endpoint
        .get_output(context);
    let vpc_access_custom_endpoint_binding = args
        .vpc_access_custom_endpoint
        .get_output(context);
    let workbench_custom_endpoint_binding = args
        .workbench_custom_endpoint
        .get_output(context);
    let workflows_custom_endpoint_binding = args
        .workflows_custom_endpoint
        .get_output(context);
    let workstations_custom_endpoint_binding = args
        .workstations_custom_endpoint
        .get_output(context);
    let zone_binding = args.zone.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:gcp".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "accessApprovalCustomEndpoint".into(),
                value: &access_approval_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "accessContextManagerCustomEndpoint".into(),
                value: &access_context_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "accessToken".into(),
                value: &access_token_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "activeDirectoryCustomEndpoint".into(),
                value: &active_directory_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "addPulumiAttributionLabel".into(),
                value: &add_pulumi_attribution_label_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "alloydbCustomEndpoint".into(),
                value: &alloydb_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apiGatewayCustomEndpoint".into(),
                value: &api_gateway_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apigeeCustomEndpoint".into(),
                value: &apigee_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apikeysCustomEndpoint".into(),
                value: &apikeys_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "appEngineCustomEndpoint".into(),
                value: &app_engine_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "apphubCustomEndpoint".into(),
                value: &apphub_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "artifactRegistryCustomEndpoint".into(),
                value: &artifact_registry_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "assuredWorkloadsCustomEndpoint".into(),
                value: &assured_workloads_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "backupDrCustomEndpoint".into(),
                value: &backup_dr_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "batching".into(),
                value: &batching_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "beyondcorpCustomEndpoint".into(),
                value: &beyondcorp_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigQueryCustomEndpoint".into(),
                value: &big_query_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "biglakeCustomEndpoint".into(),
                value: &biglake_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigqueryAnalyticsHubCustomEndpoint".into(),
                value: &bigquery_analytics_hub_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigqueryConnectionCustomEndpoint".into(),
                value: &bigquery_connection_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigqueryDataTransferCustomEndpoint".into(),
                value: &bigquery_data_transfer_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigqueryDatapolicyCustomEndpoint".into(),
                value: &bigquery_datapolicy_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigqueryReservationCustomEndpoint".into(),
                value: &bigquery_reservation_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "bigtableCustomEndpoint".into(),
                value: &bigtable_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "billingCustomEndpoint".into(),
                value: &billing_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "billingProject".into(),
                value: &billing_project_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "binaryAuthorizationCustomEndpoint".into(),
                value: &binary_authorization_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "blockchainNodeEngineCustomEndpoint".into(),
                value: &blockchain_node_engine_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "certificateManagerCustomEndpoint".into(),
                value: &certificate_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudAssetCustomEndpoint".into(),
                value: &cloud_asset_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudBillingCustomEndpoint".into(),
                value: &cloud_billing_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudBuildCustomEndpoint".into(),
                value: &cloud_build_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudBuildWorkerPoolCustomEndpoint".into(),
                value: &cloud_build_worker_pool_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudFunctionsCustomEndpoint".into(),
                value: &cloud_functions_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudIdentityCustomEndpoint".into(),
                value: &cloud_identity_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudIdsCustomEndpoint".into(),
                value: &cloud_ids_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudQuotasCustomEndpoint".into(),
                value: &cloud_quotas_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudResourceManagerCustomEndpoint".into(),
                value: &cloud_resource_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudRunCustomEndpoint".into(),
                value: &cloud_run_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudRunV2CustomEndpoint".into(),
                value: &cloud_run_v2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudSchedulerCustomEndpoint".into(),
                value: &cloud_scheduler_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudTasksCustomEndpoint".into(),
                value: &cloud_tasks_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudbuildv2CustomEndpoint".into(),
                value: &cloudbuildv2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clouddeployCustomEndpoint".into(),
                value: &clouddeploy_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "clouddomainsCustomEndpoint".into(),
                value: &clouddomains_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "cloudfunctions2CustomEndpoint".into(),
                value: &cloudfunctions2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "composerCustomEndpoint".into(),
                value: &composer_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "computeCustomEndpoint".into(),
                value: &compute_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "containerAnalysisCustomEndpoint".into(),
                value: &container_analysis_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "containerAttachedCustomEndpoint".into(),
                value: &container_attached_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "containerAwsCustomEndpoint".into(),
                value: &container_aws_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "containerAzureCustomEndpoint".into(),
                value: &container_azure_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "containerCustomEndpoint".into(),
                value: &container_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "coreBillingCustomEndpoint".into(),
                value: &core_billing_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "credentials".into(),
                value: &credentials_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataCatalogCustomEndpoint".into(),
                value: &data_catalog_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataFusionCustomEndpoint".into(),
                value: &data_fusion_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataLossPreventionCustomEndpoint".into(),
                value: &data_loss_prevention_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataPipelineCustomEndpoint".into(),
                value: &data_pipeline_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "databaseMigrationServiceCustomEndpoint".into(),
                value: &database_migration_service_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataflowCustomEndpoint".into(),
                value: &dataflow_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataformCustomEndpoint".into(),
                value: &dataform_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataplexCustomEndpoint".into(),
                value: &dataplex_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataprocCustomEndpoint".into(),
                value: &dataproc_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataprocGdcCustomEndpoint".into(),
                value: &dataproc_gdc_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dataprocMetastoreCustomEndpoint".into(),
                value: &dataproc_metastore_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "datastreamCustomEndpoint".into(),
                value: &datastream_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "defaultLabels".into(),
                value: &default_labels_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "deploymentManagerCustomEndpoint".into(),
                value: &deployment_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "developerConnectCustomEndpoint".into(),
                value: &developer_connect_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dialogflowCustomEndpoint".into(),
                value: &dialogflow_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dialogflowCxCustomEndpoint".into(),
                value: &dialogflow_cx_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "disableGooglePartnerName".into(),
                value: &disable_google_partner_name_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "discoveryEngineCustomEndpoint".into(),
                value: &discovery_engine_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "dnsCustomEndpoint".into(),
                value: &dns_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "documentAiCustomEndpoint".into(),
                value: &document_ai_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "documentAiWarehouseCustomEndpoint".into(),
                value: &document_ai_warehouse_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "edgecontainerCustomEndpoint".into(),
                value: &edgecontainer_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "edgenetworkCustomEndpoint".into(),
                value: &edgenetwork_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "essentialContactsCustomEndpoint".into(),
                value: &essential_contacts_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "eventarcCustomEndpoint".into(),
                value: &eventarc_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "filestoreCustomEndpoint".into(),
                value: &filestore_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseAppCheckCustomEndpoint".into(),
                value: &firebase_app_check_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseCustomEndpoint".into(),
                value: &firebase_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseDatabaseCustomEndpoint".into(),
                value: &firebase_database_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseExtensionsCustomEndpoint".into(),
                value: &firebase_extensions_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseHostingCustomEndpoint".into(),
                value: &firebase_hosting_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaseStorageCustomEndpoint".into(),
                value: &firebase_storage_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firebaserulesCustomEndpoint".into(),
                value: &firebaserules_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "firestoreCustomEndpoint".into(),
                value: &firestore_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "geminiCustomEndpoint".into(),
                value: &gemini_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "gkeBackupCustomEndpoint".into(),
                value: &gke_backup_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "gkeHub2CustomEndpoint".into(),
                value: &gke_hub2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "gkeHubCustomEndpoint".into(),
                value: &gke_hub_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "gkehubFeatureCustomEndpoint".into(),
                value: &gkehub_feature_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "gkeonpremCustomEndpoint".into(),
                value: &gkeonprem_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "googlePartnerName".into(),
                value: &google_partner_name_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "healthcareCustomEndpoint".into(),
                value: &healthcare_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iam2CustomEndpoint".into(),
                value: &iam2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iam3CustomEndpoint".into(),
                value: &iam3_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iamBetaCustomEndpoint".into(),
                value: &iam_beta_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iamCredentialsCustomEndpoint".into(),
                value: &iam_credentials_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iamCustomEndpoint".into(),
                value: &iam_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iamWorkforcePoolCustomEndpoint".into(),
                value: &iam_workforce_pool_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "iapCustomEndpoint".into(),
                value: &iap_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "identityPlatformCustomEndpoint".into(),
                value: &identity_platform_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "impersonateServiceAccount".into(),
                value: &impersonate_service_account_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "impersonateServiceAccountDelegates".into(),
                value: &impersonate_service_account_delegates_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "integrationConnectorsCustomEndpoint".into(),
                value: &integration_connectors_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "integrationsCustomEndpoint".into(),
                value: &integrations_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "kmsCustomEndpoint".into(),
                value: &kms_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "loggingCustomEndpoint".into(),
                value: &logging_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "lookerCustomEndpoint".into(),
                value: &looker_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "managedKafkaCustomEndpoint".into(),
                value: &managed_kafka_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "memcacheCustomEndpoint".into(),
                value: &memcache_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "memorystoreCustomEndpoint".into(),
                value: &memorystore_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "migrationCenterCustomEndpoint".into(),
                value: &migration_center_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "mlEngineCustomEndpoint".into(),
                value: &ml_engine_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "monitoringCustomEndpoint".into(),
                value: &monitoring_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "netappCustomEndpoint".into(),
                value: &netapp_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "networkConnectivityCustomEndpoint".into(),
                value: &network_connectivity_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "networkManagementCustomEndpoint".into(),
                value: &network_management_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "networkSecurityCustomEndpoint".into(),
                value: &network_security_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "networkServicesCustomEndpoint".into(),
                value: &network_services_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "notebooksCustomEndpoint".into(),
                value: &notebooks_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "oracleDatabaseCustomEndpoint".into(),
                value: &oracle_database_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "orgPolicyCustomEndpoint".into(),
                value: &org_policy_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "osConfigCustomEndpoint".into(),
                value: &os_config_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "osLoginCustomEndpoint".into(),
                value: &os_login_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "parallelstoreCustomEndpoint".into(),
                value: &parallelstore_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "privatecaCustomEndpoint".into(),
                value: &privateca_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "privilegedAccessManagerCustomEndpoint".into(),
                value: &privileged_access_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "project".into(),
                value: &project_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "publicCaCustomEndpoint".into(),
                value: &public_ca_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "pubsubCustomEndpoint".into(),
                value: &pubsub_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "pubsubLiteCustomEndpoint".into(),
                value: &pubsub_lite_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "pulumiAttributionLabelAdditionStrategy".into(),
                value: &pulumi_attribution_label_addition_strategy_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "recaptchaEnterpriseCustomEndpoint".into(),
                value: &recaptcha_enterprise_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "redisCustomEndpoint".into(),
                value: &redis_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "region".into(),
                value: &region_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "requestReason".into(),
                value: &request_reason_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "requestTimeout".into(),
                value: &request_timeout_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "resourceManagerCustomEndpoint".into(),
                value: &resource_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "resourceManagerV3CustomEndpoint".into(),
                value: &resource_manager_v3_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "runtimeConfigCustomEndpoint".into(),
                value: &runtime_config_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "runtimeconfigCustomEndpoint".into(),
                value: &runtimeconfig_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "scopes".into(),
                value: &scopes_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretManagerCustomEndpoint".into(),
                value: &secret_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretManagerRegionalCustomEndpoint".into(),
                value: &secret_manager_regional_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secureSourceManagerCustomEndpoint".into(),
                value: &secure_source_manager_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "securityCenterCustomEndpoint".into(),
                value: &security_center_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "securityCenterManagementCustomEndpoint".into(),
                value: &security_center_management_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "securityCenterV2CustomEndpoint".into(),
                value: &security_center_v2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "securityScannerCustomEndpoint".into(),
                value: &security_scanner_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "securitypostureCustomEndpoint".into(),
                value: &securityposture_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "serviceDirectoryCustomEndpoint".into(),
                value: &service_directory_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "serviceManagementCustomEndpoint".into(),
                value: &service_management_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "serviceNetworkingCustomEndpoint".into(),
                value: &service_networking_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "serviceUsageCustomEndpoint".into(),
                value: &service_usage_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "siteVerificationCustomEndpoint".into(),
                value: &site_verification_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "sourceRepoCustomEndpoint".into(),
                value: &source_repo_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "spannerCustomEndpoint".into(),
                value: &spanner_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "sqlCustomEndpoint".into(),
                value: &sql_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "storageCustomEndpoint".into(),
                value: &storage_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "storageInsightsCustomEndpoint".into(),
                value: &storage_insights_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "storageTransferCustomEndpoint".into(),
                value: &storage_transfer_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tagsCustomEndpoint".into(),
                value: &tags_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tagsLocationCustomEndpoint".into(),
                value: &tags_location_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tpuCustomEndpoint".into(),
                value: &tpu_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tpuV2CustomEndpoint".into(),
                value: &tpu_v2_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "transcoderCustomEndpoint".into(),
                value: &transcoder_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "universeDomain".into(),
                value: &universe_domain_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "userProjectOverride".into(),
                value: &user_project_override_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "vertexAiCustomEndpoint".into(),
                value: &vertex_ai_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "vmwareengineCustomEndpoint".into(),
                value: &vmwareengine_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "vpcAccessCustomEndpoint".into(),
                value: &vpc_access_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "workbenchCustomEndpoint".into(),
                value: &workbench_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "workflowsCustomEndpoint".into(),
                value: &workflows_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "workstationsCustomEndpoint".into(),
                value: &workstations_custom_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "zone".into(),
                value: &zone_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        access_approval_custom_endpoint: o.get_field("accessApprovalCustomEndpoint"),
        access_context_manager_custom_endpoint: o
            .get_field("accessContextManagerCustomEndpoint"),
        access_token: o.get_field("accessToken"),
        active_directory_custom_endpoint: o.get_field("activeDirectoryCustomEndpoint"),
        add_pulumi_attribution_label: o.get_field("addPulumiAttributionLabel"),
        alloydb_custom_endpoint: o.get_field("alloydbCustomEndpoint"),
        api_gateway_custom_endpoint: o.get_field("apiGatewayCustomEndpoint"),
        apigee_custom_endpoint: o.get_field("apigeeCustomEndpoint"),
        apikeys_custom_endpoint: o.get_field("apikeysCustomEndpoint"),
        app_engine_custom_endpoint: o.get_field("appEngineCustomEndpoint"),
        apphub_custom_endpoint: o.get_field("apphubCustomEndpoint"),
        artifact_registry_custom_endpoint: o.get_field("artifactRegistryCustomEndpoint"),
        assured_workloads_custom_endpoint: o.get_field("assuredWorkloadsCustomEndpoint"),
        backup_dr_custom_endpoint: o.get_field("backupDrCustomEndpoint"),
        batching: o.get_field("batching"),
        beyondcorp_custom_endpoint: o.get_field("beyondcorpCustomEndpoint"),
        big_query_custom_endpoint: o.get_field("bigQueryCustomEndpoint"),
        biglake_custom_endpoint: o.get_field("biglakeCustomEndpoint"),
        bigquery_analytics_hub_custom_endpoint: o
            .get_field("bigqueryAnalyticsHubCustomEndpoint"),
        bigquery_connection_custom_endpoint: o
            .get_field("bigqueryConnectionCustomEndpoint"),
        bigquery_data_transfer_custom_endpoint: o
            .get_field("bigqueryDataTransferCustomEndpoint"),
        bigquery_datapolicy_custom_endpoint: o
            .get_field("bigqueryDatapolicyCustomEndpoint"),
        bigquery_reservation_custom_endpoint: o
            .get_field("bigqueryReservationCustomEndpoint"),
        bigtable_custom_endpoint: o.get_field("bigtableCustomEndpoint"),
        billing_custom_endpoint: o.get_field("billingCustomEndpoint"),
        billing_project: o.get_field("billingProject"),
        binary_authorization_custom_endpoint: o
            .get_field("binaryAuthorizationCustomEndpoint"),
        blockchain_node_engine_custom_endpoint: o
            .get_field("blockchainNodeEngineCustomEndpoint"),
        certificate_manager_custom_endpoint: o
            .get_field("certificateManagerCustomEndpoint"),
        cloud_asset_custom_endpoint: o.get_field("cloudAssetCustomEndpoint"),
        cloud_billing_custom_endpoint: o.get_field("cloudBillingCustomEndpoint"),
        cloud_build_custom_endpoint: o.get_field("cloudBuildCustomEndpoint"),
        cloud_build_worker_pool_custom_endpoint: o
            .get_field("cloudBuildWorkerPoolCustomEndpoint"),
        cloud_functions_custom_endpoint: o.get_field("cloudFunctionsCustomEndpoint"),
        cloud_identity_custom_endpoint: o.get_field("cloudIdentityCustomEndpoint"),
        cloud_ids_custom_endpoint: o.get_field("cloudIdsCustomEndpoint"),
        cloud_quotas_custom_endpoint: o.get_field("cloudQuotasCustomEndpoint"),
        cloud_resource_manager_custom_endpoint: o
            .get_field("cloudResourceManagerCustomEndpoint"),
        cloud_run_custom_endpoint: o.get_field("cloudRunCustomEndpoint"),
        cloud_run_v2_custom_endpoint: o.get_field("cloudRunV2CustomEndpoint"),
        cloud_scheduler_custom_endpoint: o.get_field("cloudSchedulerCustomEndpoint"),
        cloud_tasks_custom_endpoint: o.get_field("cloudTasksCustomEndpoint"),
        cloudbuildv2_custom_endpoint: o.get_field("cloudbuildv2CustomEndpoint"),
        clouddeploy_custom_endpoint: o.get_field("clouddeployCustomEndpoint"),
        clouddomains_custom_endpoint: o.get_field("clouddomainsCustomEndpoint"),
        cloudfunctions2_custom_endpoint: o.get_field("cloudfunctions2CustomEndpoint"),
        composer_custom_endpoint: o.get_field("composerCustomEndpoint"),
        compute_custom_endpoint: o.get_field("computeCustomEndpoint"),
        container_analysis_custom_endpoint: o
            .get_field("containerAnalysisCustomEndpoint"),
        container_attached_custom_endpoint: o
            .get_field("containerAttachedCustomEndpoint"),
        container_aws_custom_endpoint: o.get_field("containerAwsCustomEndpoint"),
        container_azure_custom_endpoint: o.get_field("containerAzureCustomEndpoint"),
        container_custom_endpoint: o.get_field("containerCustomEndpoint"),
        core_billing_custom_endpoint: o.get_field("coreBillingCustomEndpoint"),
        credentials: o.get_field("credentials"),
        data_catalog_custom_endpoint: o.get_field("dataCatalogCustomEndpoint"),
        data_fusion_custom_endpoint: o.get_field("dataFusionCustomEndpoint"),
        data_loss_prevention_custom_endpoint: o
            .get_field("dataLossPreventionCustomEndpoint"),
        data_pipeline_custom_endpoint: o.get_field("dataPipelineCustomEndpoint"),
        database_migration_service_custom_endpoint: o
            .get_field("databaseMigrationServiceCustomEndpoint"),
        dataflow_custom_endpoint: o.get_field("dataflowCustomEndpoint"),
        dataform_custom_endpoint: o.get_field("dataformCustomEndpoint"),
        dataplex_custom_endpoint: o.get_field("dataplexCustomEndpoint"),
        dataproc_custom_endpoint: o.get_field("dataprocCustomEndpoint"),
        dataproc_gdc_custom_endpoint: o.get_field("dataprocGdcCustomEndpoint"),
        dataproc_metastore_custom_endpoint: o
            .get_field("dataprocMetastoreCustomEndpoint"),
        datastream_custom_endpoint: o.get_field("datastreamCustomEndpoint"),
        default_labels: o.get_field("defaultLabels"),
        deployment_manager_custom_endpoint: o
            .get_field("deploymentManagerCustomEndpoint"),
        developer_connect_custom_endpoint: o.get_field("developerConnectCustomEndpoint"),
        dialogflow_custom_endpoint: o.get_field("dialogflowCustomEndpoint"),
        dialogflow_cx_custom_endpoint: o.get_field("dialogflowCxCustomEndpoint"),
        disable_google_partner_name: o.get_field("disableGooglePartnerName"),
        discovery_engine_custom_endpoint: o.get_field("discoveryEngineCustomEndpoint"),
        dns_custom_endpoint: o.get_field("dnsCustomEndpoint"),
        document_ai_custom_endpoint: o.get_field("documentAiCustomEndpoint"),
        document_ai_warehouse_custom_endpoint: o
            .get_field("documentAiWarehouseCustomEndpoint"),
        edgecontainer_custom_endpoint: o.get_field("edgecontainerCustomEndpoint"),
        edgenetwork_custom_endpoint: o.get_field("edgenetworkCustomEndpoint"),
        essential_contacts_custom_endpoint: o
            .get_field("essentialContactsCustomEndpoint"),
        eventarc_custom_endpoint: o.get_field("eventarcCustomEndpoint"),
        filestore_custom_endpoint: o.get_field("filestoreCustomEndpoint"),
        firebase_app_check_custom_endpoint: o
            .get_field("firebaseAppCheckCustomEndpoint"),
        firebase_custom_endpoint: o.get_field("firebaseCustomEndpoint"),
        firebase_database_custom_endpoint: o.get_field("firebaseDatabaseCustomEndpoint"),
        firebase_extensions_custom_endpoint: o
            .get_field("firebaseExtensionsCustomEndpoint"),
        firebase_hosting_custom_endpoint: o.get_field("firebaseHostingCustomEndpoint"),
        firebase_storage_custom_endpoint: o.get_field("firebaseStorageCustomEndpoint"),
        firebaserules_custom_endpoint: o.get_field("firebaserulesCustomEndpoint"),
        firestore_custom_endpoint: o.get_field("firestoreCustomEndpoint"),
        gemini_custom_endpoint: o.get_field("geminiCustomEndpoint"),
        gke_backup_custom_endpoint: o.get_field("gkeBackupCustomEndpoint"),
        gke_hub2_custom_endpoint: o.get_field("gkeHub2CustomEndpoint"),
        gke_hub_custom_endpoint: o.get_field("gkeHubCustomEndpoint"),
        gkehub_feature_custom_endpoint: o.get_field("gkehubFeatureCustomEndpoint"),
        gkeonprem_custom_endpoint: o.get_field("gkeonpremCustomEndpoint"),
        google_partner_name: o.get_field("googlePartnerName"),
        healthcare_custom_endpoint: o.get_field("healthcareCustomEndpoint"),
        iam2_custom_endpoint: o.get_field("iam2CustomEndpoint"),
        iam3_custom_endpoint: o.get_field("iam3CustomEndpoint"),
        iam_beta_custom_endpoint: o.get_field("iamBetaCustomEndpoint"),
        iam_credentials_custom_endpoint: o.get_field("iamCredentialsCustomEndpoint"),
        iam_custom_endpoint: o.get_field("iamCustomEndpoint"),
        iam_workforce_pool_custom_endpoint: o
            .get_field("iamWorkforcePoolCustomEndpoint"),
        iap_custom_endpoint: o.get_field("iapCustomEndpoint"),
        identity_platform_custom_endpoint: o.get_field("identityPlatformCustomEndpoint"),
        impersonate_service_account: o.get_field("impersonateServiceAccount"),
        impersonate_service_account_delegates: o
            .get_field("impersonateServiceAccountDelegates"),
        integration_connectors_custom_endpoint: o
            .get_field("integrationConnectorsCustomEndpoint"),
        integrations_custom_endpoint: o.get_field("integrationsCustomEndpoint"),
        kms_custom_endpoint: o.get_field("kmsCustomEndpoint"),
        logging_custom_endpoint: o.get_field("loggingCustomEndpoint"),
        looker_custom_endpoint: o.get_field("lookerCustomEndpoint"),
        managed_kafka_custom_endpoint: o.get_field("managedKafkaCustomEndpoint"),
        memcache_custom_endpoint: o.get_field("memcacheCustomEndpoint"),
        memorystore_custom_endpoint: o.get_field("memorystoreCustomEndpoint"),
        migration_center_custom_endpoint: o.get_field("migrationCenterCustomEndpoint"),
        ml_engine_custom_endpoint: o.get_field("mlEngineCustomEndpoint"),
        monitoring_custom_endpoint: o.get_field("monitoringCustomEndpoint"),
        netapp_custom_endpoint: o.get_field("netappCustomEndpoint"),
        network_connectivity_custom_endpoint: o
            .get_field("networkConnectivityCustomEndpoint"),
        network_management_custom_endpoint: o
            .get_field("networkManagementCustomEndpoint"),
        network_security_custom_endpoint: o.get_field("networkSecurityCustomEndpoint"),
        network_services_custom_endpoint: o.get_field("networkServicesCustomEndpoint"),
        notebooks_custom_endpoint: o.get_field("notebooksCustomEndpoint"),
        oracle_database_custom_endpoint: o.get_field("oracleDatabaseCustomEndpoint"),
        org_policy_custom_endpoint: o.get_field("orgPolicyCustomEndpoint"),
        os_config_custom_endpoint: o.get_field("osConfigCustomEndpoint"),
        os_login_custom_endpoint: o.get_field("osLoginCustomEndpoint"),
        parallelstore_custom_endpoint: o.get_field("parallelstoreCustomEndpoint"),
        privateca_custom_endpoint: o.get_field("privatecaCustomEndpoint"),
        privileged_access_manager_custom_endpoint: o
            .get_field("privilegedAccessManagerCustomEndpoint"),
        project: o.get_field("project"),
        public_ca_custom_endpoint: o.get_field("publicCaCustomEndpoint"),
        pubsub_custom_endpoint: o.get_field("pubsubCustomEndpoint"),
        pubsub_lite_custom_endpoint: o.get_field("pubsubLiteCustomEndpoint"),
        pulumi_attribution_label_addition_strategy: o
            .get_field("pulumiAttributionLabelAdditionStrategy"),
        recaptcha_enterprise_custom_endpoint: o
            .get_field("recaptchaEnterpriseCustomEndpoint"),
        redis_custom_endpoint: o.get_field("redisCustomEndpoint"),
        region: o.get_field("region"),
        request_reason: o.get_field("requestReason"),
        request_timeout: o.get_field("requestTimeout"),
        resource_manager_custom_endpoint: o.get_field("resourceManagerCustomEndpoint"),
        resource_manager_v3_custom_endpoint: o
            .get_field("resourceManagerV3CustomEndpoint"),
        runtime_config_custom_endpoint: o.get_field("runtimeConfigCustomEndpoint"),
        runtimeconfig_custom_endpoint: o.get_field("runtimeconfigCustomEndpoint"),
        scopes: o.get_field("scopes"),
        secret_manager_custom_endpoint: o.get_field("secretManagerCustomEndpoint"),
        secret_manager_regional_custom_endpoint: o
            .get_field("secretManagerRegionalCustomEndpoint"),
        secure_source_manager_custom_endpoint: o
            .get_field("secureSourceManagerCustomEndpoint"),
        security_center_custom_endpoint: o.get_field("securityCenterCustomEndpoint"),
        security_center_management_custom_endpoint: o
            .get_field("securityCenterManagementCustomEndpoint"),
        security_center_v2_custom_endpoint: o
            .get_field("securityCenterV2CustomEndpoint"),
        security_scanner_custom_endpoint: o.get_field("securityScannerCustomEndpoint"),
        securityposture_custom_endpoint: o.get_field("securitypostureCustomEndpoint"),
        service_directory_custom_endpoint: o.get_field("serviceDirectoryCustomEndpoint"),
        service_management_custom_endpoint: o
            .get_field("serviceManagementCustomEndpoint"),
        service_networking_custom_endpoint: o
            .get_field("serviceNetworkingCustomEndpoint"),
        service_usage_custom_endpoint: o.get_field("serviceUsageCustomEndpoint"),
        site_verification_custom_endpoint: o.get_field("siteVerificationCustomEndpoint"),
        source_repo_custom_endpoint: o.get_field("sourceRepoCustomEndpoint"),
        spanner_custom_endpoint: o.get_field("spannerCustomEndpoint"),
        sql_custom_endpoint: o.get_field("sqlCustomEndpoint"),
        storage_custom_endpoint: o.get_field("storageCustomEndpoint"),
        storage_insights_custom_endpoint: o.get_field("storageInsightsCustomEndpoint"),
        storage_transfer_custom_endpoint: o.get_field("storageTransferCustomEndpoint"),
        tags_custom_endpoint: o.get_field("tagsCustomEndpoint"),
        tags_location_custom_endpoint: o.get_field("tagsLocationCustomEndpoint"),
        tpu_custom_endpoint: o.get_field("tpuCustomEndpoint"),
        tpu_v2_custom_endpoint: o.get_field("tpuV2CustomEndpoint"),
        transcoder_custom_endpoint: o.get_field("transcoderCustomEndpoint"),
        universe_domain: o.get_field("universeDomain"),
        user_project_override: o.get_field("userProjectOverride"),
        vertex_ai_custom_endpoint: o.get_field("vertexAiCustomEndpoint"),
        vmwareengine_custom_endpoint: o.get_field("vmwareengineCustomEndpoint"),
        vpc_access_custom_endpoint: o.get_field("vpcAccessCustomEndpoint"),
        workbench_custom_endpoint: o.get_field("workbenchCustomEndpoint"),
        workflows_custom_endpoint: o.get_field("workflowsCustomEndpoint"),
        workstations_custom_endpoint: o.get_field("workstationsCustomEndpoint"),
        zone: o.get_field("zone"),
    }
}
