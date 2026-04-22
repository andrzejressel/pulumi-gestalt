/// Represents an Instance Settings resource. Instance settings are centralized configuration parameters that allow users to configure the default values for specific VM parameters that are normally set using GCE instance API methods.
///
///
/// To get more information about InstanceSettings, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/instanceSettings)
/// * How-to Guides
///     * [Update Instance Settings](https://cloud.google.com/compute/docs/metadata/setting-custom-metadata#set-custom-project-zonal-metadata)
///
/// ## Example Usage
///
/// ### Instance Settings Basic
///
///
/// ```yaml
/// resources:
///   gceInstanceSettings:
///     type: gcp:compute:InstanceSettings
///     name: gce_instance_settings
///     properties:
///       zone: us-east7-b
///       metadata:
///         items:
///           foo: baz
/// ```
///
/// ## Import
///
/// InstanceSettings can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instanceSettings`
///
/// * `{{project}}/{{zone}}`
///
/// * `{{zone}}`
///
/// When using the `pulumi import` command, InstanceSettings can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default projects/{{project}}/zones/{{zone}}/instanceSettings
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default {{project}}/{{zone}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instanceSettings:InstanceSettings default {{zone}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod instance_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceSettingsArgs {
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::Input<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The metadata key/value pairs assigned to all the instances in the corresponding scope.
        /// Structure is documented below.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceSettingsMetadata>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A reference to the zone where the machine resides.
        ///
        ///
        /// - - -
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceSettingsArgs,
    ) -> InstanceSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> InstanceSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> InstanceSettingsResult {
        let metadata_binding = args.metadata.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let zone_binding = args.zone.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceSettings:InstanceSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        InstanceSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            fingerprint: o.get_field("fingerprint"),
            metadata: o.get_field("metadata"),
            project: o.get_field("project"),
            zone: o.get_field("zone"),
        }
    }
}
