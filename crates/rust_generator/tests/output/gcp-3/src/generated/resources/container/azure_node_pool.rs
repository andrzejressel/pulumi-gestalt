/// An Anthos node pool running on Azure.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_azure_node_pool
/// A basic example of a containerazure azure node pool
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AzureCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: mmv2@google.com
///       azureRegion: westus2
///       client: projects/my-project-number/locations/us-west1/azureClients/${basic.name}
///       controlPlane:
///         sshConfig:
///           authorizedKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8yaayO6lnb2v+SedxUMa2c8vtIEzCzBjM3EJJsv8Vm9zUDWR7dXWKoNGARUb2mNGXASvI6mFIDXTIlkQ0poDEPpMaXR0g2cb5xT8jAAJq7fqXL3+0rcJhY/uigQ+MrT6s+ub0BFVbsmGHNrMQttXX9gtmwkeAEvj3mra9e5pkNf90qlKnZz6U0SVArxVsLx07vHPHDIYrl0OPG4zUREF52igbBPiNrHJFDQJT/4YlDMJmo/QT/A1D6n9ocemvZSzhRx15/Arjowhr+VVKSbaxzPtEfY0oIg2SrqJnnr/l3Du5qIefwh5VmCZe4xopPUaDDoOIEFriZ88sB+3zz8ib8sk8zJJQCgeP78tQvXCgS+4e5W3TUg9mxjB6KjXTyHIVhDZqhqde0OI3Fy1UuVzRUwnBaLjBnAwP5EoFQGRmDYk/rEYe7HTmovLeEBUDQocBQKT4Ripm/xJkkWY7B07K/tfo56dGUCkvyIVXKBInCh+dLK7gZapnd4UWkY0xBYcwo1geMLRq58iFTLA2j/JmpmHXp7m0l7jJii7d44uD3tTIFYThn7NlOnvhLim/YcBK07GMGIN7XwrrKZKmxXaspw6KBWVhzuw1UPxctxshYEaMLfFg/bwOw8HvMPr9VtrElpSB7oiOh91PDIPdPBgHCi7N2QgQ5l/ZDBHieSpNrQ== thomasrodgers
///         subnetId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet/subnets/default
///         version: ${versions.validVersions[0]}
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.200.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.32.0.0/24
///         virtualNetworkId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet
///       resourceGroupId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-cluster
///       project: my-project-name
///   basic:
///     type: gcp:container:AzureClient
///     properties:
///       applicationId: 12345678-1234-1234-1234-123456789111
///       location: us-west1
///       name: client-name
///       tenantId: 12345678-1234-1234-1234-123456789111
///       project: my-project-name
///   primaryAzureNodePool:
///     type: gcp:container:AzureNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 3
///         minNodeCount: 2
///       cluster: ${primary.name}
///       config:
///         sshConfig:
///           authorizedKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8yaayO6lnb2v+SedxUMa2c8vtIEzCzBjM3EJJsv8Vm9zUDWR7dXWKoNGARUb2mNGXASvI6mFIDXTIlkQ0poDEPpMaXR0g2cb5xT8jAAJq7fqXL3+0rcJhY/uigQ+MrT6s+ub0BFVbsmGHNrMQttXX9gtmwkeAEvj3mra9e5pkNf90qlKnZz6U0SVArxVsLx07vHPHDIYrl0OPG4zUREF52igbBPiNrHJFDQJT/4YlDMJmo/QT/A1D6n9ocemvZSzhRx15/Arjowhr+VVKSbaxzPtEfY0oIg2SrqJnnr/l3Du5qIefwh5VmCZe4xopPUaDDoOIEFriZ88sB+3zz8ib8sk8zJJQCgeP78tQvXCgS+4e5W3TUg9mxjB6KjXTyHIVhDZqhqde0OI3Fy1UuVzRUwnBaLjBnAwP5EoFQGRmDYk/rEYe7HTmovLeEBUDQocBQKT4Ripm/xJkkWY7B07K/tfo56dGUCkvyIVXKBInCh+dLK7gZapnd4UWkY0xBYcwo1geMLRq58iFTLA2j/JmpmHXp7m0l7jJii7d44uD3tTIFYThn7NlOnvhLim/YcBK07GMGIN7XwrrKZKmxXaspw6KBWVhzuw1UPxctxshYEaMLfFg/bwOw8HvMPr9VtrElpSB7oiOh91PDIPdPBgHCi7N2QgQ5l/ZDBHieSpNrQ== thomasrodgers
///         proxyConfig:
///           resourceGroupId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-cluster
///           secretId: https://my--dev-keyvault.vault.azure.net/secrets/my--dev-secret/0000000000000000000000000000000000
///         rootVolume:
///           sizeGib: 32
///         tags:
///           owner: mmv2
///         labels:
///           key_one: label_one
///         vmSize: Standard_DS2_v2
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: /subscriptions/12345678-1234-1234-1234-123456789111/resourceGroups/my--dev-byo/providers/Microsoft.Network/virtualNetworks/my--dev-vnet/subnets/default
///       version: ${versions.validVersions[0]}
///       annotations:
///         annotation-one: value-one
///       management:
///         autoRepair: true
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAzureVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
///
/// ## Import
///
/// NodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/azureClusters/{{cluster}}/azureNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{name}}`
///
/// * `{{location}}/{{cluster}}/{{name}}`
///
/// When using the `pulumi import` command, NodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default projects/{{project}}/locations/{{location}}/azureClusters/{{cluster}}/azureNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default {{project}}/{{location}}/{{cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureNodePool:AzureNodePool default {{location}}/{{cluster}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod azure_node_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AzureNodePoolArgs {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
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
            super::super::types::container::AzureNodePoolAutoscaling,
        >,
        /// Optional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`.
        #[builder(into, default)]
        pub azure_availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The azureCluster for the resource
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The node configuration of the node pool.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureNodePoolConfig,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Management configuration for this node pool.
        #[builder(into, default)]
        pub management: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::container::AzureNodePoolManagement>,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        #[builder(into)]
        pub max_pods_constraint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::container::AzureNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AzureNodePoolResult {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Keys can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        pub autoscaling: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureNodePoolAutoscaling,
        >,
        /// Optional. The Azure availability zone of the nodes in this nodepool. When unspecified, it defaults to `1`.
        pub azure_availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The azureCluster for the resource
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// The node configuration of the node pool.
        pub config: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureNodePoolConfig,
        >,
        /// Output only. The time at which this node pool was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Management configuration for this node pool.
        pub management: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureNodePoolManagement,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        pub max_pods_constraint: pulumi_gestalt_rust::Output<
            super::super::types::container::AzureNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. If set, there are currently pending changes to the node pool.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. The current state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The ARM ID of the subnet where the node pool VMs run. Make sure it's a subnet under the virtual network in the cluster configuration.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. A globally unique identifier for the node pool.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time at which this node pool was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AzureNodePoolArgs,
    ) -> AzureNodePoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let autoscaling_binding = args.autoscaling.get_output(context);
        let azure_availability_zone_binding = args
            .azure_availability_zone
            .get_output(context);
        let cluster_binding = args.cluster.get_output(context);
        let config_binding = args.config.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_binding = args.management.get_output(context);
        let max_pods_constraint_binding = args.max_pods_constraint.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:container/azureNodePool:AzureNodePool".into(),
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
                    name: "azureAvailabilityZone".into(),
                    value: &azure_availability_zone_binding.drop_type(),
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
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AzureNodePoolResult {
            annotations: o.get_field("annotations"),
            autoscaling: o.get_field("autoscaling"),
            azure_availability_zone: o.get_field("azureAvailabilityZone"),
            cluster: o.get_field("cluster"),
            config: o.get_field("config"),
            create_time: o.get_field("createTime"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            management: o.get_field("management"),
            max_pods_constraint: o.get_field("maxPodsConstraint"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            subnet_id: o.get_field("subnetId"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
        }
    }
}
