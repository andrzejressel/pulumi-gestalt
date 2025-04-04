/// An Anthos node pool running on AWS.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_aws_cluster
/// A basic example of a containeraws node pool
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: PREFER_NO_SCHEDULE
///             key: taint-key
///             value: taint-value
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       management:
///         autoRepair: true
///       kubeletConfig:
///         cpuManagerPolicy: none
///         cpuCfsQuota: true
///         cpuCfsQuotaPeriod: 100ms
///         podPidsLimit: 1024
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Basic_enum_aws_cluster
/// A basic example of a containeraws node pool with lowercase enums
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: prefer_no_schedule
///             key: taint-key
///             value: taint-value
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Beta_basic_enum_aws_cluster
/// A basic example of a containeraws node pool with lowercase enums (beta)
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: prefer_no_schedule
///             key: taint-key
///             value: taint-value
///         instancePlacement:
///           tenancy: dedicated
///         imageType: ubuntu
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
///
/// ## Import
///
/// NodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/awsClusters/{{cluster}}/awsNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{name}}`
///
/// * `{{location}}/{{cluster}}/{{name}}`
///
/// When using the `pulumi import` command, NodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default projects/{{project}}/locations/{{location}}/awsClusters/{{cluster}}/awsNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default {{project}}/{{location}}/{{cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default {{location}}/{{cluster}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod aws_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AwsNodePoolArgs {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        #[builder(into)]
        pub autoscaling: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolAutoscaling,
        >,
        /// The awsCluster for the resource
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration of the node pool.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolConfig,
        >,
        /// The kubelet configuration for the node pool.
        #[builder(into, default)]
        pub kubelet_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolKubeletConfig>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Management configuration for this node pool.
        #[builder(into, default)]
        pub management: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolManagement>,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        #[builder(into)]
        pub max_pods_constraint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subnet where the node pool node run.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Update settings control the speed and disruption of the node pool update.
        #[builder(into, default)]
        pub update_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolUpdateSettings>,
        >,
        /// The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AwsNodePoolResult {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        pub autoscaling: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolAutoscaling,
        >,
        /// The awsCluster for the resource
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// The configuration of the node pool.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolConfig,
        >,
        /// Output only. The time at which this node pool was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The kubelet configuration for the node pool.
        pub kubelet_config: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolKubeletConfig,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Management configuration for this node pool.
        pub management: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolManagement,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        pub max_pods_constraint: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. If set, there are currently changes in flight to the node pool.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The subnet where the node pool node run.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. A globally unique identifier for the node pool.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Optional. Update settings control the speed and disruption of the node pool update.
        pub update_settings: pulumi_gestalt_rust::Output<
            super::super::types::container::AwsNodePoolUpdateSettings,
        >,
        /// Output only. The time at which this node pool was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AwsNodePoolArgs,
    ) -> AwsNodePoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let autoscaling_binding = args.autoscaling.get_output(context);
        let cluster_binding = args.cluster.get_output(context);
        let config_binding = args.config.get_output(context);
        let kubelet_config_binding = args.kubelet_config.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_binding = args.management.get_output(context);
        let max_pods_constraint_binding = args.max_pods_constraint.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let update_settings_binding = args.update_settings.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:container/awsNodePool:AwsNodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscaling".into(),
                    value: &autoscaling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: &config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubeletConfig".into(),
                    value: &kubelet_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "management".into(),
                    value: &management_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxPodsConstraint".into(),
                    value: &max_pods_constraint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateSettings".into(),
                    value: &update_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AwsNodePoolResult {
            annotations: o.get_field("annotations"),
            autoscaling: o.get_field("autoscaling"),
            cluster: o.get_field("cluster"),
            config: o.get_field("config"),
            create_time: o.get_field("createTime"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            etag: o.get_field("etag"),
            kubelet_config: o.get_field("kubeletConfig"),
            location: o.get_field("location"),
            management: o.get_field("management"),
            max_pods_constraint: o.get_field("maxPodsConstraint"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            subnet_id: o.get_field("subnetId"),
            uid: o.get_field("uid"),
            update_settings: o.get_field("updateSettings"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
        }
    }
}
