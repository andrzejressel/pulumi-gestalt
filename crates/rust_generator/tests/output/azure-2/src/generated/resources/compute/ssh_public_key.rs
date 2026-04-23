/// Manages a SSH Public Key.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:compute:SshPublicKey
///     properties:
///       name: example
///       resourceGroupName: example
///       location: West Europe
///       publicKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ~/.ssh/id_rsa.pub
///           return: result
/// ```
///
/// ## Import
///
/// SSH Public Keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sshPublicKey:SshPublicKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/sshPublicKeys/mySshPublicKeyName1
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
        /// The Azure Region where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for this SSH Public Key. Changing this forces a new SSH Public Key to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// SSH public key used to authenticate to a virtual machine through ssh. the provided public key needs to be at least 2048-bit and in ssh-rsa format.
        #[builder(into)]
        pub public_key: pulumi_gestalt_rust::Input<String>,
        /// The name of the Resource Group where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
        /// A mapping of tags which should be assigned to the SSH Public Key.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SshPublicKeyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this SSH Public Key. Changing this forces a new SSH Public Key to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// SSH public key used to authenticate to a virtual machine through ssh. the provided public key needs to be at least 2048-bit and in ssh-rsa format.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SSH Public Key.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        let location_binding = args.location.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let public_key_binding = args.public_key.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/sshPublicKey:SshPublicKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SshPublicKeyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_key: o.get_field("publicKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
