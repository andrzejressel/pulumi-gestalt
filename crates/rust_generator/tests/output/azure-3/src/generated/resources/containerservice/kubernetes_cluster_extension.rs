/// Manages a Kubernetes Cluster Extension.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-aks
///       location: West Europe
///       resourceGroupName: ${example.name}
///       dnsPrefix: example-aks
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///       identity:
///         type: SystemAssigned
///   exampleKubernetesClusterExtension:
///     type: azure:containerservice:KubernetesClusterExtension
///     name: example
///     properties:
///       name: example-ext
///       clusterId: ${exampleKubernetesCluster.id}
///       extensionType: microsoft.flux
/// ```
///
/// ## Import
///
/// Kubernetes Cluster Extension can be imported using the `resource id` for different `cluster_resource_name`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/kubernetesClusterExtension:KubernetesClusterExtension example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/managedClusters/cluster1/providers/Microsoft.KubernetesConfiguration/extensions/extension1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod kubernetes_cluster_extension {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionArgs {
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_protected_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        #[builder(into, default)]
        pub configuration_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into)]
        pub extension_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub release_train: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub target_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KubernetesClusterExtensionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An `aks_assigned_identity` block as defined below.
        pub aks_assigned_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::containerservice::KubernetesClusterExtensionAksAssignedIdentity,
            >,
        >,
        /// Specifies the Cluster ID. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration settings that are sensitive, as name-value pairs for configuring this extension.
        pub configuration_protected_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration settings, as name-value pairs for configuring this extension.
        pub configuration_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The current version of the extension.
        pub current_version: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of extension. It must be one of the extension types registered with Microsoft.KubernetesConfiguration by the Extension publisher. For more information, please refer to [Available Extensions for AKS](https://learn.microsoft.com/en-us/azure/aks/cluster-extensions?tabs=azure-cli#currently-available-extensions). Changing this forces a new Kubernetes Cluster Extension to be created.
        pub extension_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Kubernetes Cluster Extension. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::KubernetesClusterExtensionPlan>,
        >,
        /// Namespace where the extension release must be placed for a cluster scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_namespace: pulumi_gestalt_rust::Output<String>,
        /// The release train used by this extension. Possible values include but are not limited to `Stable`, `Preview`. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub release_train: pulumi_gestalt_rust::Output<String>,
        /// Namespace where the extension will be created for a namespace scoped extension. If this namespace does not exist, it will be created. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub target_namespace: pulumi_gestalt_rust::Output<String>,
        /// User-specified version that the extension should pin to. If it is not set, Azure will use the latest version and auto upgrade it. Changing this forces a new Kubernetes Cluster Extension to be created.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KubernetesClusterExtensionArgs,
    ) -> KubernetesClusterExtensionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KubernetesClusterExtensionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> KubernetesClusterExtensionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KubernetesClusterExtensionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> KubernetesClusterExtensionResult {
        let cluster_id_binding = args.cluster_id.get_output(ctx);
        let configuration_protected_settings_binding = args
            .configuration_protected_settings
            .get_output(ctx);
        let configuration_settings_binding = args.configuration_settings.get_output(ctx);
        let extension_type_binding = args.extension_type.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let plan_binding = args.plan.get_output(ctx);
        let release_namespace_binding = args.release_namespace.get_output(ctx);
        let release_train_binding = args.release_train.get_output(ctx);
        let target_namespace_binding = args.target_namespace.get_output(ctx);
        let version_binding = args.version.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesClusterExtension:KubernetesClusterExtension"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationProtectedSettings".into(),
                    value: &configuration_protected_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSettings".into(),
                    value: &configuration_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionType".into(),
                    value: &extension_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseNamespace".into(),
                    value: &release_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseTrain".into(),
                    value: &release_train_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNamespace".into(),
                    value: &target_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        KubernetesClusterExtensionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            aks_assigned_identities: o.get_field("aksAssignedIdentities"),
            cluster_id: o.get_field("clusterId"),
            configuration_protected_settings: o
                .get_field("configurationProtectedSettings"),
            configuration_settings: o.get_field("configurationSettings"),
            current_version: o.get_field("currentVersion"),
            extension_type: o.get_field("extensionType"),
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            release_namespace: o.get_field("releaseNamespace"),
            release_train: o.get_field("releaseTrain"),
            target_namespace: o.get_field("targetNamespace"),
            version: o.get_field("version"),
        }
    }
}
