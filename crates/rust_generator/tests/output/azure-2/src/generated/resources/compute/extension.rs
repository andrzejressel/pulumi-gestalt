/// Manages a Virtual Machine Extension to provide post deployment configuration
/// and run automated tasks.
///
/// > **NOTE:** Custom Script Extensions for Linux & Windows require that the `commandToExecute` returns a `0` exit code to be classified as successfully deployed. You can achieve this by appending `exit 0` to the end of your `commandToExecute`.
///
/// > **NOTE:** Custom Script Extensions require that the Azure Virtual Machine Guest Agent is running on the Virtual Machine.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: acctni
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: testconfiguration1
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-machine
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleExtension:
///     type: azure:compute:Extension
///     name: example
///     properties:
///       name: hostname
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       publisher: Microsoft.Azure.Extensions
///       type: CustomScript
///       typeHandlerVersion: '2.0'
///       settings: |2
///          {
///           "commandToExecute": "hostname && uptime"
///          }
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Virtual Machine Extensions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/extension:Extension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/myVM/extensions/extensionName
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionArgs {
        /// Specifies if the platform deploys the latest minor version update to the `type_handler_version` specified.
        #[builder(into, default)]
        pub auto_upgrade_minor_version: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        #[builder(into, default)]
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        #[builder(into, default)]
        pub failure_suppression_enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The name of the virtual machine extension peering. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The protected_settings passed to the extension, like settings, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `protected_settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        #[builder(into, default)]
        pub protected_settings: pulumi_gestalt_rust::Input<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        #[builder(into, default)]
        pub protected_settings_from_key_vault: pulumi_gestalt_rust::Input<
            Option<super::super::types::compute::ExtensionProtectedSettingsFromKeyVault>,
        >,
        /// Specifies the collection of extension names after which this extension needs to be provisioned.
        #[builder(into, default)]
        pub provision_after_extensions: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// The publisher of the extension, available publishers can be found by using the Azure CLI. Changing this forces a new resource to be created.
        #[builder(into)]
        pub publisher: pulumi_gestalt_rust::Input<String>,
        /// The settings passed to the extension, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::Input<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of extension, available types for a publisher can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vm extension image list --location westus -o table
        /// ```
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::Input<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        #[builder(into)]
        pub type_handler_version: pulumi_gestalt_rust::Input<String>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ExtensionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the platform deploys the latest minor version update to the `type_handler_version` specified.
        pub auto_upgrade_minor_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the Extension be automatically updated whenever the Publisher releases a new version of this VM Extension?
        pub automatic_upgrade_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should failures from the extension be suppressed? Possible values are `true` or `false`. Defaults to `false`.
        ///
        /// > **NOTE:** Operational failures such as not connecting to the VM will not be suppressed regardless of the `failure_suppression_enabled` value.
        pub failure_suppression_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the virtual machine extension peering. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The protected_settings passed to the extension, like settings, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `protected_settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        pub protected_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `protected_settings_from_key_vault` block as defined below.
        ///
        /// > **Note:** `protected_settings_from_key_vault` cannot be used with `protected_settings`
        pub protected_settings_from_key_vault: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ExtensionProtectedSettingsFromKeyVault>,
        >,
        /// Specifies the collection of extension names after which this extension needs to be provisioned.
        pub provision_after_extensions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The publisher of the extension, available publishers can be found by using the Azure CLI. Changing this forces a new resource to be created.
        pub publisher: pulumi_gestalt_rust::Output<String>,
        /// The settings passed to the extension, these are specified as a JSON object in a string.
        ///
        /// > **Please Note:** Certain VM Extensions require that the keys in the `settings` block are case sensitive. If you're seeing unhelpful errors, please ensure the keys are consistent with how Azure is expecting them (for instance, for the `JsonADDomainExtension` extension, the keys are expected to be in `TitleCase`.)
        pub settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of extension, available types for a publisher can be found using the Azure CLI.
        ///
        /// > **Note:** The `Publisher` and `Type` of Virtual Machine Extensions can be found using the Azure CLI, via:
        ///
        /// ```shell
        /// az vm extension image list --location westus -o table
        /// ```
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of the extension to use, available versions can be found using the Azure CLI.
        pub type_handler_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Machine. Changing this forces a new resource to be created
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExtensionArgs,
    ) -> ExtensionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExtensionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ExtensionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExtensionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ExtensionResult {
        let auto_upgrade_minor_version_binding = args
            .auto_upgrade_minor_version
            .get_output(ctx);
        let automatic_upgrade_enabled_binding = args
            .automatic_upgrade_enabled
            .get_output(ctx);
        let failure_suppression_enabled_binding = args
            .failure_suppression_enabled
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let protected_settings_binding = args.protected_settings.get_output(ctx);
        let protected_settings_from_key_vault_binding = args
            .protected_settings_from_key_vault
            .get_output(ctx);
        let provision_after_extensions_binding = args
            .provision_after_extensions
            .get_output(ctx);
        let publisher_binding = args.publisher.get_output(ctx);
        let settings_binding = args.settings.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let type__binding = args.type_.get_output(ctx);
        let type_handler_version_binding = args.type_handler_version.get_output(ctx);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/extension:Extension".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpgradeMinorVersion".into(),
                    value: &auto_upgrade_minor_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticUpgradeEnabled".into(),
                    value: &automatic_upgrade_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failureSuppressionEnabled".into(),
                    value: &failure_suppression_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectedSettings".into(),
                    value: &protected_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectedSettingsFromKeyVault".into(),
                    value: &protected_settings_from_key_vault_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisionAfterExtensions".into(),
                    value: &provision_after_extensions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: &settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "typeHandlerVersion".into(),
                    value: &type_handler_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ExtensionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            auto_upgrade_minor_version: o.get_field("autoUpgradeMinorVersion"),
            automatic_upgrade_enabled: o.get_field("automaticUpgradeEnabled"),
            failure_suppression_enabled: o.get_field("failureSuppressionEnabled"),
            name: o.get_field("name"),
            protected_settings: o.get_field("protectedSettings"),
            protected_settings_from_key_vault: o
                .get_field("protectedSettingsFromKeyVault"),
            provision_after_extensions: o.get_field("provisionAfterExtensions"),
            publisher: o.get_field("publisher"),
            settings: o.get_field("settings"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            type_handler_version: o.get_field("typeHandlerVersion"),
            virtual_machine_id: o.get_field("virtualMachineId"),
        }
    }
}
