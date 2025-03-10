#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_canonical_user_id {
    #[allow(dead_code)]
    pub struct GetCanonicalUserIdResult {
        /// Human-friendly name linked to the canonical user ID. The bucket owner's display name. **NOTE:** [This value](https://docs.aws.amazon.com/AmazonS3/latest/API/RESTServiceGET.html) is only included in the response in the US East (N. Virginia), US West (N. California), US West (Oregon), Asia Pacific (Singapore), Asia Pacific (Sydney), Asia Pacific (Tokyo), EU (Ireland), and South America (São Paulo) regions.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetCanonicalUserIdResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:s3/getCanonicalUserId:getCanonicalUserId".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetCanonicalUserIdResult {
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
        }
    }
}
