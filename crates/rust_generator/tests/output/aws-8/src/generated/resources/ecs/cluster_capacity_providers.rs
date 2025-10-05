/// Manages the capacity providers of an ECS Cluster.
///
/// More information about capacity providers can be found in the [ECS User Guide](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-capacity-providers.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder().name("my-cluster").build_struct(),
///     );
///     let exampleClusterCapacityProviders = cluster_capacity_providers::create(
///         "exampleClusterCapacityProviders",
///         ClusterCapacityProvidersArgs::builder()
///             .capacity_providers(vec!["FARGATE",])
///             .cluster_name("${example.name}")
///             .default_capacity_provider_strategies(
///                 vec![
///                     ClusterCapacityProvidersDefaultCapacityProviderStrategy::builder()
///                     .base(1).capacityProvider("FARGATE").weight(100).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECS cluster capacity providers using the `cluster_name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:ecs/clusterCapacityProviders:ClusterCapacityProviders example my-cluster
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_capacity_providers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterCapacityProvidersArgs {
        /// Set of names of one or more capacity providers to associate with the cluster. Valid values also include `FARGATE` and `FARGATE_SPOT`.
        #[builder(into, default)]
        pub capacity_providers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Name of the ECS cluster to manage capacity providers for.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of capacity provider strategies to use by default for the cluster. Detailed below.
        #[builder(into, default)]
        pub default_capacity_provider_strategies: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::ecs::ClusterCapacityProvidersDefaultCapacityProviderStrategy,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterCapacityProvidersResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of names of one or more capacity providers to associate with the cluster. Valid values also include `FARGATE` and `FARGATE_SPOT`.
        pub capacity_providers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Name of the ECS cluster to manage capacity providers for.
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Set of capacity provider strategies to use by default for the cluster. Detailed below.
        pub default_capacity_provider_strategies: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::ecs::ClusterCapacityProvidersDefaultCapacityProviderStrategy,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterCapacityProvidersArgs,
    ) -> ClusterCapacityProvidersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_providers_binding = args.capacity_providers.get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let default_capacity_provider_strategies_binding = args
            .default_capacity_provider_strategies
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ecs/clusterCapacityProviders:ClusterCapacityProviders".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityProviders".into(),
                    value: &capacity_providers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultCapacityProviderStrategies".into(),
                    value: &default_capacity_provider_strategies_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterCapacityProvidersResult {
            id: o.get_field("id"),
            capacity_providers: o.get_field("capacityProviders"),
            cluster_name: o.get_field("clusterName"),
            default_capacity_provider_strategies: o
                .get_field("defaultCapacityProviderStrategies"),
        }
    }
}
