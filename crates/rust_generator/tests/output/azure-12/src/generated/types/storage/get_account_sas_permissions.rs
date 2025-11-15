#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountSasPermissions {
    /// Should Add permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "add")]
    pub r#add: bool,
    /// Should Create permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "create")]
    pub r#create: bool,
    /// Should Delete permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: bool,
    /// Should Filter by Index Tags permissions be enabled for this SAS?
    /// 
    /// Refer to the [SAS creation reference from Azure](https://docs.microsoft.com/rest/api/storageservices/constructing-an-account-sas)
    /// for additional details on the fields above.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: bool,
    /// Should List permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "list")]
    pub r#list: bool,
    /// Should Process permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "process")]
    pub r#process: bool,
    /// Should Read permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: bool,
    /// Should Get / Set Index Tags permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: bool,
    /// Should Update permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "update")]
    pub r#update: bool,
    /// Should Write permissions be enabled for this SAS?
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: bool,
}
