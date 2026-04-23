/// Creates a new Amazon Redshift endpoint access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_access::create(
///         "example",
///         EndpointAccessArgs::builder()
///             .cluster_identifier("${exampleAwsRedshiftCluster.clusterIdentifier}")
///             .endpoint_name("example")
///             .subnet_group_name("${exampleAwsRedshiftSubnetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift endpoint access using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/endpointAccess:EndpointAccess example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod endpoint_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointAccessArgs {
        /// The cluster identifier of the cluster to access.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::Input<String>,
        /// The Redshift-managed VPC endpoint name.
        #[builder(into)]
        pub endpoint_name: pulumi_gestalt_rust::Input<String>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        #[builder(into, default)]
        pub resource_owner: pulumi_gestalt_rust::Input<Option<String>>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        #[builder(into)]
        pub subnet_group_name: pulumi_gestalt_rust::Input<String>,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct EndpointAccessResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The DNS address of the endpoint.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier of the cluster to access.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Redshift-managed VPC endpoint name.
        pub endpoint_name: pulumi_gestalt_rust::Output<String>,
        /// The port number on which the cluster accepts incoming connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The Amazon Web Services account ID of the owner of the cluster. This is only required if the cluster is in another Amazon Web Services account.
        pub resource_owner: pulumi_gestalt_rust::Output<String>,
        /// The subnet group from which Amazon Redshift chooses the subnet to deploy the endpoint.
        pub subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// The connection endpoint for connecting to an Amazon Redshift cluster through the proxy. See details below.
        pub vpc_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::redshift::EndpointAccessVpcEndpoint>,
        >,
        /// The security group that defines the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAccessArgs,
    ) -> EndpointAccessResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAccessArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EndpointAccessResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointAccessArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EndpointAccessResult {
        let cluster_identifier_binding = args.cluster_identifier.get_output(ctx);
        let endpoint_name_binding = args.endpoint_name.get_output(ctx);
        let resource_owner_binding = args.resource_owner.get_output(ctx);
        let subnet_group_name_binding = args.subnet_group_name.get_output(ctx);
        let vpc_security_group_ids_binding = args.vpc_security_group_ids.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/endpointAccess:EndpointAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointName".into(),
                    value: &endpoint_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceOwner".into(),
                    value: &resource_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetGroupName".into(),
                    value: &subnet_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EndpointAccessResult {
            id: o.get_id(),
            urn: o.get_urn(),
            address: o.get_field("address"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            endpoint_name: o.get_field("endpointName"),
            port: o.get_field("port"),
            resource_owner: o.get_field("resourceOwner"),
            subnet_group_name: o.get_field("subnetGroupName"),
            vpc_endpoints: o.get_field("vpcEndpoints"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
