#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dataset_data_lake_gen_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetDataLakeGen2Args {
        /// The name of this Data Share Data Lake Gen2 Dataset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the Data Share where this Data Share Data Lake Gen2 Dataset should be created.
        #[builder(into)]
        pub share_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetDataLakeGen2Result {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The path of the file in the data lake file system to be shared with the receiver.
        pub file_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the data lake file system to be shared with the receiver.
        pub file_system_name: pulumi_gestalt_rust::Output<String>,
        /// The folder path in the data lake file system to be shared with the receiver.
        pub folder_path: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub share_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the storage account of the data lake file system to be shared with the receiver.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatasetDataLakeGen2Args,
    ) -> GetDatasetDataLakeGen2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let share_id_binding = args.share_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datashare/getDatasetDataLakeGen2:getDatasetDataLakeGen2"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatasetDataLakeGen2Result {
            display_name: o.get_field("displayName"),
            file_path: o.get_field("filePath"),
            file_system_name: o.get_field("fileSystemName"),
            folder_path: o.get_field("folderPath"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            share_id: o.get_field("shareId"),
            storage_account_id: o.get_field("storageAccountId"),
        }
    }
}
