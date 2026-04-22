/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Kubernetes Fleet Member.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:containerservice:KubernetesCluster
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       dnsPrefix: acctestaksexample
///       defaultNodePool:
///         name: example-value
///         nodeCount: example-value
///         vmSize: example-value
///         upgradeSettings:
///           maxSurge: example-value
///       identity:
///         type: example-value
///   exampleKubernetesFleetManager:
///     type: azure:containerservice:KubernetesFleetManager
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFleetMember:
///     type: azure:containerservice:FleetMember
///     name: example
///     properties:
///       kubernetesClusterId: ${example.id}
///       kubernetesFleetId: ${exampleKubernetesFleetManager.id}
///       name: example
/// ```
///
/// ## Import
///
/// An existing Kubernetes Fleet Member can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fleetMember:FleetMember example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/fleets/{fleetName}/members/{memberName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Kubernetes Fleet Member exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Kubernetes Fleet Member exists. For example `example-resource-group`.
///
/// * Where `{fleetName}` is the name of the Fleet. For example `fleetValue`.
///
/// * Where `{memberName}` is the name of the Member. For example `memberValue`.
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod fleet_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetMemberArgs {
        /// The group this member belongs to for multi-cluster update management.
        #[builder(into, default)]
        pub group: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ARM resource ID of the cluster that joins the Fleet. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the Kubernetes Fleet Id within which this Kubernetes Fleet Member should exist. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into)]
        pub kubernetes_fleet_id: pulumi_gestalt_rust::Input<String>,
        /// Specifies the name of this Kubernetes Fleet Member. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FleetMemberResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The group this member belongs to for multi-cluster update management.
        pub group: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARM resource ID of the cluster that joins the Fleet. Changing this forces a new Kubernetes Fleet Member to be created.
        pub kubernetes_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Kubernetes Fleet Id within which this Kubernetes Fleet Member should exist. Changing this forces a new Kubernetes Fleet Member to be created.
        pub kubernetes_fleet_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Kubernetes Fleet Member. Changing this forces a new Kubernetes Fleet Member to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetMemberArgs,
    ) -> FleetMemberResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetMemberArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FleetMemberResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetMemberArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FleetMemberResult {
        let group_binding = args.group.get_output(ctx);
        let kubernetes_cluster_id_binding = args.kubernetes_cluster_id.get_output(ctx);
        let kubernetes_fleet_id_binding = args.kubernetes_fleet_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/fleetMember:FleetMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesFleetId".into(),
                    value: &kubernetes_fleet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FleetMemberResult {
            id: o.get_id(),
            urn: o.get_urn(),
            group: o.get_field("group"),
            kubernetes_cluster_id: o.get_field("kubernetesClusterId"),
            kubernetes_fleet_id: o.get_field("kubernetesFleetId"),
            name: o.get_field("name"),
        }
    }
}
