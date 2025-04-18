/// A repository associated to a parent connection.
///
///
/// To get more information about Repository, see:
///
/// * [API documentation](https://cloud.google.com/build/docs/api/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/build/docs)
///
/// ## Example Usage
///
/// ### Cloudbuildv2 Repository Ghe Doc
///
///
/// ```yaml
/// resources:
///   private-key-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: ghe-pk-secret
///       replication:
///         auto: {}
///   private-key-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["private-key-secret"].id}
///       secretData:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: private-key.pem
///           return: result
///   webhook-secret-secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: github-token-secret
///       replication:
///         auto: {}
///   webhook-secret-secret-version:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["webhook-secret-secret"].id}
///       secretData: <webhook-secret-data>
///   policy-pk:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["private-key-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   policy-whs:
///     type: gcp:secretmanager:SecretIamPolicy
///     properties:
///       secretId: ${["webhook-secret-secret"].secretId}
///       policyData: ${["p4sa-secretAccessor"].policyData}
///   my-connection:
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-terraform-ghe-connection
///       githubEnterpriseConfig:
///         hostUri: https://ghe.com
///         privateKeySecretVersion: ${["private-key-secret-version"].id}
///         webhookSecretSecretVersion: ${["webhook-secret-secret-version"].id}
///         appId: 200
///         appSlug: gcb-app
///         appInstallationId: 300
///     options:
///       dependsOn:
///         - ${["policy-pk"]}
///         - ${["policy-whs"]}
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       name: my-terraform-ghe-repo
///       location: us-central1
///       parentConnection: ${["my-connection"].name}
///       remoteUri: https://ghe.com/hashicorp/terraform-provider-google.git
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-cloudbuild.iam.gserviceaccount.com
/// ```
/// ### Cloudbuildv2 Repository Github Doc
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
///     type: gcp:cloudbuildv2:Connection
///     properties:
///       location: us-central1
///       name: my-connection
///       githubConfig:
///         appInstallationId: 123123
///         authorizerCredential:
///           oauthTokenSecretVersion: ${["github-token-secret-version"].id}
///   my-repository:
///     type: gcp:cloudbuildv2:Repository
///     properties:
///       location: us-central1
///       name: my-repo
///       parentConnection: ${["my-connection"].name}
///       remoteUri: https://github.com/myuser/myrepo.git
/// variables:
///   p4sa-secretAccessor:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/secretmanager.secretAccessor
///             members:
///               - serviceAccount:service-123456789@gcp-sa-cloudbuild.iam.gserviceaccount.com
/// ```
///
/// ## Import
///
/// Repository can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/repositories/{{name}}`
///
/// * `{{project}}/{{location}}/{{parent_connection}}/{{name}}`
///
/// * `{{location}}/{{parent_connection}}/{{name}}`
///
/// When using the `pulumi import` command, Repository can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default projects/{{project}}/locations/{{location}}/connections/{{parent_connection}}/repositories/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default {{project}}/{{location}}/{{parent_connection}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuildv2/repository:Repository default {{location}}/{{parent_connection}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the repository.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The connection for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent_connection: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Git Clone HTTPS URI.
        #[builder(into)]
        pub remote_uri: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Allows clients to store small amounts of arbitrary data.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Server assigned timestamp for when the connection was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the repository.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The connection for the resource
        ///
        ///
        /// - - -
        pub parent_connection: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Required. Git Clone HTTPS URI.
        pub remote_uri: pulumi_gestalt_rust::Output<String>,
        /// Output only. Server assigned timestamp for when the connection was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_connection_binding = args.parent_connection.get_output(context);
        let project_binding = args.project.get_output(context);
        let remote_uri_binding = args.remote_uri.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudbuildv2/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
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
                    name: "parentConnection".into(),
                    value: &parent_connection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteUri".into(),
                    value: &remote_uri_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent_connection: o.get_field("parentConnection"),
            project: o.get_field("project"),
            remote_uri: o.get_field("remoteUri"),
            update_time: o.get_field("updateTime"),
        }
    }
}
