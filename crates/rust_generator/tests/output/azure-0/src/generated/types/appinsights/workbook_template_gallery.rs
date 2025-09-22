#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkbookTemplateGallery {
    /// Category for the gallery.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// Name of the workbook template in the gallery.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Order of the template within the gallery. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// Azure resource type supported by the gallery. Defaults to `Azure Monitor`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
    /// Type of workbook supported by the workbook template. Defaults to `workbook`.
    /// 
    /// > **Note:** See [documentation](https://docs.microsoft.com/en-us/azure/azure-monitor/visualize/workbooks-automate#galleries) for more information of `resource_type` and `type`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
