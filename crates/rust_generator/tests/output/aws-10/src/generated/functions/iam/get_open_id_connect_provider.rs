#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_open_id_connect_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOpenIdConnectProviderArgs {
        /// ARN of the OpenID Connect provider.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of resource tags for the IAM OIDC provider.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// URL of the OpenID Connect provider.
        #[builder(into, default)]
        pub url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOpenIdConnectProviderResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of client IDs (also known as audiences). When a mobile or web app registers with an OpenID Connect provider, they establish a value that identifies the application. (This is the value that's sent as the client_id parameter on OAuth requests.)
        pub client_id_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags for the IAM OIDC provider.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// List of server certificate thumbprints for the OpenID Connect (OIDC) identity provider's server certificate(s).
        pub thumbprint_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOpenIdConnectProviderArgs,
    ) -> GetOpenIdConnectProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getOpenIdConnectProvider:getOpenIdConnectProvider".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOpenIdConnectProviderResult {
            arn: o.get_field("arn"),
            client_id_lists: o.get_field("clientIdLists"),
            id: o.get_field("id"),
            tags: o.get_field("tags"),
            thumbprint_lists: o.get_field("thumbprintLists"),
            url: o.get_field("url"),
        }
    }
}
