/// The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
/// to manage Cloudflare Workers for Platforms namespaces.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:WorkersForPlatformsNamespace
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example-namespace
///   customerWorker1:
///     type: cloudflare:WorkersScript
///     name: customer_worker_1
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: customer-worker-1
///       content:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: script.js
///           Return: result
///       dispatchNamespace: ${example.name}
///       tags:
///         - free
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace example <account_id>/<namespace_name>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod workers_for_platforms_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersForPlatformsNamespaceArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// The name of the Workers for Platforms namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersForPlatformsNamespaceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Workers for Platforms namespace.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersForPlatformsNamespaceArgs,
    ) -> WorkersForPlatformsNamespaceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersForPlatformsNamespaceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> WorkersForPlatformsNamespaceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersForPlatformsNamespaceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> WorkersForPlatformsNamespaceResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        WorkersForPlatformsNamespaceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            name: o.get_field("name"),
        }
    }
}
