/// Manages a Container App Environment Certificate.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: acctest-01
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///       retentionInDays: 30
///   exampleEnvironment:
///     type: azure:containerapp:Environment
///     name: example
///     properties:
///       name: myEnvironment
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       logAnalyticsWorkspaceId: ${exampleAnalyticsWorkspace.id}
///   exampleEnvironmentCertificate:
///     type: azure:containerapp:EnvironmentCertificate
///     name: example
///     properties:
///       name: myfriendlyname
///       containerAppEnvironmentId: ${exampleEnvironment.id}
///       certificateBlob:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: path/to/certificate_file.pfx
///           return: result
///       certificatePassword: $3cretSqu1rreL
/// ```
///
/// ## Import
///
/// A Container App Environment Certificate can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/environmentCertificate:EnvironmentCertificate example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/managedEnvironments/myenv/certificates/mycertificate"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentCertificateArgs {
        /// The Certificate Private Key as a base64 encoded PFX or PEM. Changing this forces a new resource to be created.
        #[builder(into)]
        pub certificate_blob_base64: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password for the Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub certificate_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Container App Managed Environment ID to configure this Certificate on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Container Apps Environment Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentCertificateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Certificate Private Key as a base64 encoded PFX or PEM. Changing this forces a new resource to be created.
        pub certificate_blob_base64: pulumi_gestalt_rust::Output<String>,
        /// The password for the Certificate. Changing this forces a new resource to be created.
        pub certificate_password: pulumi_gestalt_rust::Output<String>,
        /// The Container App Managed Environment ID to configure this Certificate on. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The expiration date for the Certificate.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The date of issue for the Certificate.
        pub issue_date: pulumi_gestalt_rust::Output<String>,
        /// The Certificate Issuer.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// The name of the Container Apps Environment Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Subject Name for the Certificate.
        pub subject_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Thumbprint of the Certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentCertificateArgs,
    ) -> EnvironmentCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_blob_base64_binding = args
            .certificate_blob_base64
            .get_output(context);
        let certificate_password_binding = args.certificate_password.get_output(context);
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerapp/environmentCertificate:EnvironmentCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateBlobBase64".into(),
                    value: &certificate_blob_base64_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificatePassword".into(),
                    value: &certificate_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentCertificateResult {
            id: o.get_field("id"),
            certificate_blob_base64: o.get_field("certificateBlobBase64"),
            certificate_password: o.get_field("certificatePassword"),
            container_app_environment_id: o.get_field("containerAppEnvironmentId"),
            expiration_date: o.get_field("expirationDate"),
            issue_date: o.get_field("issueDate"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            subject_name: o.get_field("subjectName"),
            tags: o.get_field("tags"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
