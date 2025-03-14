#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBucketArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it is not provided then the data source will use the Compute API to find the project id that corresponds to the project number returned from the Storage API, and if no Compute API permissions are available or if the Compute API is disabled it defaults to the provider value. Supplying a value for `project` doesn't influence retrieving data about the bucket but it can be used to prevent use of the Compute API. If you do provide a `project` value ensure that it is the correct value for that bucket; the data source will not check that the project id and project number match.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBucketResult {
        pub autoclasses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketAutoclass>,
        >,
        pub cors: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketCor>,
        >,
        pub custom_placement_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketCustomPlacementConfig>,
        >,
        pub default_event_based_hold: pulumi_gestalt_rust::Output<bool>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_object_retention: pulumi_gestalt_rust::Output<bool>,
        pub encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketEncryption>,
        >,
        pub force_destroy: pulumi_gestalt_rust::Output<bool>,
        pub hierarchical_namespaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketHierarchicalNamespace>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub lifecycle_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketLifecycleRule>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub loggings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketLogging>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub project_number: pulumi_gestalt_rust::Output<i32>,
        pub public_access_prevention: pulumi_gestalt_rust::Output<String>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub requester_pays: pulumi_gestalt_rust::Output<bool>,
        pub retention_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketRetentionPolicy>,
        >,
        pub rpo: pulumi_gestalt_rust::Output<String>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub soft_delete_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketSoftDeletePolicy>,
        >,
        pub storage_class: pulumi_gestalt_rust::Output<String>,
        pub uniform_bucket_level_access: pulumi_gestalt_rust::Output<bool>,
        pub url: pulumi_gestalt_rust::Output<String>,
        pub versionings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketVersioning>,
        >,
        pub websites: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::storage::GetBucketWebsite>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBucketArgs,
    ) -> GetBucketResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:storage/getBucket:getBucket".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetBucketResult {
            autoclasses: o.get_field("autoclasses"),
            cors: o.get_field("cors"),
            custom_placement_configs: o.get_field("customPlacementConfigs"),
            default_event_based_hold: o.get_field("defaultEventBasedHold"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_object_retention: o.get_field("enableObjectRetention"),
            encryptions: o.get_field("encryptions"),
            force_destroy: o.get_field("forceDestroy"),
            hierarchical_namespaces: o.get_field("hierarchicalNamespaces"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            lifecycle_rules: o.get_field("lifecycleRules"),
            location: o.get_field("location"),
            loggings: o.get_field("loggings"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            project_number: o.get_field("projectNumber"),
            public_access_prevention: o.get_field("publicAccessPrevention"),
            pulumi_labels: o.get_field("pulumiLabels"),
            requester_pays: o.get_field("requesterPays"),
            retention_policies: o.get_field("retentionPolicies"),
            rpo: o.get_field("rpo"),
            self_link: o.get_field("selfLink"),
            soft_delete_policies: o.get_field("softDeletePolicies"),
            storage_class: o.get_field("storageClass"),
            uniform_bucket_level_access: o.get_field("uniformBucketLevelAccess"),
            url: o.get_field("url"),
            versionings: o.get_field("versionings"),
            websites: o.get_field("websites"),
        }
    }
}
