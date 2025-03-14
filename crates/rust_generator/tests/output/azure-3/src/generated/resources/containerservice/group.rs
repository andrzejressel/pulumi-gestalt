/// Manages as an Azure Container Group instance.
///
/// ## Example Usage
///
/// This example provisions a Basic Container.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleGroup:
///     type: azure:containerservice:Group
///     name: example
///     properties:
///       name: example-continst
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipAddressType: Public
///       dnsNameLabel: aci-label
///       osType: Linux
///       containers:
///         - name: hello-world
///           image: mcr.microsoft.com/azuredocs/aci-helloworld:latest
///           cpu: '0.5'
///           memory: '1.5'
///           ports:
///             - port: 443
///               protocol: TCP
///         - name: sidecar
///           image: mcr.microsoft.com/azuredocs/aci-tutorial-sidecar
///           cpu: '0.5'
///           memory: '1.5'
///       tags:
///         environment: testing
/// ```
///
/// ## Import
///
/// Container Group's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/group:Group containerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerInstance/containerGroups/myContainerGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// The definition of a container that is part of the group as documented in the `container` block below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub containers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::containerservice::GroupContainer>,
        >,
        /// A `diagnostics` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub diagnostics: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::GroupDiagnostics>,
        >,
        /// A `dns_config` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::GroupDnsConfig>,
        >,
        /// The DNS label/name for the container group's IP. Changing this forces a new resource to be created.
        ///
        /// > **Note:** DNS label/name is not supported when deploying to virtual networks.
        #[builder(into, default)]
        pub dns_name_label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value representing the security enum. `Noreuse`, `ResourceGroupReuse`, `SubscriptionReuse`, `TenantReuse` or `Unsecure`. Defaults to `Unsecure`.
        #[builder(into, default)]
        pub dns_name_label_reuse_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Zero or more `exposed_port` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `exposed_port` can only contain ports that are also exposed on one or more containers in the group.
        #[builder(into, default)]
        pub exposed_ports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::GroupExposedPort>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::GroupIdentity>,
        >,
        /// An `image_registry_credential` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub image_registry_credentials: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::containerservice::GroupImageRegistryCredential>,
            >,
        >,
        /// The definition of an init container that is part of the group as documented in the `init_container` block below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub init_containers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::GroupInitContainer>>,
        >,
        /// Specifies the IP address type of the container. `Public`, `Private` or `None`. Changing this forces a new resource to be created. If set to `Private`, `subnet_ids` also needs to be set. Defaults to `Public`.
        ///
        /// > **Note:** `dns_name_label` and `os_type` set to `windows` are not compatible with `Private` `ip_address_type`
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Key Vault key URI for CMK encryption. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, the RP principal named "Azure Container Instance Service" will be used instead. Make sure the identity has the proper `key_permissions` set, at least with `Get`, `UnwrapKey`, `WrapKey` and `GetRotationPolicy`.
        #[builder(into, default)]
        pub key_vault_user_assigned_identity_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Container Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub network_profile_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The OS for the container group. Allowed values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** if `os_type` is set to `Windows` currently only a single `container` block is supported. Windows containers are not supported in virtual networks.
        #[builder(into)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority of the Container Group. Possible values are `Regular` and `Spot`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `priority` is set to `Spot`, the `ip_address_type` has to be `None`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Container Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Restart policy for the container group. Allowed values are `Always`, `Never`, `OnFailure`. Defaults to `Always`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub restart_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the sku of the Container Group. Possible values are `Confidential`, `Dedicated` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subnet resource IDs for a container group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this Container Group is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// The definition of a container that is part of the group as documented in the `container` block below. Changing this forces a new resource to be created.
        pub containers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::GroupContainer>,
        >,
        /// A `diagnostics` block as documented below. Changing this forces a new resource to be created.
        pub diagnostics: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::GroupDiagnostics>,
        >,
        /// A `dns_config` block as documented below. Changing this forces a new resource to be created.
        pub dns_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::GroupDnsConfig>,
        >,
        /// The DNS label/name for the container group's IP. Changing this forces a new resource to be created.
        ///
        /// > **Note:** DNS label/name is not supported when deploying to virtual networks.
        pub dns_name_label: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value representing the security enum. `Noreuse`, `ResourceGroupReuse`, `SubscriptionReuse`, `TenantReuse` or `Unsecure`. Defaults to `Unsecure`.
        pub dns_name_label_reuse_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Zero or more `exposed_port` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `exposed_port` can only contain ports that are also exposed on one or more containers in the group.
        pub exposed_ports: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::GroupExposedPort>,
        >,
        /// The FQDN of the container group derived from `dns_name_label`.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::GroupIdentity>,
        >,
        /// An `image_registry_credential` block as documented below. Changing this forces a new resource to be created.
        pub image_registry_credentials: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::containerservice::GroupImageRegistryCredential>,
            >,
        >,
        /// The definition of an init container that is part of the group as documented in the `init_container` block below. Changing this forces a new resource to be created.
        pub init_containers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerservice::GroupInitContainer>>,
        >,
        /// The IP address allocated to the container group.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IP address type of the container. `Public`, `Private` or `None`. Changing this forces a new resource to be created. If set to `Private`, `subnet_ids` also needs to be set. Defaults to `Public`.
        ///
        /// > **Note:** `dns_name_label` and `os_type` set to `windows` are not compatible with `Private` `ip_address_type`
        pub ip_address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Key Vault key URI for CMK encryption. Changing this forces a new resource to be created.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, the RP principal named "Azure Container Instance Service" will be used instead. Make sure the identity has the proper `key_permissions` set, at least with `Get`, `UnwrapKey`, `WrapKey` and `GetRotationPolicy`.
        pub key_vault_user_assigned_identity_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Container Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The OS for the container group. Allowed values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** if `os_type` is set to `Windows` currently only a single `container` block is supported. Windows containers are not supported in virtual networks.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The priority of the Container Group. Possible values are `Regular` and `Spot`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `priority` is set to `Spot`, the `ip_address_type` has to be `None`.
        pub priority: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Container Group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Restart policy for the container group. Allowed values are `Always`, `Never`, `OnFailure`. Defaults to `Always`. Changing this forces a new resource to be created.
        pub restart_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the sku of the Container Group. Possible values are `Confidential`, `Dedicated` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// The subnet resource IDs for a container group. Changing this forces a new resource to be created.
        pub subnet_ids: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this Container Group is located. Changing this forces a new resource to be created.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let containers_binding = args.containers.get_output(context);
        let diagnostics_binding = args.diagnostics.get_output(context);
        let dns_config_binding = args.dns_config.get_output(context);
        let dns_name_label_binding = args.dns_name_label.get_output(context);
        let dns_name_label_reuse_policy_binding = args
            .dns_name_label_reuse_policy
            .get_output(context);
        let exposed_ports_binding = args.exposed_ports.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let image_registry_credentials_binding = args
            .image_registry_credentials
            .get_output(context);
        let init_containers_binding = args.init_containers.get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let key_vault_key_id_binding = args.key_vault_key_id.get_output(context);
        let key_vault_user_assigned_identity_id_binding = args
            .key_vault_user_assigned_identity_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_profile_id_binding = args.network_profile_id.get_output(context);
        let os_type_binding = args.os_type.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let restart_policy_binding = args.restart_policy.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containers".into(),
                    value: &containers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diagnostics".into(),
                    value: &diagnostics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsConfig".into(),
                    value: &dns_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsNameLabel".into(),
                    value: &dns_name_label_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsNameLabelReusePolicy".into(),
                    value: &dns_name_label_reuse_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exposedPorts".into(),
                    value: &exposed_ports_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageRegistryCredentials".into(),
                    value: &image_registry_credentials_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initContainers".into(),
                    value: &init_containers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultUserAssignedIdentityId".into(),
                    value: &key_vault_user_assigned_identity_id_binding.drop_type(),
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
                    name: "networkProfileId".into(),
                    value: &network_profile_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "osType".into(),
                    value: &os_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restartPolicy".into(),
                    value: &restart_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
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
        GroupResult {
            containers: o.get_field("containers"),
            diagnostics: o.get_field("diagnostics"),
            dns_config: o.get_field("dnsConfig"),
            dns_name_label: o.get_field("dnsNameLabel"),
            dns_name_label_reuse_policy: o.get_field("dnsNameLabelReusePolicy"),
            exposed_ports: o.get_field("exposedPorts"),
            fqdn: o.get_field("fqdn"),
            identity: o.get_field("identity"),
            image_registry_credentials: o.get_field("imageRegistryCredentials"),
            init_containers: o.get_field("initContainers"),
            ip_address: o.get_field("ipAddress"),
            ip_address_type: o.get_field("ipAddressType"),
            key_vault_key_id: o.get_field("keyVaultKeyId"),
            key_vault_user_assigned_identity_id: o
                .get_field("keyVaultUserAssignedIdentityId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_profile_id: o.get_field("networkProfileId"),
            os_type: o.get_field("osType"),
            priority: o.get_field("priority"),
            resource_group_name: o.get_field("resourceGroupName"),
            restart_policy: o.get_field("restartPolicy"),
            sku: o.get_field("sku"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
