/// Provides a resource to manage an RDS DB proxy default target group resource.
///
/// The `aws.rds.ProxyDefaultTargetGroup` behaves differently from normal resources, in that the provider does not _create_ or _destroy_ this resource, since it implicitly exists as part of an RDS DB Proxy. On the provider resource creation it is automatically imported and on resource destruction, the provider performs no actions in RDS.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rds:Proxy
///     properties:
///       name: example
///       debugLogging: false
///       engineFamily: MYSQL
///       idleClientTimeout: 1800
///       requireTls: true
///       roleArn: ${exampleAwsIamRole.arn}
///       vpcSecurityGroupIds:
///         - ${exampleAwsSecurityGroup.id}
///       vpcSubnetIds:
///         - ${exampleAwsSubnet.id}
///       auths:
///         - authScheme: SECRETS
///           description: example
///           iamAuth: DISABLED
///           secretArn: ${exampleAwsSecretsmanagerSecret.arn}
///       tags:
///         Name: example
///         Key: value
///   exampleProxyDefaultTargetGroup:
///     type: aws:rds:ProxyDefaultTargetGroup
///     name: example
///     properties:
///       dbProxyName: ${example.name}
///       connectionPoolConfig:
///         connectionBorrowTimeout: 120
///         initQuery: SET x=1, y=2
///         maxConnectionsPercent: 100
///         maxIdleConnectionsPercent: 50
///         sessionPinningFilters:
///           - EXCLUDE_VARIABLE_SETS
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DB proxy default target groups using the `db_proxy_name`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/proxyDefaultTargetGroup:ProxyDefaultTargetGroup example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod proxy_default_target_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupArgs {
        /// The settings that determine the size and behavior of the connection pool for the target group.
        #[builder(into, default)]
        pub connection_pool_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig>,
        >,
        /// Name of the RDS DB Proxy.
        #[builder(into)]
        pub db_proxy_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProxyDefaultTargetGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) representing the target group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The settings that determine the size and behavior of the connection pool for the target group.
        pub connection_pool_config: pulumi_gestalt_rust::Output<
            super::super::types::rds::ProxyDefaultTargetGroupConnectionPoolConfig,
        >,
        /// Name of the RDS DB Proxy.
        pub db_proxy_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the default target group.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyDefaultTargetGroupArgs,
    ) -> ProxyDefaultTargetGroupResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyDefaultTargetGroupArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ProxyDefaultTargetGroupResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProxyDefaultTargetGroupArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ProxyDefaultTargetGroupResult {
        let connection_pool_config_binding = args.connection_pool_config.get_output(ctx);
        let db_proxy_name_binding = args.db_proxy_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/proxyDefaultTargetGroup:ProxyDefaultTargetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionPoolConfig".into(),
                    value: &connection_pool_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbProxyName".into(),
                    value: &db_proxy_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ProxyDefaultTargetGroupResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            connection_pool_config: o.get_field("connectionPoolConfig"),
            db_proxy_name: o.get_field("dbProxyName"),
            name: o.get_field("name"),
        }
    }
}
