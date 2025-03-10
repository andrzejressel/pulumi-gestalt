/// Manages a Kubernetes Fleet Update Run.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: westeurope
///   exampleKubernetesFleetManager:
///     type: azure:containerservice:KubernetesFleetManager
///     name: example
///     properties:
///       location: ${example.location}
///       name: example
///       resourceGroupName: ${example.name}
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: example
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///       identity:
///         type: SystemAssigned
///   exampleFleetMember:
///     type: azure:containerservice:FleetMember
///     name: example
///     properties:
///       name: example
///       kubernetesFleetId: ${exampleKubernetesFleetManager.id}
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       group: example-group
///   exampleFleetUpdateRun:
///     type: azure:containerservice:FleetUpdateRun
///     name: example
///     properties:
///       name: example
///       kubernetesFleetManagerId: ${exampleKubernetesFleetManager.id}
///       managedClusterUpdate:
///         upgrade:
///           type: Full
///           kubernetesVersion: '1.27'
///         nodeImageSelection:
///           type: Latest
///       stages:
///         - name: example
///           groups:
///             - name: example-group
///           afterStageWaitInSeconds: 21
/// ```
///
/// ## Import
///
/// Kubernetes Fleet Update Runs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fleetUpdateRun:FleetUpdateRun example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/fleets/fleet1/updateRuns/updateRun1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet_update_run {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetUpdateRunArgs {
        /// The ID of the Fleet Update Strategy. Only one of `fleet_update_strategy_id` or `stage` can be specified.
        #[builder(into, default)]
        pub fleet_update_strategy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Run to be created.
        #[builder(into)]
        pub kubernetes_fleet_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `managed_cluster_update` block as defined below.
        #[builder(into)]
        pub managed_cluster_update: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containerservice::FleetUpdateRunManagedClusterUpdate,
        >,
        /// The name which should be used for this Kubernetes Fleet Update Run. Changing this forces a new Kubernetes Fleet Update Run to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `stage` blocks as defined below. Only one of `stage` or `fleet_update_strategy_id` can be specified.
        #[builder(into, default)]
        pub stages: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::FleetUpdateRunStage>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetUpdateRunResult {
        /// The ID of the Fleet Update Strategy. Only one of `fleet_update_strategy_id` or `stage` can be specified.
        pub fleet_update_strategy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Run to be created.
        pub kubernetes_fleet_manager_id: pulumi_gestalt_rust::Output<String>,
        /// A `managed_cluster_update` block as defined below.
        pub managed_cluster_update: pulumi_gestalt_rust::Output<
            super::super::types::containerservice::FleetUpdateRunManagedClusterUpdate,
        >,
        /// The name which should be used for this Kubernetes Fleet Update Run. Changing this forces a new Kubernetes Fleet Update Run to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `stage` blocks as defined below. Only one of `stage` or `fleet_update_strategy_id` can be specified.
        pub stages: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerservice::FleetUpdateRunStage>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetUpdateRunArgs,
    ) -> FleetUpdateRunResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let fleet_update_strategy_id_binding = args
            .fleet_update_strategy_id
            .get_output(context);
        let kubernetes_fleet_manager_id_binding = args
            .kubernetes_fleet_manager_id
            .get_output(context);
        let managed_cluster_update_binding = args
            .managed_cluster_update
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let stages_binding = args.stages.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/fleetUpdateRun:FleetUpdateRun".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetUpdateStrategyId".into(),
                    value: &fleet_update_strategy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesFleetManagerId".into(),
                    value: &kubernetes_fleet_manager_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedClusterUpdate".into(),
                    value: &managed_cluster_update_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stages".into(),
                    value: &stages_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetUpdateRunResult {
            fleet_update_strategy_id: o.get_field("fleetUpdateStrategyId"),
            kubernetes_fleet_manager_id: o.get_field("kubernetesFleetManagerId"),
            managed_cluster_update: o.get_field("managedClusterUpdate"),
            name: o.get_field("name"),
            stages: o.get_field("stages"),
        }
    }
}
