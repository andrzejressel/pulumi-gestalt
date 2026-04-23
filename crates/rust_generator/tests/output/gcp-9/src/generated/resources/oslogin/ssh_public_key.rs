/// The SSH public key information associated with a Google account.
///
///
/// To get more information about SSHPublicKey, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/oslogin/rest/v1/users.sshPublicKeys)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/oslogin)
///
/// ## Example Usage
///
/// ### Os Login Ssh Key Basic
///
///
/// ```yaml
/// resources:
///   cache:
///     type: gcp:oslogin:SshPublicKey
///     properties:
///       user: ${me.email}
///       key:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/id_rsa.pub
///           return: result
/// variables:
///   me:
///     fn::invoke:
///       function: gcp:organizations:getClientOpenIdUserInfo
///       arguments: {}
/// ```
///
/// ## Import
///
/// SSHPublicKey can be imported using any of these accepted formats:
///
/// * `users/{{user}}/sshPublicKeys/{{fingerprint}}`
///
/// * `{{user}}/{{fingerprint}}`
///
/// When using the `pulumi import` command, SSHPublicKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:oslogin/sshPublicKey:SshPublicKey default users/{{user}}/sshPublicKeys/{{fingerprint}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oslogin/sshPublicKey:SshPublicKey default {{user}}/{{fingerprint}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod ssh_public_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshPublicKeyArgs {
        /// An expiration time in microseconds since epoch.
        #[builder(into, default)]
        pub expiration_time_usec: pulumi_gestalt_rust::Input<Option<String>>,
        /// Public key text in SSH format, defined by RFC4253 section 6.6.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::Input<String>,
        /// The project ID of the Google Cloud Platform project.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The user email.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub user: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SshPublicKeyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An expiration time in microseconds since epoch.
        pub expiration_time_usec: pulumi_gestalt_rust::Output<Option<String>>,
        /// The SHA-256 fingerprint of the SSH public key.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Public key text in SSH format, defined by RFC4253 section 6.6.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The project ID of the Google Cloud Platform project.
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user email.
        ///
        ///
        /// - - -
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SshPublicKeyArgs,
    ) -> SshPublicKeyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SshPublicKeyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SshPublicKeyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SshPublicKeyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SshPublicKeyResult {
        let expiration_time_usec_binding = args.expiration_time_usec.get_output(ctx);
        let key_binding = args.key.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let user_binding = args.user.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:oslogin/sshPublicKey:SshPublicKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expirationTimeUsec".into(),
                    value: &expiration_time_usec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SshPublicKeyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            expiration_time_usec: o.get_field("expirationTimeUsec"),
            fingerprint: o.get_field("fingerprint"),
            key: o.get_field("key"),
            project: o.get_field("project"),
            user: o.get_field("user"),
        }
    }
}
