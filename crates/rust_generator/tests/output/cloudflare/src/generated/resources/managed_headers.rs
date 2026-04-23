/// The [Cloudflare Managed Headers](https://developers.cloudflare.com/rules/transform/managed-transforms/)
/// allows you to add or remove some predefined headers to one's
/// requests or origin responses.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = managed_headers::create(
///         "example",
///         ManagedHeadersArgs::builder()
///             .managed_request_headers(
///                 vec![
///                     ManagedHeadersManagedRequestHeader::builder().enabled(true)
///                     .id("add_true_client_ip_headers").build_struct(),
///                 ],
///             )
///             .managed_response_headers(
///                 vec![
///                     ManagedHeadersManagedResponseHeader::builder().enabled(true)
///                     .id("remove_x-powered-by_header").build_struct(),
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
pub mod managed_headers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHeadersArgs {
        /// The list of managed request headers.
        #[builder(into, default)]
        pub managed_request_headers: pulumi_gestalt_rust::Input<
            Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
        >,
        /// The list of managed response headers.
        #[builder(into, default)]
        pub managed_response_headers: pulumi_gestalt_rust::Input<
            Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHeadersResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The list of managed request headers.
        pub managed_request_headers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
        >,
        /// The list of managed response headers.
        pub managed_response_headers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
        >,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHeadersArgs,
    ) -> ManagedHeadersResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHeadersArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ManagedHeadersResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHeadersArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ManagedHeadersResult {
        let managed_request_headers_binding = args
            .managed_request_headers
            .get_output(ctx);
        let managed_response_headers_binding = args
            .managed_response_headers
            .get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/managedHeaders:ManagedHeaders".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedRequestHeaders".into(),
                    value: &managed_request_headers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedResponseHeaders".into(),
                    value: &managed_response_headers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ManagedHeadersResult {
            id: o.get_id(),
            urn: o.get_urn(),
            managed_request_headers: o.get_field("managedRequestHeaders"),
            managed_response_headers: o.get_field("managedResponseHeaders"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
