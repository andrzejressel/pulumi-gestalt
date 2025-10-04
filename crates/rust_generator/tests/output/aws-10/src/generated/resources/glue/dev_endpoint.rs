/// Provides a Glue Development Endpoint resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   exampleDevEndpoint:
///     type: aws:glue:DevEndpoint
///     name: example
///     properties:
///       name: foo
///       roleArn: ${exampleRole.arn}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: AWSGlueServiceRole-foo
///       assumeRolePolicy: ${example.json}
///   example-AWSGlueServiceRole:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSGlueServiceRole
///       role: ${exampleRole.name}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - glue.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Glue Development Endpoint using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/devEndpoint:DevEndpoint example foo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dev_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevEndpointArgs {
        /// A map of arguments used to configure the endpoint.
        #[builder(into, default)]
        pub arguments: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Path to one or more Java Jars in an S3 bucket that should be loaded in this endpoint.
        #[builder(into, default)]
        pub extra_jars_s3_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path(s) to one or more Python libraries in an S3 bucket that should be loaded in this endpoint. Multiple values must be complete paths separated by a comma.
        #[builder(into, default)]
        pub extra_python_libs_s3_path: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the versions of Python and Apache Spark to use. Defaults to AWS Glue version 0.9.
        #[builder(into, default)]
        pub glue_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of this endpoint. It must be unique in your account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of AWS Glue Data Processing Units (DPUs) to allocate to this endpoint. Conflicts with `worker_type`.
        #[builder(into, default)]
        pub number_of_nodes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of workers of a defined worker type that are allocated to this endpoint. This field is available only when you choose worker type G.1X or G.2X.
        #[builder(into, default)]
        pub number_of_workers: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The public key to be used by this endpoint for authentication.
        #[builder(into, default)]
        pub public_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of public keys to be used by this endpoint for authentication.
        #[builder(into, default)]
        pub public_keys: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The IAM role for this endpoint.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Security Configuration structure to be used with this endpoint.
        #[builder(into, default)]
        pub security_configuration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Security group IDs for the security groups to be used by this endpoint.
        #[builder(into, default)]
        pub security_group_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The subnet ID for the new endpoint to use.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of predefined worker that is allocated to this endpoint. Accepts a value of Standard, G.1X, or G.2X.
        #[builder(into, default)]
        pub worker_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DevEndpointResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A map of arguments used to configure the endpoint.
        pub arguments: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the endpoint.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS availability zone where this endpoint is located.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Path to one or more Java Jars in an S3 bucket that should be loaded in this endpoint.
        pub extra_jars_s3_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Path(s) to one or more Python libraries in an S3 bucket that should be loaded in this endpoint. Multiple values must be complete paths separated by a comma.
        pub extra_python_libs_s3_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The reason for a current failure in this endpoint.
        pub failure_reason: pulumi_gestalt_rust::Output<String>,
        /// Specifies the versions of Python and Apache Spark to use. Defaults to AWS Glue version 0.9.
        pub glue_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of this endpoint. It must be unique in your account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of AWS Glue Data Processing Units (DPUs) to allocate to this endpoint. Conflicts with `worker_type`.
        pub number_of_nodes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of workers of a defined worker type that are allocated to this endpoint. This field is available only when you choose worker type G.1X or G.2X.
        pub number_of_workers: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A private IP address to access the endpoint within a VPC, if this endpoint is created within one.
        pub private_address: pulumi_gestalt_rust::Output<String>,
        /// The public IP address used by this endpoint. The PublicAddress field is present only when you create a non-VPC endpoint.
        pub public_address: pulumi_gestalt_rust::Output<String>,
        /// The public key to be used by this endpoint for authentication.
        pub public_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of public keys to be used by this endpoint for authentication.
        pub public_keys: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The IAM role for this endpoint.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Security Configuration structure to be used with this endpoint.
        pub security_configuration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Security group IDs for the security groups to be used by this endpoint.
        pub security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The current status of this endpoint.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The subnet ID for the new endpoint to use.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// he ID of the VPC used by this endpoint.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The type of predefined worker that is allocated to this endpoint. Accepts a value of Standard, G.1X, or G.2X.
        pub worker_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The YARN endpoint address used by this endpoint.
        pub yarn_endpoint_address: pulumi_gestalt_rust::Output<String>,
        /// The Apache Zeppelin port for the remote Apache Spark interpreter.
        pub zeppelin_remote_spark_interpreter_port: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DevEndpointArgs,
    ) -> DevEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arguments_binding = args.arguments.get_output(context);
        let extra_jars_s3_path_binding = args.extra_jars_s3_path.get_output(context);
        let extra_python_libs_s3_path_binding = args
            .extra_python_libs_s3_path
            .get_output(context);
        let glue_version_binding = args.glue_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let number_of_nodes_binding = args.number_of_nodes.get_output(context);
        let number_of_workers_binding = args.number_of_workers.get_output(context);
        let public_key_binding = args.public_key.get_output(context);
        let public_keys_binding = args.public_keys.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let security_configuration_binding = args
            .security_configuration
            .get_output(context);
        let security_group_ids_binding = args.security_group_ids.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let worker_type_binding = args.worker_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/devEndpoint:DevEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arguments".into(),
                    value: &arguments_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extraJarsS3Path".into(),
                    value: &extra_jars_s3_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extraPythonLibsS3Path".into(),
                    value: &extra_python_libs_s3_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "glueVersion".into(),
                    value: &glue_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberOfNodes".into(),
                    value: &number_of_nodes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberOfWorkers".into(),
                    value: &number_of_workers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKeys".into(),
                    value: &public_keys_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupIds".into(),
                    value: &security_group_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerType".into(),
                    value: &worker_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DevEndpointResult {
            id: o.get_field("id"),
            arguments: o.get_field("arguments"),
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            extra_jars_s3_path: o.get_field("extraJarsS3Path"),
            extra_python_libs_s3_path: o.get_field("extraPythonLibsS3Path"),
            failure_reason: o.get_field("failureReason"),
            glue_version: o.get_field("glueVersion"),
            name: o.get_field("name"),
            number_of_nodes: o.get_field("numberOfNodes"),
            number_of_workers: o.get_field("numberOfWorkers"),
            private_address: o.get_field("privateAddress"),
            public_address: o.get_field("publicAddress"),
            public_key: o.get_field("publicKey"),
            public_keys: o.get_field("publicKeys"),
            role_arn: o.get_field("roleArn"),
            security_configuration: o.get_field("securityConfiguration"),
            security_group_ids: o.get_field("securityGroupIds"),
            status: o.get_field("status"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
            worker_type: o.get_field("workerType"),
            yarn_endpoint_address: o.get_field("yarnEndpointAddress"),
            zeppelin_remote_spark_interpreter_port: o
                .get_field("zeppelinRemoteSparkInterpreterPort"),
        }
    }
}
