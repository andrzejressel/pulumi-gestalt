/// Resource for managing an AWS FinSpace Kx Volume.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:finspace:KxVolume
///     properties:
///       name: my-tf-kx-volume
///       environmentId: ${exampleAwsFinspaceKxEnvironment.id}
///       availabilityZones: use1-az2
///       azMode: SINGLE
///       type: NAS_1
///       nas1Configurations:
///         - size: 1200
///           type: SSD_250
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Volume using the `id` (environment ID and volume name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxVolume:KxVolume example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-volume
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_volume {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxVolumeArgs {
        /// The identifier of the AWS Availability Zone IDs.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub availability_zones: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The number of availability zones you want to assign per volume. Currently, Finspace only support SINGLE for volumes.
        /// * `SINGLE` - Assigns one availability zone per volume.
        #[builder(into)]
        pub az_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the volume.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique identifier for the kdb environment, whose clusters can attach to the volume.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique name for the volumr that you want to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the configuration for the Network attached storage (`NAS_1`) file system volume. This parameter is required when `volume_type` is `NAS_1`. See `nas1_configuration` Argument Reference below.
        #[builder(into, default)]
        pub nas1_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::finspace::KxVolumeNas1Configuration>>,
        >,
        /// A list of key-value pairs to label the volume. You can add up to 50 tags to a volume
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of file system volume. Currently, FinSpace only supports the `NAS_1` volume type. When you select the `NAS_1` volume type, you must also provide `nas1_configuration`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KxVolumeResult {
        /// Amazon Resource Name (ARN) identifier of the KX volume.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attached_clusters: pulumi_gestalt_rust::Output<
            Vec<super::super::types::finspace::KxVolumeAttachedCluster>,
        >,
        /// The identifier of the AWS Availability Zone IDs.
        ///
        /// The following arguments are optional:
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The number of availability zones you want to assign per volume. Currently, Finspace only support SINGLE for volumes.
        /// * `SINGLE` - Assigns one availability zone per volume.
        pub az_mode: pulumi_gestalt_rust::Output<String>,
        /// The timestamp at which the volume was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Description of the volume.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A unique identifier for the kdb environment, whose clusters can attach to the volume.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// Last timestamp at which the volume was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the volumr that you want to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the configuration for the Network attached storage (`NAS_1`) file system volume. This parameter is required when `volume_type` is `NAS_1`. See `nas1_configuration` Argument Reference below.
        pub nas1_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::finspace::KxVolumeNas1Configuration>>,
        >,
        /// The status of volume creation.
        /// * `CREATING` – The volume creation is in progress.
        /// * `CREATE_FAILED` – The volume creation has failed.
        /// * `ACTIVE` – The volume is active.
        /// * `UPDATING` – The volume is in the process of being updated.
        /// * `UPDATE_FAILED` – The update action failed.
        /// * `UPDATED` – The volume is successfully updated.
        /// * `DELETING` – The volume is in the process of being deleted.
        /// * `DELETE_FAILED` – The system failed to delete the volume.
        /// * `DELETED` – The volume is successfully deleted.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The error message when a failed state occurs.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// A list of key-value pairs to label the volume. You can add up to 50 tags to a volume
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of file system volume. Currently, FinSpace only supports the `NAS_1` volume type. When you select the `NAS_1` volume type, you must also provide `nas1_configuration`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KxVolumeArgs,
    ) -> KxVolumeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zones_binding = args.availability_zones.get_output(context);
        let az_mode_binding = args.az_mode.get_output(context);
        let description_binding = args.description.get_output(context);
        let environment_id_binding = args.environment_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let nas1_configurations_binding = args.nas1_configurations.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:finspace/kxVolume:KxVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azMode".into(),
                    value: &az_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nas1Configurations".into(),
                    value: &nas1_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KxVolumeResult {
            arn: o.get_field("arn"),
            attached_clusters: o.get_field("attachedClusters"),
            availability_zones: o.get_field("availabilityZones"),
            az_mode: o.get_field("azMode"),
            created_timestamp: o.get_field("createdTimestamp"),
            description: o.get_field("description"),
            environment_id: o.get_field("environmentId"),
            last_modified_timestamp: o.get_field("lastModifiedTimestamp"),
            name: o.get_field("name"),
            nas1_configurations: o.get_field("nas1Configurations"),
            status: o.get_field("status"),
            status_reason: o.get_field("statusReason"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
