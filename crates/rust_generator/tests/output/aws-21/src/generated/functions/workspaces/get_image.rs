#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// ID of the image.
        #[builder(into)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The description of the image.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the image.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub operating_system_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the image is running on dedicated hardware. When Bring Your Own License (BYOL) is enabled, this value is set to DEDICATED. For more information, see [Bring Your Own Windows Desktop Images](https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.html).
        pub required_tenancy: pulumi_gestalt_rust::Output<String>,
        /// The status of the image.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImageArgs,
    ) -> GetImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let image_id_binding = args.image_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:workspaces/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImageResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            image_id: o.get_field("imageId"),
            name: o.get_field("name"),
            operating_system_type: o.get_field("operatingSystemType"),
            required_tenancy: o.get_field("requiredTenancy"),
            state: o.get_field("state"),
        }
    }
}
