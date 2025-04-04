/// Manages the Pricing Tier for Azure Security Center in the current subscription.
///
/// > **NOTE:** Deletion of this resource will reset the pricing tier to `Free`
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription_pricing::create(
///         "example",
///         SubscriptionPricingArgs::builder()
///             .resource_type("VirtualMachines")
///             .tier("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Extensions with Defender CSPM
///
/// ```yaml
/// resources:
///   example1:
///     type: azure:securitycenter:SubscriptionPricing
///     properties:
///       tier: Standard
///       resourceType: CloudPosture
///       extensions:
///         - name: ContainerRegistriesVulnerabilityAssessments
///         - name: AgentlessVmScanning
///           additionalExtensionProperties:
///             ExclusionTags: '[]'
///         - name: AgentlessDiscoveryForKubernetes
///         - name: SensitiveDataDiscovery
/// ```
///
/// ## Import
///
/// The pricing tier can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/subscriptionPricing:SubscriptionPricing example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/pricings/<resource_type>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription_pricing {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionPricingArgs {
        /// One or more `extension` blocks as defined below.
        #[builder(into, default)]
        pub extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::securitycenter::SubscriptionPricingExtension>,
            >,
        >,
        /// The resource type this setting affects. Possible values are `Api`, `AppServices`, `ContainerRegistry`, `KeyVaults`, `KubernetesService`, `SqlServers`, `SqlServerVirtualMachines`, `StorageAccounts`, `VirtualMachines`, `Arm`, `Dns`, `OpenSourceRelationalDatabases`, `Containers`, `CosmosDbs` and `CloudPosture`. Defaults to `VirtualMachines`
        #[builder(into, default)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource type pricing subplan. Contact your MSFT representative for possible values.
        #[builder(into, default)]
        pub subplan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The pricing tier to use. Possible values are `Free` and `Standard`.
        #[builder(into)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionPricingResult {
        /// One or more `extension` blocks as defined below.
        pub extensions: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::securitycenter::SubscriptionPricingExtension>,
            >,
        >,
        /// The resource type this setting affects. Possible values are `Api`, `AppServices`, `ContainerRegistry`, `KeyVaults`, `KubernetesService`, `SqlServers`, `SqlServerVirtualMachines`, `StorageAccounts`, `VirtualMachines`, `Arm`, `Dns`, `OpenSourceRelationalDatabases`, `Containers`, `CosmosDbs` and `CloudPosture`. Defaults to `VirtualMachines`
        pub resource_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource type pricing subplan. Contact your MSFT representative for possible values.
        pub subplan: pulumi_gestalt_rust::Output<Option<String>>,
        /// The pricing tier to use. Possible values are `Free` and `Standard`.
        pub tier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionPricingArgs,
    ) -> SubscriptionPricingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let extensions_binding = args.extensions.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let subplan_binding = args.subplan.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/subscriptionPricing:SubscriptionPricing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensions".into(),
                    value: &extensions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subplan".into(),
                    value: &subplan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: &tier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionPricingResult {
            extensions: o.get_field("extensions"),
            resource_type: o.get_field("resourceType"),
            subplan: o.get_field("subplan"),
            tier: o.get_field("tier"),
        }
    }
}
