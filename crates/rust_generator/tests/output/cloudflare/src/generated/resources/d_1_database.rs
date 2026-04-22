/// The [D1 Database](https://developers.cloudflare.com/d1/) resource allows you to manage Cloudflare D1 databases.
///
/// !> When a D1 Database is replaced all the data is lost. Please ensure you have a
///    backup of your data before replacing a D1 Database.
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = d_1_database::create(
///         "example",
///         D1DatabaseArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .name("terraform-database")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/d1Database:D1Database example <account id>/<database id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod d_1_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct D1DatabaseArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// The name of the D1 Database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct D1DatabaseResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the D1 Database.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The backend version of the database.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: D1DatabaseArgs,
    ) -> D1DatabaseResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: D1DatabaseArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> D1DatabaseResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: D1DatabaseArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> D1DatabaseResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/d1Database:D1Database".into(),
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
        D1DatabaseResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            name: o.get_field("name"),
            version: o.get_field("version"),
        }
    }
}
