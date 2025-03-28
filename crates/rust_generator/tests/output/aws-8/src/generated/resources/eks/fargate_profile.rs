/// Manages an EKS Fargate Profile.
///
/// ## Example Usage
///
///
/// ### Example IAM Role for EKS Fargate Profile
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: eks-fargate-profile-example
///       assumeRolePolicy:
///         fn::toJSON:
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: eks-fargate-pods.amazonaws.com
///           Version: 2012-10-17
///   example-AmazonEKSFargatePodExecutionRolePolicy:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKSFargatePodExecutionRolePolicy
///       role: ${example.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Fargate Profiles using the `cluster_name` and `fargate_profile_name` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/fargateProfile:FargateProfile my_fargate_profile my_cluster:my_fargate_profile
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fargate_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FargateProfileArgs {
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the EKS Fargate Profile.
        #[builder(into, default)]
        pub fargate_profile_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Fargate Profile.
        #[builder(into)]
        pub pod_execution_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block(s) for selecting Kubernetes Pods to execute with this EKS Fargate Profile. Detailed below.
        #[builder(into)]
        pub selectors: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::eks::FargateProfileSelector>,
        >,
        /// Identifiers of private EC2 Subnets to associate with the EKS Fargate Profile. These subnets must have the following resource tag: `kubernetes.io/cluster/CLUSTER_NAME` (where `CLUSTER_NAME` is replaced with the name of the EKS Cluster).
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FargateProfileResult {
        /// Amazon Resource Name (ARN) of the EKS Fargate Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the EKS Cluster.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the EKS Fargate Profile.
        pub fargate_profile_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role that provides permissions for the EKS Fargate Profile.
        pub pod_execution_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block(s) for selecting Kubernetes Pods to execute with this EKS Fargate Profile. Detailed below.
        pub selectors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::eks::FargateProfileSelector>,
        >,
        /// Status of the EKS Fargate Profile.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of private EC2 Subnets to associate with the EKS Fargate Profile. These subnets must have the following resource tag: `kubernetes.io/cluster/CLUSTER_NAME` (where `CLUSTER_NAME` is replaced with the name of the EKS Cluster).
        ///
        /// The following arguments are optional:
        pub subnet_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: FargateProfileArgs,
    ) -> FargateProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_name_binding = args.cluster_name.get_output(context);
        let fargate_profile_name_binding = args.fargate_profile_name.get_output(context);
        let pod_execution_role_arn_binding = args
            .pod_execution_role_arn
            .get_output(context);
        let selectors_binding = args.selectors.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:eks/fargateProfile:FargateProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fargateProfileName".into(),
                    value: &fargate_profile_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "podExecutionRoleArn".into(),
                    value: &pod_execution_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectors".into(),
                    value: &selectors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FargateProfileResult {
            arn: o.get_field("arn"),
            cluster_name: o.get_field("clusterName"),
            fargate_profile_name: o.get_field("fargateProfileName"),
            pod_execution_role_arn: o.get_field("podExecutionRoleArn"),
            selectors: o.get_field("selectors"),
            status: o.get_field("status"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
