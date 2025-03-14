/// `Ekm Connections` are used to control the connection settings for an `EXTERNAL_VPC` CryptoKey.
/// It is used to connect customer's external key manager to Google Cloud EKM.
///
///
/// > **Note:** Ekm Connections cannot be deleted from Google Cloud Platform.
///
///
/// To get more information about EkmConnection, see:
///
/// * [API documentation](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.ekmConnections)
/// * How-to Guides
///     * [Creating a Ekm Connection](https://cloud.google.com/kms/docs/create-ekm-connection)
///
/// ## Example Usage
///
/// ### Kms Ekm Connection Basic
///
///
/// ```yaml
/// resources:
///   example-ekmconnection:
///     type: gcp:kms:EkmConnection
///     properties:
///       name: ekmconnection_example
///       location: us-central1
///       keyManagementMode: MANUAL
///       serviceResolvers:
///         - serviceDirectoryService: projects/project_id/locations/us-central1/namespaces/namespace_name/services/service_name
///           hostname: example-ekm.goog
///           serverCertificates:
///             - rawDer: ==HAwIBCCAr6gAwIBAgIUWR+EV4lqiV7Ql12VY==
/// ```
///
/// ## Import
///
/// EkmConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/ekmConnections/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, EkmConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default projects/{{project}}/locations/{{location}}/ekmConnections/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:kms/ekmConnection:EkmConnection default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ekm_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EkmConnectionArgs {
        /// Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if
        /// KeyManagementMode is CLOUD_KMS.
        #[builder(into, default)]
        pub crypto_space_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Etag of the currently stored EkmConnection.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default
        /// value: "MANUAL" Possible values: ["MANUAL", "CLOUD_KMS"]
        #[builder(into, default)]
        pub key_management_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name for the EkmConnection.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported
        /// Structure is documented below.
        #[builder(into)]
        pub service_resolvers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::kms::EkmConnectionServiceResolver>,
        >,
    }
    #[allow(dead_code)]
    pub struct EkmConnectionResult {
        /// Output only. The time at which the EkmConnection was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if
        /// KeyManagementMode is CLOUD_KMS.
        pub crypto_space_path: pulumi_gestalt_rust::Output<String>,
        /// Optional. Etag of the currently stored EkmConnection.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default
        /// value: "MANUAL" Possible values: ["MANUAL", "CLOUD_KMS"]
        pub key_management_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location for the EkmConnection.
        /// A full list of valid locations can be found by running `gcloud kms locations list`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the EkmConnection.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported
        /// Structure is documented below.
        pub service_resolvers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kms::EkmConnectionServiceResolver>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EkmConnectionArgs,
    ) -> EkmConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crypto_space_path_binding = args.crypto_space_path.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let key_management_mode_binding = args.key_management_mode.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_resolvers_binding = args.service_resolvers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:kms/ekmConnection:EkmConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoSpacePath".into(),
                    value: &crypto_space_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "etag".into(),
                    value: &etag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyManagementMode".into(),
                    value: &key_management_mode_binding.drop_type(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceResolvers".into(),
                    value: &service_resolvers_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EkmConnectionResult {
            create_time: o.get_field("createTime"),
            crypto_space_path: o.get_field("cryptoSpacePath"),
            etag: o.get_field("etag"),
            key_management_mode: o.get_field("keyManagementMode"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_resolvers: o.get_field("serviceResolvers"),
        }
    }
}
