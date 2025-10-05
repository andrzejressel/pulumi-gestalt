/// Provides an RDS DB proxy endpoint resource. For additional information, see the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-proxy-endpoints.html).
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import DB proxy endpoints using the `DB-PROXY-NAME/DB-PROXY-ENDPOINT-NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/proxyEndpoint:ProxyEndpoint example example/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod proxy_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyEndpointArgs {
        /// The identifier for the proxy endpoint. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        #[builder(into)]
        pub db_proxy_endpoint_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the DB proxy associated with the DB proxy endpoint that you create.
        #[builder(into)]
        pub db_proxy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether the DB proxy endpoint can be used for read/write or read-only operations. The default is `READ_WRITE`. Valid values are `READ_WRITE` and `READ_ONLY`.
        #[builder(into, default)]
        pub target_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more VPC security group IDs to associate with the new proxy.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// One or more VPC subnet IDs to associate with the new proxy.
        #[builder(into)]
        pub vpc_subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct ProxyEndpointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the proxy endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the proxy endpoint. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive hyphens.
        pub db_proxy_endpoint_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the DB proxy associated with the DB proxy endpoint that you create.
        pub db_proxy_name: pulumi_gestalt_rust::Output<String>,
        /// The endpoint that you can use to connect to the proxy. You include the endpoint value in the connection string for a database client application.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether this endpoint is the default endpoint for the associated DB proxy.
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates whether the DB proxy endpoint can be used for read/write or read-only operations. The default is `READ_WRITE`. Valid values are `READ_WRITE` and `READ_ONLY`.
        pub target_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// The VPC ID of the DB proxy endpoint.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// One or more VPC security group IDs to associate with the new proxy.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more VPC subnet IDs to associate with the new proxy.
        pub vpc_subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyEndpointArgs,
    ) -> ProxyEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let db_proxy_endpoint_name_binding = args
            .db_proxy_endpoint_name
            .get_output(context);
        let db_proxy_name_binding = args.db_proxy_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_role_binding = args.target_role.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let vpc_subnet_ids_binding = args.vpc_subnet_ids.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/proxyEndpoint:ProxyEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbProxyEndpointName".into(),
                    value: &db_proxy_endpoint_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbProxyName".into(),
                    value: &db_proxy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetRole".into(),
                    value: &target_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSubnetIds".into(),
                    value: &vpc_subnet_ids_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProxyEndpointResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            db_proxy_endpoint_name: o.get_field("dbProxyEndpointName"),
            db_proxy_name: o.get_field("dbProxyName"),
            endpoint: o.get_field("endpoint"),
            is_default: o.get_field("isDefault"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            target_role: o.get_field("targetRole"),
            vpc_id: o.get_field("vpcId"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
            vpc_subnet_ids: o.get_field("vpcSubnetIds"),
        }
    }
}
