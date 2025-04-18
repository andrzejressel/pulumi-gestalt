/// Manages an association with WAF Regional Web ACL.
///
/// > **Note:** An Application Load Balancer can only be associated with one WAF Regional WebACL.
///
/// ## Example Usage
///
/// ### Application Load Balancer Association
///
/// ```yaml
/// resources:
///   ipset:
///     type: aws:wafregional:IpSet
///     properties:
///       name: tfIPSet
///       ipSetDescriptors:
///         - type: IPV4
///           value: 192.0.7.0/24
///   foo:
///     type: aws:wafregional:Rule
///     properties:
///       name: tfWAFRule
///       metricName: tfWAFRule
///       predicates:
///         - dataId: ${ipset.id}
///           negated: false
///           type: IPMatch
///   fooWebAcl:
///     type: aws:wafregional:WebAcl
///     name: foo
///     properties:
///       name: foo
///       metricName: foo
///       defaultAction:
///         type: ALLOW
///       rules:
///         - action:
///             type: BLOCK
///           priority: 1
///           ruleId: ${foo.id}
///   fooVpc:
///     type: aws:ec2:Vpc
///     name: foo
///     properties:
///       cidrBlock: 10.1.0.0/16
///   fooSubnet:
///     type: aws:ec2:Subnet
///     name: foo
///     properties:
///       vpcId: ${fooVpc.id}
///       cidrBlock: 10.1.1.0/24
///       availabilityZone: ${available.names[0]}
///   bar:
///     type: aws:ec2:Subnet
///     properties:
///       vpcId: ${fooVpc.id}
///       cidrBlock: 10.1.2.0/24
///       availabilityZone: ${available.names[1]}
///   fooLoadBalancer:
///     type: aws:alb:LoadBalancer
///     name: foo
///     properties:
///       internal: true
///       subnets:
///         - ${fooSubnet.id}
///         - ${bar.id}
///   fooWebAclAssociation:
///     type: aws:wafregional:WebAclAssociation
///     name: foo
///     properties:
///       resourceArn: ${fooLoadBalancer.arn}
///       webAclId: ${fooWebAcl.id}
/// variables:
///   available:
///     fn::invoke:
///       function: aws:getAvailabilityZones
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Web ACL Association using their `web_acl_id:resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/webAclAssociation:WebAclAssociation foo web_acl_id:resource_arn
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_acl_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclAssociationArgs {
        /// ARN of the resource to associate with. For example, an Application Load Balancer or API Gateway Stage.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the WAF Regional WebACL to create an association.
        #[builder(into)]
        pub web_acl_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAclAssociationResult {
        /// ARN of the resource to associate with. For example, an Application Load Balancer or API Gateway Stage.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the WAF Regional WebACL to create an association.
        pub web_acl_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAclAssociationArgs,
    ) -> WebAclAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let resource_arn_binding = args.resource_arn.get_output(context);
        let web_acl_id_binding = args.web_acl_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/webAclAssociation:WebAclAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webAclId".into(),
                    value: &web_acl_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAclAssociationResult {
            resource_arn: o.get_field("resourceArn"),
            web_acl_id: o.get_field("webAclId"),
        }
    }
}
