/// Provides a Cloudflare Fallback Domain resource. Fallback domains are
/// used to ignore DNS requests to a given list of domains. These DNS
/// requests will be passed back to other DNS servers configured on
/// existing network interfaces on the device.
///
/// ## Import
///
/// Fallback Domains for default device policies must use "default" as the policy ID.
///
/// ```sh
/// $ pulumi import cloudflare:index/fallbackDomain:FallbackDomain example <account_id>/<policy_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod fallback_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FallbackDomainArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        #[builder(into)]
        pub domains: pulumi_gestalt_rust::Input<Vec<super::types::FallbackDomainDomain>>,
        /// The settings policy for which to configure this fallback domain policy.
        #[builder(into, default)]
        pub policy_id: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FallbackDomainResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        pub domains: pulumi_gestalt_rust::Output<
            Vec<super::types::FallbackDomainDomain>,
        >,
        /// The settings policy for which to configure this fallback domain policy.
        pub policy_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FallbackDomainArgs,
    ) -> FallbackDomainResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FallbackDomainArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FallbackDomainResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FallbackDomainArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FallbackDomainResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let domains_binding = args.domains.get_output(ctx);
        let policy_id_binding = args.policy_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/fallbackDomain:FallbackDomain".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domains".into(),
                    value: &domains_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FallbackDomainResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            domains: o.get_field("domains"),
            policy_id: o.get_field("policyId"),
        }
    }
}
