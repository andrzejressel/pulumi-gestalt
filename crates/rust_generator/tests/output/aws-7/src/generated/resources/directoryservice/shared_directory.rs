/// Manages a directory in your account (directory owner) shared with another account (directory consumer).
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Shared Directories using the owner directory ID/shared directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/sharedDirectory:SharedDirectory example d-1234567890/d-9267633ece
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedDirectoryArgs {
        /// Identifier of the Managed Microsoft AD directory that you want to share with other accounts.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Method used when sharing a directory. Valid values are `ORGANIZATIONS` and `HANDSHAKE`. Default is `HANDSHAKE`.
        #[builder(into, default)]
        pub method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        #[builder(into, default)]
        pub notes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier for the directory consumer account with whom the directory is to be shared. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::directoryservice::SharedDirectoryTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct SharedDirectoryResult {
        /// Identifier of the Managed Microsoft AD directory that you want to share with other accounts.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// Method used when sharing a directory. Valid values are `ORGANIZATIONS` and `HANDSHAKE`. Default is `HANDSHAKE`.
        pub method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        pub notes: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        pub shared_directory_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the directory consumer account with whom the directory is to be shared. See below.
        ///
        /// The following arguments are optional:
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::directoryservice::SharedDirectoryTarget,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedDirectoryArgs,
    ) -> SharedDirectoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let method_binding = args.method.get_output(context);
        let notes_binding = args.notes.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/sharedDirectory:SharedDirectory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: &method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notes".into(),
                    value: &notes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedDirectoryResult {
            directory_id: o.get_field("directoryId"),
            method: o.get_field("method"),
            notes: o.get_field("notes"),
            shared_directory_id: o.get_field("sharedDirectoryId"),
            target: o.get_field("target"),
        }
    }
}
