/// BitbucketServerConfig represents the configuration for a Bitbucket Server.
///
///
/// To get more information about BitbucketServerConfig, see:
///
/// * [API documentation](https://cloud.google.com/build/docs/api/reference/rest/v1/projects.locations.bitbucketServerConfigs)
/// * How-to Guides
///     * [Connect to a Bitbucket Server host](https://cloud.google.com/build/docs/automating-builds/bitbucket/connect-host-bitbucket-server)
///
/// ## Example Usage
///
/// ### Cloudbuild Bitbucket Server Config
///
///
/// ```yaml
/// resources:
///   bbs-config:
///     type: gcp:cloudbuild:BitbucketServerConfig
///     properties:
///       configId: bbs-config
///       location: us-central1
///       hostUri: https://bbs.com
///       secrets:
///         adminAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         readAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         webhookSecretVersionName: projects/myProject/secrets/mybbspat/versions/1
///       username: test
///       apiKey: <api-key>
/// ```
/// ### Cloudbuild Bitbucket Server Config Repositories
///
///
/// ```yaml
/// resources:
///   bbs-config-with-repos:
///     type: gcp:cloudbuild:BitbucketServerConfig
///     properties:
///       configId: bbs-config
///       location: us-central1
///       hostUri: https://bbs.com
///       secrets:
///         adminAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         readAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         webhookSecretVersionName: projects/myProject/secrets/mybbspat/versions/1
///       username: test
///       apiKey: <api-key>
///       connectedRepositories:
///         - projectKey: DEV
///           repoSlug: repo1
///         - projectKey: PROD
///           repoSlug: repo1
/// ```
/// ### Cloudbuild Bitbucket Server Config Peered Network
///
///
/// ```yaml
/// resources:
///   servicenetworking:
///     type: gcp:projects:Service
///     properties:
///       service: servicenetworking.googleapis.com
///       disableOnDestroy: false
///   vpcNetwork:
///     type: gcp:compute:Network
///     name: vpc_network
///     properties:
///       name: vpc-network
///     options:
///       dependsOn:
///         - ${servicenetworking}
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: private-ip-alloc
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${vpcNetwork.id}
///   default:
///     type: gcp:servicenetworking:Connection
///     properties:
///       network: ${vpcNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///     options:
///       dependsOn:
///         - ${servicenetworking}
///   bbs-config-with-peered-network:
///     type: gcp:cloudbuild:BitbucketServerConfig
///     properties:
///       configId: bbs-config
///       location: us-central1
///       hostUri: https://bbs.com
///       secrets:
///         adminAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         readAccessTokenVersionName: projects/myProject/secrets/mybbspat/versions/1
///         webhookSecretVersionName: projects/myProject/secrets/mybbspat/versions/1
///       username: test
///       apiKey: <api-key>
///       peeredNetwork:
///         fn::invoke:
///           function: std:replace
///           arguments:
///             text: ${vpcNetwork.id}
///             search: ${project.name}
///             replace: ${project.number}
///           return: result
///       sslCa: |
///         -----BEGIN CERTIFICATE-----
///         -----END CERTIFICATE-----
///         -----BEGIN CERTIFICATE-----
///         -----END CERTIFICATE-----
///     options:
///       dependsOn:
///         - ${default}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// BitbucketServerConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/bitbucketServerConfigs/{{config_id}}`
///
/// * `{{project}}/{{location}}/{{config_id}}`
///
/// * `{{location}}/{{config_id}}`
///
/// When using the `pulumi import` command, BitbucketServerConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/bitbucketServerConfig:BitbucketServerConfig default projects/{{project}}/locations/{{location}}/bitbucketServerConfigs/{{config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/bitbucketServerConfig:BitbucketServerConfig default {{project}}/{{location}}/{{config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/bitbucketServerConfig:BitbucketServerConfig default {{location}}/{{config_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bitbucket_server_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BitbucketServerConfigArgs {
        /// Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed.
        /// Changing this field will result in deleting/ recreating the resource.
        #[builder(into)]
        pub api_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID to use for the BitbucketServerConfig, which will become the final component of the BitbucketServerConfig's resource name.
        #[builder(into)]
        pub config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Connected Bitbucket Server repositories for this config.
        #[builder(into, default)]
        pub connected_repositories: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::cloudbuild::BitbucketServerConfigConnectedRepository,
                >,
            >,
        >,
        /// Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed.
        /// If you need to change it, please create another BitbucketServerConfig.
        #[builder(into)]
        pub host_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of this bitbucket server config.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private
        /// service connection. This should be set if the Bitbucket Server instance is hosted on-premises and not reachable by
        /// public internet. If this field is left empty, no network peering will occur and calls to the Bitbucket Server instance
        /// will be made over the public internet. Must be in the format projects/{project}/global/networks/{network}, where
        /// {project} is a project number or id and {network} is the name of a VPC network in the project.
        #[builder(into, default)]
        pub peered_network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Secret Manager secrets needed by the config.
        /// Structure is documented below.
        #[builder(into)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudbuild::BitbucketServerConfigSecrets,
        >,
        /// SSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of
        /// .pem, .cer, or .crt.
        #[builder(into, default)]
        pub ssl_ca: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Username of the account Cloud Build will use on Bitbucket Server.
        #[builder(into)]
        pub username: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BitbucketServerConfigResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed.
        /// Changing this field will result in deleting/ recreating the resource.
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// The ID to use for the BitbucketServerConfig, which will become the final component of the BitbucketServerConfig's resource name.
        pub config_id: pulumi_gestalt_rust::Output<String>,
        /// Connected Bitbucket Server repositories for this config.
        pub connected_repositories: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::cloudbuild::BitbucketServerConfigConnectedRepository,
                >,
            >,
        >,
        /// Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed.
        /// If you need to change it, please create another BitbucketServerConfig.
        pub host_uri: pulumi_gestalt_rust::Output<String>,
        /// The location of this bitbucket server config.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the config.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private
        /// service connection. This should be set if the Bitbucket Server instance is hosted on-premises and not reachable by
        /// public internet. If this field is left empty, no network peering will occur and calls to the Bitbucket Server instance
        /// will be made over the public internet. Must be in the format projects/{project}/global/networks/{network}, where
        /// {project} is a project number or id and {network} is the name of a VPC network in the project.
        pub peered_network: pulumi_gestalt_rust::Output<Option<String>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Secret Manager secrets needed by the config.
        /// Structure is documented below.
        pub secrets: pulumi_gestalt_rust::Output<
            super::super::types::cloudbuild::BitbucketServerConfigSecrets,
        >,
        /// SSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of
        /// .pem, .cer, or .crt.
        pub ssl_ca: pulumi_gestalt_rust::Output<Option<String>>,
        /// Username of the account Cloud Build will use on Bitbucket Server.
        pub username: pulumi_gestalt_rust::Output<String>,
        /// Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config.
        pub webhook_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BitbucketServerConfigArgs,
    ) -> BitbucketServerConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_key_binding = args.api_key.get_output(context);
        let config_id_binding = args.config_id.get_output(context);
        let connected_repositories_binding = args
            .connected_repositories
            .get_output(context);
        let host_uri_binding = args.host_uri.get_output(context);
        let location_binding = args.location.get_output(context);
        let peered_network_binding = args.peered_network.get_output(context);
        let project_binding = args.project.get_output(context);
        let secrets_binding = args.secrets.get_output(context);
        let ssl_ca_binding = args.ssl_ca.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudbuild/bitbucketServerConfig:BitbucketServerConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKey".into(),
                    value: &api_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configId".into(),
                    value: &config_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectedRepositories".into(),
                    value: &connected_repositories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostUri".into(),
                    value: &host_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "peeredNetwork".into(),
                    value: &peered_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslCa".into(),
                    value: &ssl_ca_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BitbucketServerConfigResult {
            id: o.get_field("id"),
            api_key: o.get_field("apiKey"),
            config_id: o.get_field("configId"),
            connected_repositories: o.get_field("connectedRepositories"),
            host_uri: o.get_field("hostUri"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            peered_network: o.get_field("peeredNetwork"),
            project: o.get_field("project"),
            secrets: o.get_field("secrets"),
            ssl_ca: o.get_field("sslCa"),
            username: o.get_field("username"),
            webhook_key: o.get_field("webhookKey"),
        }
    }
}
