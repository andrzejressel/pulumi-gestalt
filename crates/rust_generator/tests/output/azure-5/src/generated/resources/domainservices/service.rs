/// ## Example Usage
///
/// ```yaml
/// resources:
///   deploy:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   deployVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: deploy
///     properties:
///       name: deploy-vnet
///       location: ${deploy.location}
///       resourceGroupName: ${deploy.name}
///       addressSpaces:
///         - 10.0.1.0/16
///   deploySubnet:
///     type: azure:network:Subnet
///     name: deploy
///     properties:
///       name: deploy-subnet
///       resourceGroupName: ${deploy.name}
///       virtualNetworkName: ${deployVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   deployNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: deploy
///     properties:
///       name: deploy-nsg
///       location: ${deploy.location}
///       resourceGroupName: ${deploy.name}
///       securityRules:
///         - name: AllowSyncWithAzureAD
///           priority: 101
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '443'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowRD
///           priority: 201
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '3389'
///           sourceAddressPrefix: CorpNetSaw
///           destinationAddressPrefix: '*'
///         - name: AllowPSRemoting
///           priority: 301
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '5986'
///           sourceAddressPrefix: AzureActiveDirectoryDomainServices
///           destinationAddressPrefix: '*'
///         - name: AllowLDAPS
///           priority: 401
///           direction: Inbound
///           access: Allow
///           protocol: Tcp
///           sourcePortRange: '*'
///           destinationPortRange: '636'
///           sourceAddressPrefix: '*'
///           destinationAddressPrefix: '*'
///   deploySubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: deploy
///     properties:
///       subnetId: ${deploySubnet.id}
///       networkSecurityGroupId: ${deployNetworkSecurityGroup.id}
///   dcAdmins:
///     type: azuread:Group
///     name: dc_admins
///     properties:
///       displayName: AAD DC Administrators
///       securityEnabled: true
///   admin:
///     type: azuread:User
///     properties:
///       userPrincipalName: dc-admin@hashicorp-example.com
///       displayName: DC Administrator
///       password: Pa55w0Rd!!1
///   adminGroupMember:
///     type: azuread:GroupMember
///     name: admin
///     properties:
///       groupObjectId: ${dcAdmins.objectId}
///       memberObjectId: ${admin.objectId}
///   example:
///     type: azuread:ServicePrincipal
///     properties:
///       applicationId: 2565bd9d-da50-47d4-8b85-4c97f669dc36
///   aadds:
///     type: azure:core:ResourceGroup
///     properties:
///       name: aadds-rg
///       location: westeurope
///   exampleService:
///     type: azure:domainservices:Service
///     name: example
///     properties:
///       name: example-aadds
///       location: ${aadds.location}
///       resourceGroupName: ${aadds.name}
///       domainName: widgetslogin.net
///       sku: Enterprise
///       filteredSyncEnabled: false
///       initialReplicaSet:
///         subnetId: ${deploySubnet.id}
///       notifications:
///         additionalRecipients:
///           - notifyA@example.net
///           - notifyB@example.org
///         notifyDcAdmins: true
///         notifyGlobalAdmins: true
///       security:
///         syncKerberosPasswords: true
///         syncNtlmPasswords: true
///         syncOnPremPasswords: true
///       tags:
///         Environment: prod
///     options:
///       dependsOn:
///         - ${example}
///         - ${deploySubnetNetworkSecurityGroupAssociation}
/// ```
///
/// ## Import
///
/// Domain Services can be imported using the resource ID, together with the Replica Set ID that you wish to designate as the initial replica set, e.g.
///
/// ```sh
/// $ pulumi import azure:domainservices/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AAD/domainServices/instance1/initialReplicaSetId/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub domain_configuration_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
        #[builder(into, default)]
        pub filtered_sync_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
        #[builder(into)]
        pub initial_replica_set: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::domainservices::ServiceInitialReplicaSet,
        >,
        /// The Azure location where the Domain Service exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `notifications` block as defined below.
        #[builder(into, default)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::domainservices::ServiceNotifications>,
        >,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `secure_ldap` block as defined below.
        #[builder(into, default)]
        pub secure_ldap: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::domainservices::ServiceSecureLdap>,
        >,
        /// A `security` block as defined below.
        #[builder(into, default)]
        pub security: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::domainservices::ServiceSecurity>,
        >,
        /// The SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A unique ID for the managed domain deployment.
        pub deployment_id: pulumi_gestalt_rust::Output<String>,
        /// The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
        pub domain_configuration_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
        pub filtered_sync_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
        pub initial_replica_set: pulumi_gestalt_rust::Output<
            super::super::types::domainservices::ServiceInitialReplicaSet,
        >,
        /// The Azure location where the Domain Service exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `notifications` block as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            super::super::types::domainservices::ServiceNotifications,
        >,
        /// The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Azure resource ID for the domain service.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// A `secure_ldap` block as defined below.
        pub secure_ldap: pulumi_gestalt_rust::Output<
            super::super::types::domainservices::ServiceSecureLdap,
        >,
        /// A `security` block as defined below.
        pub security: pulumi_gestalt_rust::Output<
            super::super::types::domainservices::ServiceSecurity,
        >,
        /// The SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        pub sync_owner: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_configuration_type_binding = args
            .domain_configuration_type
            .get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let filtered_sync_enabled_binding = args
            .filtered_sync_enabled
            .get_output(context);
        let initial_replica_set_binding = args.initial_replica_set.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let notifications_binding = args.notifications.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let secure_ldap_binding = args.secure_ldap.get_output(context);
        let security_binding = args.security.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:domainservices/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainConfigurationType".into(),
                    value: &domain_configuration_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filteredSyncEnabled".into(),
                    value: &filtered_sync_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialReplicaSet".into(),
                    value: &initial_replica_set_binding.drop_type(),
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
                    name: "notifications".into(),
                    value: &notifications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secureLdap".into(),
                    value: &secure_ldap_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "security".into(),
                    value: &security_binding.drop_type(),
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
        };
        let o = context.register_resource(request);
        ServiceResult {
            id: o.get_field("id"),
            deployment_id: o.get_field("deploymentId"),
            domain_configuration_type: o.get_field("domainConfigurationType"),
            domain_name: o.get_field("domainName"),
            filtered_sync_enabled: o.get_field("filteredSyncEnabled"),
            initial_replica_set: o.get_field("initialReplicaSet"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_id: o.get_field("resourceId"),
            secure_ldap: o.get_field("secureLdap"),
            security: o.get_field("security"),
            sku: o.get_field("sku"),
            sync_owner: o.get_field("syncOwner"),
            tags: o.get_field("tags"),
            tenant_id: o.get_field("tenantId"),
            version: o.get_field("version"),
        }
    }
}
