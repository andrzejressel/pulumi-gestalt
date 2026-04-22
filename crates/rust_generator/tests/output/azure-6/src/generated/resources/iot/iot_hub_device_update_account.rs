/// Manages an IoT Hub Device Update Account.
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
///       identity:
///         type: SystemAssigned
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// IoT Hub Device Update Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubDeviceUpdateAccount:IotHubDeviceUpdateAccount example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DeviceUpdate/accounts/account1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod iot_hub_device_update_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateAccountArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::Input<
            Option<super::super::types::iot::IotHubDeviceUpdateAccountIdentity>,
        >,
        /// Specifies the Azure Region where the IoT Hub Device Update Account should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies the name which should be used for this IoT Hub Device Update Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Specifies whether the public network access is enabled for the IoT Hub Device Update Account. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Specifies the name of the Resource Group where the IoT Hub Device Update Account should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// Sku of the IoT Hub Device Update Account. Possible values are `Free` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::Input<Option<String>>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Account.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateAccountResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The API host name of the IoT Hub Device Update Account.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::IotHubDeviceUpdateAccountIdentity>,
        >,
        /// Specifies the Azure Region where the IoT Hub Device Update Account should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this IoT Hub Device Update Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the public network access is enabled for the IoT Hub Device Update Account. Possible values are `true` and `false`. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the Resource Group where the IoT Hub Device Update Account should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Sku of the IoT Hub Device Update Account. Possible values are `Free` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Account.
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
        args: IotHubDeviceUpdateAccountArgs,
    ) -> IotHubDeviceUpdateAccountResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubDeviceUpdateAccountArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IotHubDeviceUpdateAccountResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IotHubDeviceUpdateAccountArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IotHubDeviceUpdateAccountResult {
        let identity_binding = args.identity.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let sku_binding = args.sku.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/iotHubDeviceUpdateAccount:IotHubDeviceUpdateAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
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
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IotHubDeviceUpdateAccountResult {
            id: o.get_id(),
            urn: o.get_urn(),
            host_name: o.get_field("hostName"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
