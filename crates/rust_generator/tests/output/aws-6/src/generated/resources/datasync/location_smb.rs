/// Manages a SMB Location within AWS DataSync.
///
/// > **NOTE:** The DataSync Agents must be available before creating this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = location_smb::create(
///         "example",
///         LocationSmbArgs::builder()
///             .agent_arns(vec!["${exampleAwsDatasyncAgent.arn}",])
///             .password("ANotGreatPassword")
///             .server_hostname("smb.example.com")
///             .subdirectory("/exported/path")
///             .user("Guest")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_smb` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationSmb:LocationSmb example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_smb {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationSmbArgs {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of the Windows domain the SMB server belongs to.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing mount options used by DataSync to access the SMB Server. Can be `AUTOMATIC`, `SMB2`, or `SMB3`.
        #[builder(into, default)]
        pub mount_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::LocationSmbMountOptions>,
        >,
        /// The password of the user who can mount the share and has file permissions in the SMB.
        #[builder(into)]
        pub password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the IP address or DNS name of the SMB server. The DataSync Agent(s) use this to mount the SMB share.
        #[builder(into)]
        pub server_hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        #[builder(into)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The user who can mount the share and has file and folder permissions in the SMB share.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocationSmbResult {
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Windows domain the SMB server belongs to.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Configuration block containing mount options used by DataSync to access the SMB Server. Can be `AUTOMATIC`, `SMB2`, or `SMB3`.
        pub mount_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::LocationSmbMountOptions>,
        >,
        /// The password of the user who can mount the share and has file permissions in the SMB.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Specifies the IP address or DNS name of the SMB server. The DataSync Agent(s) use this to mount the SMB share.
        pub server_hostname: pulumi_gestalt_rust::Output<String>,
        /// Subdirectory to perform actions as source or destination. Should be exported by the NFS server.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_gestalt_rust::Output<String>,
        /// The user who can mount the share and has file and folder permissions in the SMB share.
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationSmbArgs,
    ) -> LocationSmbResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_arns_binding = args.agent_arns.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let mount_options_binding = args.mount_options.get_output(context);
        let password_binding = args.password.get_output(context);
        let server_hostname_binding = args.server_hostname.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationSmb:LocationSmb".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentArns".into(),
                    value: &agent_arns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mountOptions".into(),
                    value: &mount_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverHostname".into(),
                    value: &server_hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: &subdirectory_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationSmbResult {
            agent_arns: o.get_field("agentArns"),
            arn: o.get_field("arn"),
            domain: o.get_field("domain"),
            mount_options: o.get_field("mountOptions"),
            password: o.get_field("password"),
            server_hostname: o.get_field("serverHostname"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
            user: o.get_field("user"),
        }
    }
}
