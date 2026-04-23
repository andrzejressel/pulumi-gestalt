/// Provides a Cloudflare worker route resource. A route will also require a `cloudflare.WorkerScript`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myRoute = workers_route::create(
///         "myRoute",
///         WorkersRouteArgs::builder()
///             .pattern("example.com/*")
///             .script_name("${myScript.name}")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myScript = workers_script::create(
///         "myScript",
///         WorkersScriptArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersRoute:WorkersRoute example <zone_id>/<route_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod workers_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersRouteArgs {
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::Input<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        #[builder(into, default)]
        pub script_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersRouteResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
        pub pattern: pulumi_gestalt_rust::Output<String>,
        /// Worker script name to invoke for requests that match the route pattern.
        pub script_name: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: WorkersRouteArgs,
    ) -> WorkersRouteResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersRouteArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> WorkersRouteResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersRouteArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> WorkersRouteResult {
        let pattern_binding = args.pattern.get_output(ctx);
        let script_name_binding = args.script_name.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workersRoute:WorkersRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        WorkersRouteResult {
            id: o.get_id(),
            urn: o.get_urn(),
            pattern: o.get_field("pattern"),
            script_name: o.get_field("scriptName"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
