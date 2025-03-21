/// Manages an EMR Containers (EMR on EKS) Virtual Cluster.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:emrcontainers:VirtualCluster
///     properties:
///       containerProvider:
///         id: ${exampleAwsEksCluster.name}
///         type: EKS
///         info:
///           eksInfo:
///             namespace: default
///       name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS Clusters using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emrcontainers/virtualCluster:VirtualCluster example a1b2c3d4e5f6g7h8i9j10k11l
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualClusterArgs {
        /// Configuration block for the container provider associated with your cluster.
        #[builder(into)]
        pub container_provider: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::emrcontainers::VirtualClusterContainerProvider,
        >,
        /// Name of the virtual cluster.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualClusterResult {
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the container provider associated with your cluster.
        pub container_provider: pulumi_gestalt_rust::Output<
            super::super::types::emrcontainers::VirtualClusterContainerProvider,
        >,
        /// Name of the virtual cluster.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: VirtualClusterArgs,
    ) -> VirtualClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_provider_binding = args.container_provider.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emrcontainers/virtualCluster:VirtualCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerProvider".into(),
                    value: &container_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualClusterResult {
            arn: o.get_field("arn"),
            container_provider: o.get_field("containerProvider"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
