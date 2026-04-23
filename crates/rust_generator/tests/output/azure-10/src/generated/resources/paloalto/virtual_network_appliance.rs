#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod virtual_network_appliance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkApplianceArgs {
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkApplianceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkApplianceArgs,
    ) -> VirtualNetworkApplianceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkApplianceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> VirtualNetworkApplianceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkApplianceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> VirtualNetworkApplianceResult {
        let name_binding = args.name.get_output(ctx);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/virtualNetworkAppliance:VirtualNetworkAppliance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: &virtual_hub_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        VirtualNetworkApplianceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
