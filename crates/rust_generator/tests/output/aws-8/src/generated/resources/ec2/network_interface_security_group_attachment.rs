/// This resource attaches a security group to an Elastic Network Interface (ENI).
/// It can be used to attach a security group to any existing ENI, be it a
/// secondary ENI or one attached as the primary interface on an instance.
///
/// > **NOTE on instances, interfaces, and security groups:** This provider currently
/// provides the capability to assign security groups via the [`aws.ec2.Instance`][1]
/// and the [`aws.ec2.NetworkInterface`][2] resources. Using this resource in
/// conjunction with security groups provided in-line in those resources will cause
/// conflicts, and will lead to spurious diffs and undefined behavior - please use
/// one or the other.
///
/// ## Example Usage
///
/// The following provides a very basic example of setting up an instance (provided
/// by `instance`) in the default security group, creating a security group
/// (provided by `sg`) and then attaching the security group to the instance's
/// primary network interface via the `aws.ec2.NetworkInterfaceSecurityGroupAttachment` resource,
/// named `sg_attachment`:
///
/// ```yaml
/// resources:
///   instance:
///     type: aws:ec2:Instance
///     properties:
///       instanceType: t2.micro
///       ami: ${ami.id}
///       tags:
///         type: test-instance
///   sg:
///     type: aws:ec2:SecurityGroup
///     properties:
///       tags:
///         type: test-security-group
///   sgAttachment:
///     type: aws:ec2:NetworkInterfaceSecurityGroupAttachment
///     name: sg_attachment
///     properties:
///       securityGroupId: ${sg.id}
///       networkInterfaceId: ${instance.primaryNetworkInterfaceId}
/// variables:
///   ami:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         filters:
///           - name: name
///             values:
///               - amzn-ami-hvm-*
///         owners:
///           - amazon
/// ```
///
/// In this example, `instance` is provided by the `aws.ec2.Instance` data source,
/// fetching an external instance, possibly not managed by this provider.
/// `sg_attachment` then attaches to the output instance's `network_interface_id`:
///
/// ```yaml
/// resources:
///   sg:
///     type: aws:ec2:SecurityGroup
///     properties:
///       tags:
///         type: test-security-group
///   sgAttachment:
///     type: aws:ec2:NetworkInterfaceSecurityGroupAttachment
///     name: sg_attachment
///     properties:
///       securityGroupId: ${sg.id}
///       networkInterfaceId: ${instance.networkInterfaceId}
/// variables:
///   instance:
///     fn::invoke:
///       function: aws:ec2:getInstance
///       arguments:
///         instanceId: i-1234567890abcdef0
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Interface Security Group attachments using the associated network interface ID and security group ID, separated by an underscore (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/networkInterfaceSecurityGroupAttachment:NetworkInterfaceSecurityGroupAttachment sg_attachment eni-1234567890abcdef0_sg-1234567890abcdef0
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_interface_security_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkInterfaceSecurityGroupAttachmentArgs {
        /// The ID of the network interface to attach to.
        #[builder(into)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkInterfaceSecurityGroupAttachmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the network interface to attach to.
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkInterfaceSecurityGroupAttachmentArgs,
    ) -> NetworkInterfaceSecurityGroupAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let network_interface_id_binding = args.network_interface_id.get_output(context);
        let security_group_id_binding = args.security_group_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/networkInterfaceSecurityGroupAttachment:NetworkInterfaceSecurityGroupAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkInterfaceSecurityGroupAttachmentResult {
            id: o.get_field("id"),
            network_interface_id: o.get_field("networkInterfaceId"),
            security_group_id: o.get_field("securityGroupId"),
        }
    }
}
