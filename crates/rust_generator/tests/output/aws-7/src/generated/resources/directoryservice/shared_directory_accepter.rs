/// Accepts a shared directory in a consumer account.
///
/// > **NOTE:** Destroying this resource removes the shared directory from the consumer account only.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = shared_directory::create(
///         "example",
///         SharedDirectoryArgs::builder()
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .notes("example")
///             .target(
///                 SharedDirectoryTarget::builder()
///                     .id("${receiver.accountId}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleSharedDirectoryAccepter = shared_directory_accepter::create(
///         "exampleSharedDirectoryAccepter",
///         SharedDirectoryAccepterArgs::builder()
///             .shared_directory_id("${example.sharedDirectoryId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Shared Directories using the shared directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter example d-9267633ece
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod shared_directory_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterArgs {
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        #[builder(into)]
        pub shared_directory_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Method used when sharing a directory (i.e., `ORGANIZATIONS` or `HANDSHAKE`).
        pub method: pulumi_gestalt_rust::Output<String>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        pub notes: pulumi_gestalt_rust::Output<String>,
        /// Account identifier of the directory owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the Managed Microsoft AD directory from the perspective of the directory owner.
        pub owner_directory_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        pub shared_directory_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedDirectoryAccepterArgs,
    ) -> SharedDirectoryAccepterResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedDirectoryAccepterArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SharedDirectoryAccepterResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedDirectoryAccepterArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SharedDirectoryAccepterResult {
        let shared_directory_id_binding = args.shared_directory_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedDirectoryId".into(),
                    value: &shared_directory_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SharedDirectoryAccepterResult {
            id: o.get_id(),
            urn: o.get_urn(),
            method: o.get_field("method"),
            notes: o.get_field("notes"),
            owner_account_id: o.get_field("ownerAccountId"),
            owner_directory_id: o.get_field("ownerDirectoryId"),
            shared_directory_id: o.get_field("sharedDirectoryId"),
        }
    }
}
