/// Manages an IoT Hub Device Update Instance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: East US
///   exampleIotHubDeviceUpdateAccount:
///     type: azure:iot:IotHubDeviceUpdateAccount
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleIotHubDeviceUpdateInstance:
///     type: azure:iot:IotHubDeviceUpdateInstance
///     name: example
///     properties:
///       name: example
///       deviceUpdateAccountId: ${exampleIotHubDeviceUpdateAccount.id}
///       iothubId: ${exampleIoTHub.id}
///       diagnosticEnabled: true
///       diagnosticStorageAccount:
///         connectionString: ${exampleAccount.primaryConnectionString}
///         id: ${exampleAccount.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// IoT Hub Device Update Instance can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubDeviceUpdateInstance:IotHubDeviceUpdateInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DeviceUpdate/accounts/account1/instances/instance1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod iot_hub_device_update_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateInstanceArgs {
        /// Specifies the ID of the IoT Hub Device Update Account where the IoT Hub Device Update Instance exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub device_update_account_id: pulumi_gestalt_rust::Input<String>,
        /// Whether the diagnostic log collection is enabled. Possible values are `true` and `false`. Defaults to `false`.
        #[builder(into, default)]
        pub diagnostic_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// A `diagnostic_storage_account` block as defined below.
        #[builder(into, default)]
        pub diagnostic_storage_account: pulumi_gestalt_rust::Input<
            Option<
                super::super::types::iot::IotHubDeviceUpdateInstanceDiagnosticStorageAccount,
            >,
        >,
        /// Specifies the ID of the IoT Hub associated with the IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name which should be used for this IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateInstanceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the IoT Hub Device Update Account where the IoT Hub Device Update Instance exists. Changing this forces a new resource to be created.
        pub device_update_account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the diagnostic log collection is enabled. Possible values are `true` and `false`. Defaults to `false`.
        pub diagnostic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `diagnostic_storage_account` block as defined below.
        pub diagnostic_storage_account: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::iot::IotHubDeviceUpdateInstanceDiagnosticStorageAccount,
            >,
        >,
        /// Specifies the ID of the IoT Hub associated with the IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Instance.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubDeviceUpdateInstanceArgs,
    ) -> IotHubDeviceUpdateInstanceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubDeviceUpdateInstanceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IotHubDeviceUpdateInstanceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubDeviceUpdateInstanceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IotHubDeviceUpdateInstanceResult {
        let device_update_account_id_binding = args
            .device_update_account_id
            .get_output(ctx);
        let diagnostic_enabled_binding = args.diagnostic_enabled.get_output(ctx);
        let diagnostic_storage_account_binding = args
            .diagnostic_storage_account
            .get_output(ctx);
        let iothub_id_binding = args.iothub_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/iotHubDeviceUpdateInstance:IotHubDeviceUpdateInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceUpdateAccountId".into(),
                    value: &device_update_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diagnosticEnabled".into(),
                    value: &diagnostic_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diagnosticStorageAccount".into(),
                    value: &diagnostic_storage_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IotHubDeviceUpdateInstanceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            device_update_account_id: o.get_field("deviceUpdateAccountId"),
            diagnostic_enabled: o.get_field("diagnosticEnabled"),
            diagnostic_storage_account: o.get_field("diagnosticStorageAccount"),
            iothub_id: o.get_field("iothubId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
