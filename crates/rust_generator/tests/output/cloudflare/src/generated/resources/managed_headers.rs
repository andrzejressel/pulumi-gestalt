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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_headers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHeadersArgs {
        /// The list of managed request headers.
        #[builder(into, default)]
        pub managed_request_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
        >,
        /// The list of managed response headers.
        #[builder(into, default)]
        pub managed_response_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedHeadersResult {
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedHeadersArgs,
    ) -> ManagedHeadersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_request_headers_binding = args
            .managed_request_headers
            .get_output(context);
        let managed_response_headers_binding = args
            .managed_response_headers
            .get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
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
        };
        let o = context.register_resource(request);
        ManagedHeadersResult {
            managed_request_headers: o.get_field("managedRequestHeaders"),
            managed_response_headers: o.get_field("managedResponseHeaders"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
