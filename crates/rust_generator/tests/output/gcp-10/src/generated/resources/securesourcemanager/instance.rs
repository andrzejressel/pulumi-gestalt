/// Instances are deployed to an available Google Cloud region and are accessible via their web interface.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/secure-source-manager/docs/create-instance)
///
/// ## Example Usage
///
/// ### Secure Source Manager Instance Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:securesourcemanager:Instance
///     properties:
///       location: us-central1
///       instanceId: my-instance
///       labels:
///         foo: bar
/// ```
/// ### Secure Source Manager Instance Cmek
///
///
/// ```yaml
/// resources:
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: my-keyring
///       location: us-central1
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: my-key
///       keyRing: ${keyRing.id}
///   cryptoKeyBinding:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key_binding
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-sourcemanager.iam.gserviceaccount.com
///   default:
///     type: gcp:securesourcemanager:Instance
///     properties:
///       location: us-central1
///       instanceId: my-instance
///       kmsKey: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${cryptoKeyBinding}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Secure Source Manager Instance Private
///
///
/// ```yaml
/// resources:
///   caPool:
///     type: gcp:certificateauthority:CaPool
///     name: ca_pool
///     properties:
///       name: ca-pool
///       location: us-central1
///       tier: ENTERPRISE
///       publishingOptions:
///         publishCaCert: true
///         publishCrl: true
///   rootCa:
///     type: gcp:certificateauthority:Authority
///     name: root_ca
///     properties:
///       pool: ${caPool.name}
///       certificateAuthorityId: root-ca
///       location: us-central1
///       config:
///         subjectConfig:
///           subject:
///             organization: google
///             commonName: my-certificate-authority
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       ignoreActiveCertificatesOnDeletion: true
///       skipGracePeriod: true
///   caPoolBinding:
///     type: gcp:certificateauthority:CaPoolIamBinding
///     name: ca_pool_binding
///     properties:
///       caPool: ${caPool.id}
///       role: roles/privateca.certificateRequester
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-sourcemanager.iam.gserviceaccount.com
///   default:
///     type: gcp:securesourcemanager:Instance
///     properties:
///       instanceId: my-instance
///       location: us-central1
///       privateConfig:
///         isPrivate: true
///         caPool: ${caPool.id}
///     options:
///       dependsOn:
///         - ${rootCa}
///         - ${wait120Seconds}
///   # ca pool IAM permissions can take time to propagate
///   wait120Seconds:
///     type: time:sleep
///     name: wait_120_seconds
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${caPoolBinding}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Secure Source Manager Instance Private Psc Backend
///
///
/// ```yaml
/// resources:
///   caPool:
///     type: gcp:certificateauthority:CaPool
///     name: ca_pool
///     properties:
///       name: ca-pool
///       location: us-central1
///       tier: ENTERPRISE
///       publishingOptions:
///         publishCaCert: true
///         publishCrl: true
///   rootCa:
///     type: gcp:certificateauthority:Authority
///     name: root_ca
///     properties:
///       pool: ${caPool.name}
///       certificateAuthorityId: root-ca
///       location: us-central1
///       config:
///         subjectConfig:
///           subject:
///             organization: google
///             commonName: my-certificate-authority
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       ignoreActiveCertificatesOnDeletion: true
///       skipGracePeriod: true
///   caPoolBinding:
///     type: gcp:certificateauthority:CaPoolIamBinding
///     name: ca_pool_binding
///     properties:
///       caPool: ${caPool.id}
///       role: roles/privateca.certificateRequester
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-sourcemanager.iam.gserviceaccount.com
///   # See https://cloud.google.com/secure-source-manager/docs/create-private-service-connect-instance#root-ca-api
///   default:
///     type: gcp:securesourcemanager:Instance
///     properties:
///       instanceId: my-instance
///       location: us-central1
///       privateConfig:
///         isPrivate: true
///         caPool: ${caPool.id}
///     options:
///       dependsOn:
///         - ${rootCa}
///         - ${wait120Seconds}
///   # ca pool IAM permissions can take time to propagate
///   wait120Seconds:
///     type: time:sleep
///     name: wait_120_seconds
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${caPoolBinding}
///   # Connect SSM private instance with L4 proxy ILB.
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-subnet
///       region: us-central1
///       network: ${network.id}
///       ipCidrRange: 10.0.1.0/24
///       privateIpGoogleAccess: true
///   pscNeg:
///     type: gcp:compute:RegionNetworkEndpointGroup
///     name: psc_neg
///     properties:
///       name: my-neg
///       region: us-central1
///       networkEndpointType: PRIVATE_SERVICE_CONNECT
///       pscTargetService: ${default.privateConfig.httpServiceAttachment}
///       network: ${network.id}
///       subnetwork: ${subnet.id}
///   backendService:
///     type: gcp:compute:RegionBackendService
///     name: backend_service
///     properties:
///       name: my-backend-service
///       region: us-central1
///       protocol: TCP
///       loadBalancingScheme: INTERNAL_MANAGED
///       backends:
///         - group: ${pscNeg.id}
///           balancingMode: UTILIZATION
///           capacityScaler: 1
///   proxySubnet:
///     type: gcp:compute:Subnetwork
///     name: proxy_subnet
///     properties:
///       name: my-proxy-subnet
///       region: us-central1
///       network: ${network.id}
///       ipCidrRange: 10.0.2.0/24
///       purpose: REGIONAL_MANAGED_PROXY
///       role: ACTIVE
///   targetProxy:
///     type: gcp:compute:RegionTargetTcpProxy
///     name: target_proxy
///     properties:
///       name: my-target-proxy
///       region: us-central1
///       backendService: ${backendService.id}
///   fwRuleTargetProxy:
///     type: gcp:compute:ForwardingRule
///     name: fw_rule_target_proxy
///     properties:
///       name: fw-rule-target-proxy
///       region: us-central1
///       loadBalancingScheme: INTERNAL_MANAGED
///       ipProtocol: TCP
///       portRange: '443'
///       target: ${targetProxy.id}
///       network: ${network.id}
///       subnetwork: ${subnet.id}
///       networkTier: PREMIUM
///     options:
///       dependsOn:
///         - ${proxySubnet}
///   privateZone:
///     type: gcp:dns:ManagedZone
///     name: private_zone
///     properties:
///       name: my-dns-zone
///       dnsName: p.sourcemanager.dev.
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${network.id}
///   ssmInstanceHtmlRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_html_record
///     properties:
///       name: ${default.hostConfigs[0].html}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleTargetProxy.ipAddress}
///   ssmInstanceApiRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_api_record
///     properties:
///       name: ${default.hostConfigs[0].api}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleTargetProxy.ipAddress}
///   ssmInstanceGitRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_git_record
///     properties:
///       name: ${default.hostConfigs[0].gitHttp}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleTargetProxy.ipAddress}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Secure Source Manager Instance Private Psc Endpoint
///
///
/// ```yaml
/// resources:
///   caPool:
///     type: gcp:certificateauthority:CaPool
///     name: ca_pool
///     properties:
///       name: ca-pool
///       location: us-central1
///       tier: ENTERPRISE
///       publishingOptions:
///         publishCaCert: true
///         publishCrl: true
///   rootCa:
///     type: gcp:certificateauthority:Authority
///     name: root_ca
///     properties:
///       pool: ${caPool.name}
///       certificateAuthorityId: root-ca
///       location: us-central1
///       config:
///         subjectConfig:
///           subject:
///             organization: google
///             commonName: my-certificate-authority
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       ignoreActiveCertificatesOnDeletion: true
///       skipGracePeriod: true
///   caPoolBinding:
///     type: gcp:certificateauthority:CaPoolIamBinding
///     name: ca_pool_binding
///     properties:
///       caPool: ${caPool.id}
///       role: roles/privateca.certificateRequester
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-sourcemanager.iam.gserviceaccount.com
///   # See https://cloud.google.com/secure-source-manager/docs/create-private-service-connect-instance#root-ca-api
///   default:
///     type: gcp:securesourcemanager:Instance
///     properties:
///       instanceId: my-instance
///       location: us-central1
///       privateConfig:
///         isPrivate: true
///         caPool: ${caPool.id}
///     options:
///       dependsOn:
///         - ${rootCa}
///         - ${wait120Seconds}
///   # ca pool IAM permissions can take time to propagate
///   wait120Seconds:
///     type: time:sleep
///     name: wait_120_seconds
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${caPoolBinding}
///   # Connect SSM private instance with endpoint.
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///       autoCreateSubnetworks: false
///   subnet:
///     type: gcp:compute:Subnetwork
///     properties:
///       name: my-subnet
///       region: us-central1
///       network: ${network.id}
///       ipCidrRange: 10.0.60.0/24
///       privateIpGoogleAccess: true
///   address:
///     type: gcp:compute:Address
///     properties:
///       name: my-address
///       region: us-central1
///       address: 10.0.60.100
///       addressType: INTERNAL
///       subnetwork: ${subnet.id}
///   fwRuleServiceAttachment:
///     type: gcp:compute:ForwardingRule
///     name: fw_rule_service_attachment
///     properties:
///       name: fw-rule-service-attachment
///       region: us-central1
///       loadBalancingScheme: ""
///       ipAddress: ${address.id}
///       network: ${network.id}
///       target: ${default.privateConfig.httpServiceAttachment}
///   privateZone:
///     type: gcp:dns:ManagedZone
///     name: private_zone
///     properties:
///       name: my-dns-zone
///       dnsName: p.sourcemanager.dev.
///       visibility: private
///       privateVisibilityConfig:
///         networks:
///           - networkUrl: ${network.id}
///   ssmInstanceHtmlRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_html_record
///     properties:
///       name: ${default.hostConfigs[0].html}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleServiceAttachment.ipAddress}
///   ssmInstanceApiRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_api_record
///     properties:
///       name: ${default.hostConfigs[0].api}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleServiceAttachment.ipAddress}
///   ssmInstanceGitRecord:
///     type: gcp:dns:RecordSet
///     name: ssm_instance_git_record
///     properties:
///       name: ${default.hostConfigs[0].gitHttp}.
///       type: A
///       ttl: 300
///       managedZone: ${privateZone.name}
///       rrdatas:
///         - ${fwRuleServiceAttachment.ipAddress}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Secure Source Manager Instance Workforce Identity Federation
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance::create(
///         "default",
///         InstanceArgs::builder()
///             .instance_id("my-instance")
///             .location("us-central1")
///             .workforce_identity_federation_config(
///                 InstanceWorkforceIdentityFederationConfig::builder()
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{location}}/{{instance_id}}`
///
/// * `{{location}}/{{instance_id}}`
///
/// * `{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/instance:Instance default {{project}}/{{location}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/instance:Instance default {{location}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/instance:Instance default {{instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The name for the Instance.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the Instance.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Private settings for private instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::securesourcemanager::InstancePrivateConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for Workforce Identity Federation to support third party identity provider.
        /// If unset, defaults to the Google OIDC IdP.
        /// Structure is documented below.
        #[builder(into, default)]
        pub workforce_identity_federation_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::securesourcemanager::InstanceWorkforceIdentityFederationConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Time the Instance was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of hostnames for this instance.
        /// Structure is documented below.
        pub host_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securesourcemanager::InstanceHostConfig>,
        >,
        /// The name for the Instance.
        ///
        ///
        /// - - -
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Customer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Labels as key value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the Instance.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the Instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Private settings for private instance.
        /// Structure is documented below.
        pub private_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::securesourcemanager::InstancePrivateConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current state of the Instance.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Provides information about the current instance state.
        pub state_note: pulumi_gestalt_rust::Output<String>,
        /// Time the Instance was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Configuration for Workforce Identity Federation to support third party identity provider.
        /// If unset, defaults to the Google OIDC IdP.
        /// Structure is documented below.
        pub workforce_identity_federation_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::securesourcemanager::InstanceWorkforceIdentityFederationConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let kms_key_binding = args.kms_key.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let private_config_binding = args.private_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let workforce_identity_federation_config_binding = args
            .workforce_identity_federation_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding.drop_type(),
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
                    name: "privateConfig".into(),
                    value: &private_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workforceIdentityFederationConfig".into(),
                    value: &workforce_identity_federation_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            host_configs: o.get_field("hostConfigs"),
            instance_id: o.get_field("instanceId"),
            kms_key: o.get_field("kmsKey"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_config: o.get_field("privateConfig"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
            state_note: o.get_field("stateNote"),
            update_time: o.get_field("updateTime"),
            workforce_identity_federation_config: o
                .get_field("workforceIdentityFederationConfig"),
        }
    }
}
