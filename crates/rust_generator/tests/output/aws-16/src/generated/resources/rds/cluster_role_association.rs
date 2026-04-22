/// Manages a RDS DB Cluster association with an IAM Role. Example use cases:
///
/// * [Creating an IAM Role to Allow Amazon Aurora to Access AWS Services](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Integrating.Authorizing.IAM.CreateRole.html)
/// * [Importing Amazon S3 Data into an RDS PostgreSQL DB Cluster](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PostgreSQL.S3Import.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_role_association::create(
///         "example",
///         ClusterRoleAssociationArgs::builder()
///             .db_cluster_identifier("${exampleAwsRdsCluster.id}")
///             .feature_name("S3_INTEGRATION")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rds_cluster_role_association` using the DB Cluster Identifier and IAM Role ARN separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterRoleAssociation:ClusterRoleAssociation example my-db-cluster,arn:aws:iam::123456789012:role/my-role
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod cluster_role_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterRoleAssociationArgs {
        /// DB Cluster Identifier to associate with the IAM Role.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_gestalt_rust::Input<String>,
        /// Name of the feature for association. This can be found in the AWS documentation relevant to the integration or a full list is available in the `SupportedFeatureNames` list returned by [AWS CLI rds describe-db-engine-versions](https://docs.aws.amazon.com/cli/latest/reference/rds/describe-db-engine-versions.html).
        #[builder(into)]
        pub feature_name: pulumi_gestalt_rust::Input<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to associate with the DB Cluster.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterRoleAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// DB Cluster Identifier to associate with the IAM Role.
        pub db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Name of the feature for association. This can be found in the AWS documentation relevant to the integration or a full list is available in the `SupportedFeatureNames` list returned by [AWS CLI rds describe-db-engine-versions](https://docs.aws.amazon.com/cli/latest/reference/rds/describe-db-engine-versions.html).
        pub feature_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to associate with the DB Cluster.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterRoleAssociationArgs,
    ) -> ClusterRoleAssociationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterRoleAssociationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ClusterRoleAssociationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterRoleAssociationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ClusterRoleAssociationResult {
        let db_cluster_identifier_binding = args.db_cluster_identifier.get_output(ctx);
        let feature_name_binding = args.feature_name.get_output(ctx);
        let role_arn_binding = args.role_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/clusterRoleAssociation:ClusterRoleAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "featureName".into(),
                    value: &feature_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ClusterRoleAssociationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            db_cluster_identifier: o.get_field("dbClusterIdentifier"),
            feature_name: o.get_field("featureName"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
