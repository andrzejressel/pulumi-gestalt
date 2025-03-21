/// Manages a Route53 Hosted Zone VPC association. VPC associations can only be made on private zones. See the `aws.route53.VpcAssociationAuthorization` resource for setting up cross-account associations.
///
/// > **NOTE:** Unless explicit association ordering is required (e.g., a separate cross-account association authorization), usage of this resource is not recommended. Use the `vpc` configuration blocks available within the `aws.route53.Zone` resource instead.
///
/// > **NOTE:** This provider provides both this standalone Zone VPC Association resource and exclusive VPC associations defined in-line in the `aws.route53.Zone` resource via `vpc` configuration blocks. At this time, you cannot use those in-line VPC associations in conjunction with this resource and the same zone ID otherwise it will cause a perpetual difference in plan output. You can optionally use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) in the `aws.route53.Zone` resource to manage additional associations via this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone::create(
///         "example",
///         ZoneArgs::builder()
///             .name("example.com")
///             .vpcs(vec![ZoneVpc::builder().vpcId("${primary.id}").build_struct(),])
///             .build_struct(),
///     );
///     let primary = vpc::create(
///         "primary",
///         VpcArgs::builder()
///             .cidr_block("10.6.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let secondary = vpc::create(
///         "secondary",
///         VpcArgs::builder()
///             .cidr_block("10.7.0.0/16")
///             .enable_dns_hostnames(true)
///             .enable_dns_support(true)
///             .build_struct(),
///     );
///     let secondaryZoneAssociation = zone_association::create(
///         "secondaryZoneAssociation",
///         ZoneAssociationArgs::builder()
///             .vpc_id("${secondary.id}")
///             .zone_id("${example.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// The VPC is _not_ in the same region where you have configured the AWS Provider:
///
/// __Using `pulumi import` to import__ Route 53 Hosted Zone Associations using the Hosted Zone ID and VPC ID, separated by a colon (`:`). For example:
///
/// The VPC is in the same region where you have configured the AWS Provider:
///
/// ```sh
/// $ pulumi import aws:route53/zoneAssociation:ZoneAssociation example Z123456ABCDEFG:vpc-12345678
/// ```
/// The VPC is _not_ in the same region where you have configured the AWS Provider:
///
/// ```sh
/// $ pulumi import aws:route53/zoneAssociation:ZoneAssociation example Z123456ABCDEFG:vpc-12345678:us-east-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneAssociationArgs {
        /// The VPC to associate with the private hosted zone.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VPC's region. Defaults to the region of the AWS provider.
        #[builder(into, default)]
        pub vpc_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private hosted zone to associate.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneAssociationResult {
        /// The account ID of the account that created the hosted zone.
        pub owning_account: pulumi_gestalt_rust::Output<String>,
        /// The VPC to associate with the private hosted zone.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC's region. Defaults to the region of the AWS provider.
        pub vpc_region: pulumi_gestalt_rust::Output<String>,
        /// The private hosted zone to associate.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneAssociationArgs,
    ) -> ZoneAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let vpc_id_binding = args.vpc_id.get_output(context);
        let vpc_region_binding = args.vpc_region.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/zoneAssociation:ZoneAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcRegion".into(),
                    value: &vpc_region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneAssociationResult {
            owning_account: o.get_field("owningAccount"),
            vpc_id: o.get_field("vpcId"),
            vpc_region: o.get_field("vpcRegion"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
