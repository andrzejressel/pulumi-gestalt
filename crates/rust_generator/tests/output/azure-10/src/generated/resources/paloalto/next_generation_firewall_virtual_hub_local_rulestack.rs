#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod next_generation_firewall_virtual_hub_local_rulestack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackArgs {
        #[builder(into, default)]
        pub destination_nats: pulumi_gestalt_rust::Input<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        #[builder(into, default)]
        pub dns_settings: pulumi_gestalt_rust::Input<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub network_profile: pulumi_gestalt_rust::Input<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        #[builder(into)]
        pub rulestack_id: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubLocalRulestackResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub destination_nats: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDestinationNat,
                >,
            >,
        >,
        pub dns_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackDnsSettings,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_profile: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub rulestack_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NextGenerationFirewallVirtualHubLocalRulestackArgs,
    ) -> NextGenerationFirewallVirtualHubLocalRulestackResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NextGenerationFirewallVirtualHubLocalRulestackArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> NextGenerationFirewallVirtualHubLocalRulestackResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NextGenerationFirewallVirtualHubLocalRulestackArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> NextGenerationFirewallVirtualHubLocalRulestackResult {
        let destination_nats_binding = args.destination_nats.get_output(ctx);
        let dns_settings_binding = args.dns_settings.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let network_profile_binding = args.network_profile.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let rulestack_id_binding = args.rulestack_id.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualHubLocalRulestack:NextGenerationFirewallVirtualHubLocalRulestack"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationNats".into(),
                    value: &destination_nats_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSettings".into(),
                    value: &dns_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkProfile".into(),
                    value: &network_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rulestackId".into(),
                    value: &rulestack_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        NextGenerationFirewallVirtualHubLocalRulestackResult {
            id: o.get_id(),
            urn: o.get_urn(),
            destination_nats: o.get_field("destinationNats"),
            dns_settings: o.get_field("dnsSettings"),
            name: o.get_field("name"),
            network_profile: o.get_field("networkProfile"),
            resource_group_name: o.get_field("resourceGroupName"),
            rulestack_id: o.get_field("rulestackId"),
            tags: o.get_field("tags"),
        }
    }
}
