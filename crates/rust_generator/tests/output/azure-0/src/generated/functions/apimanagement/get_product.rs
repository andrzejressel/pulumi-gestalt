#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_product {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// The Name of the API Management Service in which this Product exists.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for the API Management Product.
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// Do subscribers need to be approved prior to being able to use the Product?
        pub approval_required: pulumi_gestalt_rust::Output<bool>,
        /// The description of this Product, which may include HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The Display Name for this API Management Product.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Is this Product Published?
        pub published: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is a Subscription required to access API's included in this Product?
        pub subscription_required: pulumi_gestalt_rust::Output<bool>,
        /// The number of subscriptions a user can have to this Product at the same time.
        pub subscriptions_limit: pulumi_gestalt_rust::Output<i32>,
        /// Any Terms and Conditions for this Product, which must be accepted by Developers before they can begin the Subscription process.
        pub terms: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProductArgs,
    ) -> GetProductResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let product_id_binding = args.product_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:apimanagement/getProduct:getProduct".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productId".into(),
                    value: &product_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProductResult {
            api_management_name: o.get_field("apiManagementName"),
            approval_required: o.get_field("approvalRequired"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            product_id: o.get_field("productId"),
            published: o.get_field("published"),
            resource_group_name: o.get_field("resourceGroupName"),
            subscription_required: o.get_field("subscriptionRequired"),
            subscriptions_limit: o.get_field("subscriptionsLimit"),
            terms: o.get_field("terms"),
        }
    }
}
