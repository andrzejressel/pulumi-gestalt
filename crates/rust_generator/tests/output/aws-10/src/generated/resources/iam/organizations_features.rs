/// Manages centralized root access features across AWS member accounts managed using AWS Organizations. More information about managing root access in IAM can be found in the [Centralize root access for member accounts](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_root-enable-root-access.html).
///
/// > **NOTE:** The AWS account utilizing this resource must be an Organizations management account. Also, you must enable trusted access for AWS Identity and Access Management in AWS Organizations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(vec!["iam.amazonaws.com",])
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleOrganizationsFeatures = organizations_features::create(
///         "exampleOrganizationsFeatures",
///         OrganizationsFeaturesArgs::builder()
///             .enabled_features(vec!["RootCredentialsManagement", "RootSessions",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import root access features using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/organizationsFeatures:OrganizationsFeatures example o-1234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organizations_features {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationsFeaturesArgs {
        /// List of IAM features to enable. Valid values are `RootCredentialsManagement` and `RootSessions`.
        #[builder(into)]
        pub enabled_features: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationsFeaturesResult {
        /// List of IAM features to enable. Valid values are `RootCredentialsManagement` and `RootSessions`.
        pub enabled_features: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationsFeaturesArgs,
    ) -> OrganizationsFeaturesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_features_binding = args.enabled_features.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/organizationsFeatures:OrganizationsFeatures".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabledFeatures".into(),
                    value: &enabled_features_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationsFeaturesResult {
            enabled_features: o.get_field("enabledFeatures"),
        }
    }
}
