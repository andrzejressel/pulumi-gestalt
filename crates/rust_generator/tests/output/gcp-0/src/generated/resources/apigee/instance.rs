/// An `Instance` is the runtime dataplane in Apigee.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances/create)
/// * How-to Guides
///     * [Creating a runtime instance](https://cloud.google.com/apigee/docs/api-platform/get-started/create-instance)
///
/// ## Example Usage
///
/// ### Apigee Instance Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance-name
///       location: us-central1
///       orgId: ${apigeeOrg.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Instance Cidr Range
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 22
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance-name
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       peeringCidrRange: SLASH_22
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Instance Ip Range
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 22
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance-name
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       ipRange: 10.87.8.0/22
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Instance Full
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeKeyring:
///     type: gcp:kms:KeyRing
///     name: apigee_keyring
///     properties:
///       name: apigee-keyring
///       location: us-central1
///   apigeeKey:
///     type: gcp:kms:CryptoKey
///     name: apigee_key
///     properties:
///       name: apigee-key
///       keyRing: ${apigeeKeyring.id}
///   apigeeSa:
///     type: gcp:projects:ServiceIdentity
///     name: apigee_sa
///     properties:
///       project: ${project.projectId}
///       service: ${apigee.service}
///   apigeeSaKeyuser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: apigee_sa_keyuser
///     properties:
///       cryptoKeyId: ${apigeeKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: ${apigeeSa.member}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       displayName: apigee-org
///       description: Auto-provisioned Apigee Org.
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///       runtimeDatabaseEncryptionKeyName: ${apigeeKey.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigeeSaKeyuser}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance-name
///       location: us-central1
///       description: Auto-managed Apigee Runtime Instance
///       displayName: my-instance-name
///       orgId: ${apigeeOrg.id}
///       diskEncryptionKeyName: ${apigeeKey.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `{{org_id}}/instances/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/instance:Instance default {{org_id}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/instance:Instance default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Optional. Customer accept list represents the list of projects (id/number) on customer
        /// side that can privately connect to the service attachment. It is an optional field
        /// which the customers can provide during the instance creation. By default, the customer
        /// project associated with the Apigee organization will be included to the list.
        #[builder(into, default)]
        pub consumer_accept_lists: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Description of the instance.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Customer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only.
        /// Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)`
        #[builder(into, default)]
        pub disk_encryption_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name of the instance.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP range represents the customer-provided CIDR block of length 22 that will be used for
        /// the Apigee instance creation. This optional range, if provided, should be freely
        /// available as part of larger named range the customer has allocated to the Service
        /// Networking peering. If this is not provided, Apigee will automatically request for any
        /// available /22 CIDR block from Service Networking. The customer should use this CIDR block
        /// for configuring their firewall needs to allow traffic from Apigee.
        /// Input format: "a.b.c.d/22"
        #[builder(into, default)]
        pub ip_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Compute Engine location where the instance resides.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource ID of the instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of the CIDR block range that will be reserved by the instance. For valid values,
        /// see [CidrRange](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances#CidrRange) on the documentation.
        #[builder(into, default)]
        pub peering_cidr_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Customer accept list represents the list of projects (id/number) on customer
        /// side that can privately connect to the service attachment. It is an optional field
        /// which the customers can provide during the instance creation. By default, the customer
        /// project associated with the Apigee organization will be included to the list.
        pub consumer_accept_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Description of the instance.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Customer Managed Encryption Key (CMEK) used for disk and volume encryption. Required for Apigee paid subscriptions only.
        /// Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)`
        pub disk_encryption_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Display name of the instance.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Hostname or IP address of the exposed Apigee endpoint used by clients to connect to the service.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// IP range represents the customer-provided CIDR block of length 22 that will be used for
        /// the Apigee instance creation. This optional range, if provided, should be freely
        /// available as part of larger named range the customer has allocated to the Service
        /// Networking peering. If this is not provided, Apigee will automatically request for any
        /// available /22 CIDR block from Service Networking. The customer should use this CIDR block
        /// for configuring their firewall needs to allow traffic from Apigee.
        /// Input format: "a.b.c.d/22"
        pub ip_range: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. Compute Engine location where the instance resides.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource ID of the instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// The size of the CIDR block range that will be reserved by the instance. For valid values,
        /// see [CidrRange](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances#CidrRange) on the documentation.
        pub peering_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// Output only. Port number of the exposed Apigee endpoint.
        pub port: pulumi_gestalt_rust::Output<String>,
        /// Output only. Resource name of the service attachment created for the instance in
        /// the format: projects/*/regions/*/serviceAttachments/* Apigee customers can privately
        /// forward traffic to this service attachment using the PSC endpoints.
        pub service_attachment: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let consumer_accept_lists_binding = args
            .consumer_accept_lists
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let disk_encryption_key_name_binding = args
            .disk_encryption_key_name
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let ip_range_binding = args.ip_range.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let peering_cidr_range_binding = args.peering_cidr_range.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consumerAcceptLists".into(),
                    value: &consumer_accept_lists_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskEncryptionKeyName".into(),
                    value: &disk_encryption_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipRange".into(),
                    value: &ip_range_binding.drop_type(),
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
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeringCidrRange".into(),
                    value: &peering_cidr_range_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            id: o.get_field("id"),
            consumer_accept_lists: o.get_field("consumerAcceptLists"),
            description: o.get_field("description"),
            disk_encryption_key_name: o.get_field("diskEncryptionKeyName"),
            display_name: o.get_field("displayName"),
            host: o.get_field("host"),
            ip_range: o.get_field("ipRange"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
            peering_cidr_range: o.get_field("peeringCidrRange"),
            port: o.get_field("port"),
            service_attachment: o.get_field("serviceAttachment"),
        }
    }
}
