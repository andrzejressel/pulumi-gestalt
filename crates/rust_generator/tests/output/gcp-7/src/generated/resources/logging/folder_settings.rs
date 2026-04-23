/// Default resource settings control whether CMEK is required for new log buckets. These settings also determine the storage location for the _Default and _Required log buckets, and whether the _Default sink is enabled or disabled.
///
///
/// To get more information about FolderSettings, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/TopLevel/getSettings)
/// * How-to Guides
///     * [Configure default settings for organizations and folders](https://cloud.google.com/logging/docs/default-settings)
///
/// ## Example Usage
///
/// ### Logging Folder Settings All
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:logging:FolderSettings
///     properties:
///       disableDefaultSink: true
///       folder: ${myFolder.folderId}
///       kmsKeyName: kms-key
///       storageLocation: us-central1
///     options:
///       dependsOn:
///         - ${iam}
///   myFolder:
///     type: gcp:organizations:Folder
///     name: my_folder
///     properties:
///       displayName: folder-name
///       parent: organizations/123456789
///       deletionProtection: false
///   iam:
///     type: gcp:kms:CryptoKeyIAMMember
///     properties:
///       cryptoKeyId: kms-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:${settings.kmsServiceAccountId}
/// variables:
///   settings:
///     fn::invoke:
///       function: gcp:logging:getFolderSettings
///       arguments:
///         folder: ${myFolder.folderId}
/// ```
///
/// ## Import
///
/// FolderSettings can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/settings`
///
/// * `{{folder}}`
///
/// When using the `pulumi import` command, FolderSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/folderSettings:FolderSettings default folders/{{folder}}/settings
/// ```
///
/// ```sh
/// $ pulumi import gcp:logging/folderSettings:FolderSettings default {{folder}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod folder_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderSettingsArgs {
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        #[builder(into, default)]
        pub disable_default_sink: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The folder for which to retrieve settings.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::Input<String>,
        /// The resource name for the configured Cloud KMS key.
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        #[builder(into, default)]
        pub storage_location: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FolderSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// If set to true, the _Default sink in newly created projects and folders will created in a disabled state. This can be used to automatically disable log storage if there is already an aggregated sink configured in the hierarchy. The _Default sink can be re-enabled manually if needed.
        pub disable_default_sink: pulumi_gestalt_rust::Output<bool>,
        /// The folder for which to retrieve settings.
        ///
        ///
        /// - - -
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the configured Cloud KMS key.
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// The service account that will be used by the Log Router to access your Cloud KMS key.
        pub kms_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The service account for the given container. Sinks use this service account as their writerIdentity if no custom service account is provided.
        pub logging_service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the settings.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The storage location that Cloud Logging will use to create new resources when a location is needed but not explicitly provided.
        pub storage_location: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderSettingsArgs,
    ) -> FolderSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FolderSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FolderSettingsResult {
        let disable_default_sink_binding = args.disable_default_sink.get_output(ctx);
        let folder_binding = args.folder.get_output(ctx);
        let kms_key_name_binding = args.kms_key_name.get_output(ctx);
        let storage_location_binding = args.storage_location.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/folderSettings:FolderSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableDefaultSink".into(),
                    value: &disable_default_sink_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageLocation".into(),
                    value: &storage_location_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FolderSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            disable_default_sink: o.get_field("disableDefaultSink"),
            folder: o.get_field("folder"),
            kms_key_name: o.get_field("kmsKeyName"),
            kms_service_account_id: o.get_field("kmsServiceAccountId"),
            logging_service_account_id: o.get_field("loggingServiceAccountId"),
            name: o.get_field("name"),
            storage_location: o.get_field("storageLocation"),
        }
    }
}
