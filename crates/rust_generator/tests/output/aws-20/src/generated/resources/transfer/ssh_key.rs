/// Provides a AWS Transfer User SSH Key resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   examplePrivateKey:
///     type: tls:PrivateKey
///     name: example
///     properties:
///       algorithm: RSA
///       rsaBits: 4096
///   exampleSshKey:
///     type: aws:transfer:SshKey
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
///       userName: ${exampleUser.userName}
///       body:
///         fn::invoke:
///           function: std:trimspace
///           arguments:
///             input: ${examplePrivateKey.publicKeyOpenssh}
///           return: result
///   exampleServer:
///     type: aws:transfer:Server
///     name: example
///     properties:
///       identityProviderType: SERVICE_MANAGED
///       tags:
///         NAME: tf-acc-test-transfer-server
///   exampleUser:
///     type: aws:transfer:User
///     name: example
///     properties:
///       serverId: ${exampleServer.id}
///       userName: tftestuser
///       role: ${exampleRole.arn}
///       tags:
///         NAME: tftestuser
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: tf-test-transfer-user-iam-role
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: tf-test-transfer-user-iam-policy
///       role: ${exampleRole.id}
///       policy: ${example.json}
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
///   example:
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
/// Using `pulumi import`, import Transfer SSH Public Key using the `server_id` and `user_name` and `ssh_public_key_id` separated by `/`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/sshKey:SshKey bar s-12345678/test-username/key-12345
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssh_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshKeyArgs {
        /// The public key portion of an SSH key pair.
        #[builder(into)]
        pub body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the user account that is assigned to one or more servers.
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SshKeyResult {
        /// The public key portion of an SSH key pair.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        pub server_id: pulumi_gestalt_rust::Output<String>,
        pub ssh_key_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the user account that is assigned to one or more servers.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SshKeyArgs,
    ) -> SshKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let body_binding = args.body.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/sshKey:SshKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "body".into(),
                    value: &body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SshKeyResult {
            body: o.get_field("body"),
            server_id: o.get_field("serverId"),
            ssh_key_id: o.get_field("sshKeyId"),
            user_name: o.get_field("userName"),
        }
    }
}
