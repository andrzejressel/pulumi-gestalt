#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_broker_nodes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerNodesArgs {
        /// ARN of the cluster the nodes belong to.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBrokerNodesResult {
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub node_info_lists: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::msk::GetBrokerNodesNodeInfoList>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBrokerNodesArgs,
    ) -> GetBrokerNodesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_arn_binding = args.cluster_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:msk/getBrokerNodes:getBrokerNodes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBrokerNodesResult {
            cluster_arn: o.get_field("clusterArn"),
            id: o.get_field("id"),
            node_info_lists: o.get_field("nodeInfoLists"),
        }
    }
}
