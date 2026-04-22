/// Manages the Defender for Storage.
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
///             .location("westus2")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("exampleacc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStorageDefender = storage_defender::create(
///         "exampleStorageDefender",
///         StorageDefenderArgs::builder()
///             .storage_account_id("${exampleAccount.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The setting can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/storageDefender:StorageDefender example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Storage/storageAccounts/storageacc
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod storage_defender {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StorageDefenderArgs {
        /// The max GB to be scanned per Month. Must be `-1` or above `0`. Omit this property or set to `-1` if no capping is needed. Defaults to `-1`.
        #[builder(into, default)]
        pub malware_scanning_on_upload_cap_gb_per_month: pulumi_gestalt_rust::Input<
            Option<i32>,
        >,
        /// Whether On Upload malware scanning should be enabled. Defaults to `false`.
        #[builder(into, default)]
        pub malware_scanning_on_upload_enabled: pulumi_gestalt_rust::Input<
            Option<bool>,
        >,
        /// Whether the settings defined for this storage account should override the settings defined for the subscription. Defaults to `false`.
        #[builder(into, default)]
        pub override_subscription_settings_enabled: pulumi_gestalt_rust::Input<
            Option<bool>,
        >,
        /// The Event Grid Topic where every scan result will be sent to. When you set an Event Grid custom topic, you must set `override_subscription_settings_enabled` to `true` to override the subscription-level settings.
        #[builder(into, default)]
        pub scan_results_event_grid_topic_id: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// Whether Sensitive Data Discovery should be enabled. Defaults to `false`.
        #[builder(into, default)]
        pub sensitive_data_discovery_enabled: pulumi_gestalt_rust::Input<
            Option<bool>,
        >,
        /// The ID of the storage account the defender applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct StorageDefenderResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The max GB to be scanned per Month. Must be `-1` or above `0`. Omit this property or set to `-1` if no capping is needed. Defaults to `-1`.
        pub malware_scanning_on_upload_cap_gb_per_month: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// Whether On Upload malware scanning should be enabled. Defaults to `false`.
        pub malware_scanning_on_upload_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether the settings defined for this storage account should override the settings defined for the subscription. Defaults to `false`.
        pub override_subscription_settings_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The Event Grid Topic where every scan result will be sent to. When you set an Event Grid custom topic, you must set `override_subscription_settings_enabled` to `true` to override the subscription-level settings.
        pub scan_results_event_grid_topic_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Whether Sensitive Data Discovery should be enabled. Defaults to `false`.
        pub sensitive_data_discovery_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the storage account the defender applied to. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StorageDefenderArgs,
    ) -> StorageDefenderResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StorageDefenderArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> StorageDefenderResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StorageDefenderArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> StorageDefenderResult {
        let malware_scanning_on_upload_cap_gb_per_month_binding = args
            .malware_scanning_on_upload_cap_gb_per_month
            .get_output(ctx);
        let malware_scanning_on_upload_enabled_binding = args
            .malware_scanning_on_upload_enabled
            .get_output(ctx);
        let override_subscription_settings_enabled_binding = args
            .override_subscription_settings_enabled
            .get_output(ctx);
        let scan_results_event_grid_topic_id_binding = args
            .scan_results_event_grid_topic_id
            .get_output(ctx);
        let sensitive_data_discovery_enabled_binding = args
            .sensitive_data_discovery_enabled
            .get_output(ctx);
        let storage_account_id_binding = args.storage_account_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/storageDefender:StorageDefender".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "malwareScanningOnUploadCapGbPerMonth".into(),
                    value: &malware_scanning_on_upload_cap_gb_per_month_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "malwareScanningOnUploadEnabled".into(),
                    value: &malware_scanning_on_upload_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideSubscriptionSettingsEnabled".into(),
                    value: &override_subscription_settings_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scanResultsEventGridTopicId".into(),
                    value: &scan_results_event_grid_topic_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sensitiveDataDiscoveryEnabled".into(),
                    value: &sensitive_data_discovery_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        StorageDefenderResult {
            id: o.get_id(),
            urn: o.get_urn(),
            malware_scanning_on_upload_cap_gb_per_month: o
                .get_field("malwareScanningOnUploadCapGbPerMonth"),
            malware_scanning_on_upload_enabled: o
                .get_field("malwareScanningOnUploadEnabled"),
            override_subscription_settings_enabled: o
                .get_field("overrideSubscriptionSettingsEnabled"),
            scan_results_event_grid_topic_id: o.get_field("scanResultsEventGridTopicId"),
            sensitive_data_discovery_enabled: o
                .get_field("sensitiveDataDiscoveryEnabled"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
