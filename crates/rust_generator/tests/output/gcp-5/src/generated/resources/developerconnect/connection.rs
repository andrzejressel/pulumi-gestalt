/// ## Example Usage
///
/// ### Developer Connect Connection Basic
///
///
/// ```yaml
/// resources:
///   my-connection:
///     type: gcp:developerconnect:Connection
///     properties:
///       location: us-central1
///       connectionId: tf-test-connection
///       githubConfig:
///         githubApp: DEVELOPER_CONNECT
///         authorizerCredential:
///           oauthTokenSecretVersion: projects/devconnect-terraform-creds/secrets/tf-test-do-not-change-github-oauthtoken-e0b9e7/versions/1
/// ```
/// ### Developer Connect Connection Github Doc
///
///
/// ```yaml
/// resources:
///   github-token-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: github-token-secret
///       replication:
///         auto: {}
///   github-token-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["github-token-secret"].id}
///       secretData:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: my-github-token.txt
///           return: result
///   policy:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["github-token-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   my-connection:
///     type: gcp:developerconnect:Connection
///     properties:
///       location: us-central1
///       connectionId: my-connection
///       githubConfig:
///         githubApp: DEVELOPER_CONNECT
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: ${["github-token-secret-version"].id}
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-devconnect.iam.gserviceaccount.com
/// ```
///
/// ## Import
///
/// Connection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{connection_id}}`
///
/// * `{{project}}/{{location}}/{{connection_id}}`
///
/// * `{{location}}/{{connection_id}}`
///
/// When using the `pulumi import` command, Connection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:developerconnect/connection:Connection default projects/{{project}}/locations/{{location}}/connections/{{connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:developerconnect/connection:Connection default {{project}}/{{location}}/{{connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:developerconnect/connection:Connection default {{location}}/{{connection_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// Optional. Allows clients to store small amounts of arbitrary data.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Required. Id of the requesting object. If auto-generating Id server-side,
        /// remove this field and connection_id from the method_signature of Create RPC.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Optional. This checksum is computed by the server based on the value
        /// of other fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for connections to github.com.
        /// Structure is documented below.
        #[builder(into, default)]
        pub github_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::developerconnect::ConnectionGithubConfig>,
        >,
        /// Optional. Labels as key value pairs
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource
        /// within its parent collection as described in https://google.aip.dev/122. See documentation
        /// for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Allows clients to store small amounts of arbitrary data.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Required. Id of the requesting object. If auto-generating Id server-side,
        /// remove this field and connection_id from the method_signature of Create RPC.
        ///
        ///
        /// - - -
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Create timestamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Delete timestamp
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. If disabled is set to true, functionality is disabled for this connection. Repository based API methods and webhooks processing for repositories in this connection will be disabled.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. This checksum is computed by the server based on the value
        /// of other fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration for connections to github.com.
        /// Structure is documented below.
        pub github_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::developerconnect::ConnectionGithubConfig>,
        >,
        /// Describes stage and necessary actions to be taken by the user to complete the installation.
        /// Used for GitHub and GitHub Enterprise based connections.
        /// Structure is documented below.
        pub installation_states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::developerconnect::ConnectionInstallationState>,
        >,
        /// Optional. Labels as key value pairs
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource
        /// within its parent collection as described in https://google.aip.dev/122. See documentation
        /// for resource type `developerconnect.googleapis.com/GitRepositoryLink`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The resource name of the connection, in the format
        /// `projects/{project}/locations/{location}/connections/{connection_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Set to true when the connection is being set up or updated
        /// in the background.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. A system-assigned unique identifier for a the GitRepositoryLink.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Update timestamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let connection_id_binding = args.connection_id.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let etag_binding = args.etag.get_output(context);
        let github_config_binding = args.github_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:developerconnect/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "etag".into(),
                    value: &etag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "githubConfig".into(),
                    value: &github_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionResult {
            id: o.get_field("id"),
            annotations: o.get_field("annotations"),
            connection_id: o.get_field("connectionId"),
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            disabled: o.get_field("disabled"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            github_config: o.get_field("githubConfig"),
            installation_states: o.get_field("installationStates"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
