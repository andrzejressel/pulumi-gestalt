/// Provides a Cloudflare Worker secret resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mySecret = workers_secret::create(
///         "mySecret",
///         WorkersSecretArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("MY_EXAMPLE_SECRET_TEXT")
///             .script_name("script_1")
///             .secret_text("my_secret_value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersSecret:WorkersSecret example <account_id>/<script_name>/<secret_name>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod workers_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersSecretArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub script_name: pulumi_gestalt_rust::Input<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub secret_text: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersSecretResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
        pub script_name: pulumi_gestalt_rust::Output<String>,
        /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
        pub secret_text: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersSecretArgs,
    ) -> WorkersSecretResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersSecretArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> WorkersSecretResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersSecretArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> WorkersSecretResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let script_name_binding = args.script_name.get_output(ctx);
        let secret_text_binding = args.secret_text.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workersSecret:WorkersSecret".into(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scriptName".into(),
                    value: &script_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretText".into(),
                    value: &secret_text_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        WorkersSecretResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            name: o.get_field("name"),
            script_name: o.get_field("scriptName"),
            secret_text: o.get_field("secretText"),
        }
    }
}
