/// Provides a resource to manage API Shield configurations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_shield::create(
///         "example",
///         ApiShieldArgs::builder()
///             .auth_id_characteristics(
///                 vec![
///                     ApiShieldAuthIdCharacteristic::builder().name("my-example-header").
///                     type ("header").build_struct(),
///                 ],
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod api_shield {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiShieldArgs {
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        #[builder(into, default)]
        pub auth_id_characteristics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiShieldResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
        pub auth_id_characteristics: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
        >,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiShieldArgs,
    ) -> ApiShieldResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiShieldArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ApiShieldResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiShieldArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ApiShieldResult {
        let auth_id_characteristics_binding = args
            .auth_id_characteristics
            .get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/apiShield:ApiShield".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authIdCharacteristics".into(),
                    value: &auth_id_characteristics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ApiShieldResult {
            id: o.get_id(),
            urn: o.get_urn(),
            auth_id_characteristics: o.get_field("authIdCharacteristics"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
