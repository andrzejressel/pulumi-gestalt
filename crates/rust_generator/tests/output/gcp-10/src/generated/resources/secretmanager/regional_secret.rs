/// A Regional Secret is a logical secret whose value and versions can be created and accessed within a region only.
///
///
/// To get more information about RegionalSecret, see:
///
/// * [API documentation](https://cloud.google.com/secret-manager/docs/reference/rest/v1/projects.locations.secrets)
///
/// ## Example Usage
///
/// ### Regional Secret Config Basic
///
///
/// ```yaml
/// resources:
///   regional-secret-basic:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       labels:
///         label: my-label
///       annotations:
///         key1: value1
///         key2: value2
///         key3: value3
/// ```
/// ### Regional Secret With Cmek
///
///
/// ```yaml
/// resources:
///   kms-secret-binding:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-secretmanager.iam.gserviceaccount.com
///   regional-secret-with-cmek:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       customerManagedEncryption:
///         kmsKeyName: kms-key
///     options:
///       dependsOn:
///         - ${["kms-secret-binding"]}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Regional Secret With Rotation
///
///
/// ```yaml
/// resources:
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: tf-topic
///   secretsManagerAccess:
///     type: gcp:pubsub:TopicIAMMember
///     name: secrets_manager_access
///     properties:
///       topic: ${topic.name}
///       role: roles/pubsub.publisher
///       member: serviceAccount:service-${project.number}@gcp-sa-secretmanager.iam.gserviceaccount.com
///   regional-secret-with-rotation:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       topics:
///         - name: ${topic.id}
///       rotation:
///         rotationPeriod: 3600s
///         nextRotationTime: 2045-11-30T00:00:00Z
///     options:
///       dependsOn:
///         - ${secretsManagerAccess}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Regional Secret With Ttl
///
///
/// ```yaml
/// resources:
///   regional-secret-with-ttl:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       labels:
///         label: my-label
///       annotations:
///         key1: value1
///         key2: value2
///         key3: value3
///       ttl: 36000s
/// ```
/// ### Regional Secret With Expire Time
///
///
/// ```yaml
/// resources:
///   regional-secret-with-expire-time:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       labels:
///         label: my-label
///       annotations:
///         key1: value1
///         key2: value2
///         key3: value3
///       expireTime: 2055-11-30T00:00:00Z
/// ```
/// ### Regional Secret With Version Destroy Ttl
///
///
/// ```yaml
/// resources:
///   regional-secret-with-version-destroy-ttl:
///     type: gcp:secretmanager:RegionalSecret
///     properties:
///       secretId: tf-reg-secret
///       location: us-central1
///       labels:
///         label: my-label
///       annotations:
///         key1: value1
///         key2: value2
///         key3: value3
///       versionDestroyTtl: 86400s
/// ```
///
/// ## Import
///
/// RegionalSecret can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}`
///
/// * `{{project}}/{{location}}/{{secret_id}}`
///
/// * `{{location}}/{{secret_id}}`
///
/// When using the `pulumi import` command, RegionalSecret can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecret:RegionalSecret default projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecret:RegionalSecret default {{project}}/{{location}}/{{secret_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:secretmanager/regionalSecret:RegionalSecret default {{location}}/{{secret_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regional_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalSecretArgs {
        /// Custom metadata about the regional secret.
        /// Annotations are distinct from various forms of labels. Annotations exist to allow
        /// client tools to store their own state information without requiring a database.
        /// Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and
        /// may have dashes (-), underscores (_), dots (.), and alphanumerics in between these
        /// symbols.
        /// The total size of annotation keys and values must be less than 16KiB.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The customer-managed encryption configuration of the regional secret.
        /// Structure is documented below.
        #[builder(into, default)]
        pub customer_managed_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::secretmanager::RegionalSecretCustomerManagedEncryption,
            >,
        >,
        /// Timestamp in UTC when the regional secret is scheduled to expire. This is always provided on
        /// output, regardless of what was sent on input. A timestamp in RFC3339 UTC "Zulu" format, with
        /// nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z". Only one of `expire_time` or `ttl` can be provided.
        #[builder(into, default)]
        pub expire_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The labels assigned to this regional secret.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,
        /// and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,
        /// and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be assigned to a given resource.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the regional secret. eg us-central1
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The rotation time and period for a regional secret. At `next_rotation_time`, Secret Manager
        /// will send a Pub/Sub notification to the topics configured on the Secret. `topics` must be
        /// set to configure rotation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub rotation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::secretmanager::RegionalSecretRotation>,
        >,
        /// This must be unique within the project.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of up to 10 Pub/Sub topics to which messages are published when control plane
        /// operations are called on the regional secret or its versions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub topics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::secretmanager::RegionalSecretTopic>>,
        >,
        /// The TTL for the regional secret. A duration in seconds with up to nine fractional digits,
        /// terminated by 's'. Example: "3.5s". Only one of `ttl` or `expire_time` can be provided.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Mapping from version alias to version name.
        /// A version alias is a string with a maximum length of 63 characters and can contain
        /// uppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')
        /// characters. An alias string must start with a letter and cannot be the string
        /// 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        #[builder(into, default)]
        pub version_aliases: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Secret Version TTL after destruction request.
        /// This is a part of the delayed delete feature on Secret Version.
        /// For secret with versionDestroyTtl>0, version destruction doesn't happen immediately
        /// on calling destroy instead the version goes to a disabled state and
        /// the actual destruction happens after this TTL expires. It must be atleast 24h.
        #[builder(into, default)]
        pub version_destroy_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionalSecretResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Custom metadata about the regional secret.
        /// Annotations are distinct from various forms of labels. Annotations exist to allow
        /// client tools to store their own state information without requiring a database.
        /// Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of
        /// maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and
        /// may have dashes (-), underscores (_), dots (.), and alphanumerics in between these
        /// symbols.
        /// The total size of annotation keys and values must be less than 16KiB.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time at which the regional secret was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The customer-managed encryption configuration of the regional secret.
        /// Structure is documented below.
        pub customer_managed_encryption: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::secretmanager::RegionalSecretCustomerManagedEncryption,
            >,
        >,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Timestamp in UTC when the regional secret is scheduled to expire. This is always provided on
        /// output, regardless of what was sent on input. A timestamp in RFC3339 UTC "Zulu" format, with
        /// nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z". Only one of `expire_time` or `ttl` can be provided.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The labels assigned to this regional secret.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,
        /// and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,
        /// and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be assigned to a given resource.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the regional secret. eg us-central1
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the regional secret. Format:
        /// `projects/{{project}}/locations/{{location}}/secrets/{{secret_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The rotation time and period for a regional secret. At `next_rotation_time`, Secret Manager
        /// will send a Pub/Sub notification to the topics configured on the Secret. `topics` must be
        /// set to configure rotation.
        /// Structure is documented below.
        pub rotation: pulumi_gestalt_rust::Output<
            Option<super::super::types::secretmanager::RegionalSecretRotation>,
        >,
        /// This must be unique within the project.
        ///
        ///
        /// - - -
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        /// A list of up to 10 Pub/Sub topics to which messages are published when control plane
        /// operations are called on the regional secret or its versions.
        /// Structure is documented below.
        pub topics: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::secretmanager::RegionalSecretTopic>>,
        >,
        /// The TTL for the regional secret. A duration in seconds with up to nine fractional digits,
        /// terminated by 's'. Example: "3.5s". Only one of `ttl` or `expire_time` can be provided.
        pub ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// Mapping from version alias to version name.
        /// A version alias is a string with a maximum length of 63 characters and can contain
        /// uppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')
        /// characters. An alias string must start with a letter and cannot be the string
        /// 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.
        /// An object containing a list of "key": value pairs. Example:
        /// { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        pub version_aliases: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Secret Version TTL after destruction request.
        /// This is a part of the delayed delete feature on Secret Version.
        /// For secret with versionDestroyTtl>0, version destruction doesn't happen immediately
        /// on calling destroy instead the version goes to a disabled state and
        /// the actual destruction happens after this TTL expires. It must be atleast 24h.
        pub version_destroy_ttl: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionalSecretArgs,
    ) -> RegionalSecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let customer_managed_encryption_binding = args
            .customer_managed_encryption
            .get_output(context);
        let expire_time_binding = args.expire_time.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let rotation_binding = args.rotation.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let topics_binding = args.topics.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let version_aliases_binding = args.version_aliases.get_output(context);
        let version_destroy_ttl_binding = args.version_destroy_ttl.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:secretmanager/regionalSecret:RegionalSecret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedEncryption".into(),
                    value: &customer_managed_encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expireTime".into(),
                    value: &expire_time_binding.drop_type(),
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
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rotation".into(),
                    value: &rotation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topics".into(),
                    value: &topics_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionAliases".into(),
                    value: &version_aliases_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionDestroyTtl".into(),
                    value: &version_destroy_ttl_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionalSecretResult {
            id: o.get_field("id"),
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            customer_managed_encryption: o.get_field("customerManagedEncryption"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rotation: o.get_field("rotation"),
            secret_id: o.get_field("secretId"),
            topics: o.get_field("topics"),
            ttl: o.get_field("ttl"),
            version_aliases: o.get_field("versionAliases"),
            version_destroy_ttl: o.get_field("versionDestroyTtl"),
        }
    }
}
