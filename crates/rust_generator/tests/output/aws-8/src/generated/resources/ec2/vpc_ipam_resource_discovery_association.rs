/// Provides an association between an Amazon IP Address Manager (IPAM) and a IPAM Resource Discovery. IPAM Resource Discoveries are resources meant for multi-organization customers. If you wish to use a single IPAM across multiple orgs, a resource discovery can be created and shared from a subordinate organization to the management organizations IPAM delegated admin account.
///
/// Once an association is created between two organizations via IPAM & a IPAM Resource Discovery, IPAM Pools can be shared via Resource Access Manager (RAM) to accounts in the subordinate organization; these RAM shares must be accepted by the end user account. Pools can then also discover and monitor IPAM resources in the subordinate organization.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:ec2:VpcIpamResourceDiscoveryAssociation
///     properties:
///       ipamId: ${testAwsVpcIpam.id}
///       ipamResourceDiscoveryId: ${testAwsVpcIpamResourceDiscovery.id}
///       tags:
///         Name: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the IPAM resource discovery association `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamResourceDiscoveryAssociation:VpcIpamResourceDiscoveryAssociation example ipam-res-disco-assoc-0178368ad2146a492
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_resource_discovery_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryAssociationArgs {
        /// The ID of the IPAM to associate.
        #[builder(into)]
        pub ipam_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Resource Discovery to associate.
        #[builder(into)]
        pub ipam_resource_discovery_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to add to the IPAM resource discovery association resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryAssociationResult {
        /// The Amazon Resource Name (ARN) of IPAM Resource Discovery Association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the IPAM.
        pub ipam_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the IPAM to associate.
        pub ipam_id: pulumi_gestalt_rust::Output<String>,
        /// The home region of the IPAM.
        pub ipam_region: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource Discovery to associate.
        pub ipam_resource_discovery_id: pulumi_gestalt_rust::Output<String>,
        /// A boolean to identify if the Resource Discovery is the accounts default resource discovery.
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// The account ID for the account that manages the Resource Discovery
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The lifecycle state of the association when you associate or disassociate a resource discovery.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to add to the IPAM resource discovery association resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamResourceDiscoveryAssociationArgs,
    ) -> VpcIpamResourceDiscoveryAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ipam_id_binding = args.ipam_id.get_output(context);
        let ipam_resource_discovery_id_binding = args
            .ipam_resource_discovery_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamResourceDiscoveryAssociation:VpcIpamResourceDiscoveryAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamId".into(),
                    value: &ipam_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamResourceDiscoveryId".into(),
                    value: &ipam_resource_discovery_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamResourceDiscoveryAssociationResult {
            arn: o.get_field("arn"),
            ipam_arn: o.get_field("ipamArn"),
            ipam_id: o.get_field("ipamId"),
            ipam_region: o.get_field("ipamRegion"),
            ipam_resource_discovery_id: o.get_field("ipamResourceDiscoveryId"),
            is_default: o.get_field("isDefault"),
            owner_id: o.get_field("ownerId"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
