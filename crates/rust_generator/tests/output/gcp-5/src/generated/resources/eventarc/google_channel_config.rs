/// The Eventarc GoogleChannelConfig resource
///
/// ## Example Usage
///
/// ### Basic
/// ```yaml
/// resources:
///   key1Member:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: key1_member
///     properties:
///       cryptoKeyId: ${key1.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${testProject.number}@gcp-sa-eventarc.iam.gserviceaccount.com
///   primary:
///     type: gcp:eventarc:GoogleChannelConfig
///     properties:
///       location: us-west1
///       name: channel
///       project: ${testProject.projectId}
///       cryptoKeyName: ${key1.id}
///     options:
///       dependsOn:
///         - ${key1Member}
/// variables:
///   testProject:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments:
///         projectId: my-project-name
///   testKeyRing:
///     fn::invoke:
///       function: gcp:kms:getKMSKeyRing
///       arguments:
///         name: keyring
///         location: us-west1
///   key:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKey
///       arguments:
///         name: key
///         keyRing: ${testKeyRing.id}
/// ```
///
/// ## Import
///
/// GoogleChannelConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/googleChannelConfig`
///
/// * `{{project}}/{{location}}`
///
/// * `{{location}}`
///
/// When using the `pulumi import` command, GoogleChannelConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default projects/{{project}}/locations/{{location}}/googleChannelConfig
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default {{project}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:eventarc/googleChannelConfig:GoogleChannelConfig default {{location}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod google_channel_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GoogleChannelConfigArgs {
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        #[builder(into, default)]
        pub crypto_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GoogleChannelConfigResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
        pub crypto_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. The last-modified time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GoogleChannelConfigArgs,
    ) -> GoogleChannelConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_key_name_binding = args.crypto_key_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:eventarc/googleChannelConfig:GoogleChannelConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyName".into(),
                    value: &crypto_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GoogleChannelConfigResult {
            id: o.get_field("id"),
            crypto_key_name: o.get_field("cryptoKeyName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            update_time: o.get_field("updateTime"),
        }
    }
}
