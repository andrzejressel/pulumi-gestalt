/// ## Example Usage
///
/// ```yaml
/// resources:
///   fooServer:
///     type: aws:transfer:Server
///     name: foo
///     properties:
///       identityProviderType: SERVICE_MANAGED
///       tags:
///         NAME: tf-acc-test-transfer-server
///   fooRole:
///     type: aws:iam:Role
///     name: foo
///     properties:
///       name: tf-test-transfer-user-iam-role
///       assumeRolePolicy: ${assumeRole.json}
///   fooRolePolicy:
///     type: aws:iam:RolePolicy
///     name: foo
///     properties:
///       name: tf-test-transfer-user-iam-policy
///       role: ${fooRole.id}
///       policy: ${foo.json}
///   fooUser:
///     type: aws:transfer:User
///     name: foo
///     properties:
///       serverId: ${fooServer.id}
///       userName: tftestuser
///       role: ${fooRole.arn}
///       homeDirectoryType: LOGICAL
///       homeDirectoryMappings:
///         - entry: /test.pdf
///           target: /bucket3/test-path/tftestuser.pdf
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - transfer.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   foo:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: AllowFullAccesstoS3
///             effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer Users using the `server_id` and `user_name` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/user:User bar s-12345678/test-username
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
        #[builder(into, default)]
        pub home_directory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
        #[builder(into, default)]
        pub home_directory_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::transfer::UserHomeDirectoryMapping>>,
        >,
        /// The type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
        #[builder(into, default)]
        pub home_directory_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
        #[builder(into, default)]
        pub posix_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::UserPosixProfile>,
        >,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to control your user’s access to your Amazon S3 bucket.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name used for log in to your SFTP server.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Amazon Resource Name (ARN) of Transfer User
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
        pub home_directory: pulumi_gestalt_rust::Output<Option<String>>,
        /// Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
        pub home_directory_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::transfer::UserHomeDirectoryMapping>>,
        >,
        /// The type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
        pub home_directory_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
        pub policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
        pub posix_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::UserPosixProfile>,
        >,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to control your user’s access to your Amazon S3 bucket.
        pub role: pulumi_gestalt_rust::Output<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        pub server_id: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name used for log in to your SFTP server.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let home_directory_binding = args.home_directory.get_output(context);
        let home_directory_mappings_binding = args
            .home_directory_mappings
            .get_output(context);
        let home_directory_type_binding = args.home_directory_type.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let posix_profile_binding = args.posix_profile.get_output(context);
        let role_binding = args.role.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectory".into(),
                    value: &home_directory_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectoryMappings".into(),
                    value: &home_directory_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectoryType".into(),
                    value: &home_directory_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "posixProfile".into(),
                    value: &posix_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            arn: o.get_field("arn"),
            home_directory: o.get_field("homeDirectory"),
            home_directory_mappings: o.get_field("homeDirectoryMappings"),
            home_directory_type: o.get_field("homeDirectoryType"),
            policy: o.get_field("policy"),
            posix_profile: o.get_field("posixProfile"),
            role: o.get_field("role"),
            server_id: o.get_field("serverId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_name: o.get_field("userName"),
        }
    }
}
