/// Three different resources help you manage your IAM policy for BigQuery dataset. Each of these resources serves a different use case:
///
/// * `gcp.bigquery.DatasetIamPolicy`: Authoritative. Sets the IAM policy for the dataset and replaces any existing policy already attached.
/// * `gcp.bigquery.DatasetIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the dataset are preserved.
/// * `gcp.bigquery.DatasetIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the dataset are preserved.
///
/// These resources are intended to convert the permissions system for BigQuery datasets to the standard IAM interface. For advanced usages, including [creating authorized views](https://cloud.google.com/bigquery/docs/share-access-views), please use either `gcp.bigquery.DatasetAccess` or the `access` field on `gcp.bigquery.Dataset`.
///
/// > **Note:** These resources **cannot** be used with `gcp.bigquery.DatasetAccess` resources or the `access` field on `gcp.bigquery.Dataset` or they will fight over what the policy should be.
///
/// > **Note:** Using any of these resources will remove any authorized view permissions from the dataset. To assign and preserve authorized view permissions use the `gcp.bigquery.DatasetAccess` instead.
///
/// > **Note:** Legacy BigQuery roles `OWNER` `WRITER` and `READER` **cannot** be used with any of these IAM resources. Instead use the full role form of: `roles/bigquery.dataOwner` `roles/bigquery.dataEditor` and `roles/bigquery.dataViewer`.
///
/// > **Note:** `gcp.bigquery.DatasetIamPolicy` **cannot** be used in conjunction with `gcp.bigquery.DatasetIamBinding` and `gcp.bigquery.DatasetIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.bigquery.DatasetIamBinding` resources **can be** used in conjunction with `gcp.bigquery.DatasetIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.bigquery.DatasetIamPolicy
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:bigquery:DatasetIamPolicy
///     properties:
///       datasetId: ${datasetDataset.datasetId}
///       policyData: ${owner.policyData}
///   datasetDataset:
///     type: gcp:bigquery:Dataset
///     name: dataset
///     properties:
///       datasetId: example_dataset
/// variables:
///   owner:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigquery.dataOwner
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigquery.DatasetIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
///     let reader = dataset_iam_binding::create(
///         "reader",
///         DatasetIamBindingArgs::builder()
///             .dataset_id("${dataset.datasetId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigquery.dataViewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigquery.DatasetIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
///     let editor = dataset_iam_member::create(
///         "editor",
///         DatasetIamMemberArgs::builder()
///             .dataset_id("${dataset.datasetId}")
///             .member("user:jane@example.com")
///             .role("roles/bigquery.dataEditor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigquery.DatasetIamPolicy
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:bigquery:DatasetIamPolicy
///     properties:
///       datasetId: ${datasetDataset.datasetId}
///       policyData: ${owner.policyData}
///   datasetDataset:
///     type: gcp:bigquery:Dataset
///     name: dataset
///     properties:
///       datasetId: example_dataset
/// variables:
///   owner:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigquery.dataOwner
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigquery.DatasetIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
///     let reader = dataset_iam_binding::create(
///         "reader",
///         DatasetIamBindingArgs::builder()
///             .dataset_id("${dataset.datasetId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigquery.dataViewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigquery.DatasetIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder().dataset_id("example_dataset").build_struct(),
///     );
///     let editor = dataset_iam_member::create(
///         "editor",
///         DatasetIamMemberArgs::builder()
///             .dataset_id("${dataset.datasetId}")
///             .member("user:jane@example.com")
///             .role("roles/bigquery.dataEditor")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the identifier of the BigQuery Dataset resource. For example:
///
/// * `projects/{{project_id}}/datasets/{{dataset_id}}`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = projects/{{project_id}}/datasets/{{dataset_id}}
///
///   to = google_bigquery_dataset_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:bigquery/datasetIamPolicy:DatasetIamPolicy default projects/{{project_id}}/datasets/{{dataset_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetIamPolicyArgs {
        /// The dataset ID.
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatasetIamPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The dataset ID.
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the dataset's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetIamPolicyArgs,
    ) -> DatasetIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dataset_id_binding = args.dataset_id.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/datasetIamPolicy:DatasetIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetIamPolicyResult {
            id: o.get_field("id"),
            dataset_id: o.get_field("datasetId"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
