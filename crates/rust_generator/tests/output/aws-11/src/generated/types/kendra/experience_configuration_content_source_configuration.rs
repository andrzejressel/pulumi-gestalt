#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperienceConfigurationContentSourceConfiguration {
    /// The identifiers of the data sources you want to use for your Amazon Kendra experience. Maximum number of 100 items.
    #[builder(into)]
    #[serde(rename = "dataSourceIds")]
    pub r#data_source_ids: Option<Vec<String>>,
    /// Whether to use documents you indexed directly using the `BatchPutDocument API`. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "directPutContent")]
    pub r#direct_put_content: Option<bool>,
    /// The identifier of the FAQs that you want to use for your Amazon Kendra experience. Maximum number of 100 items.
    #[builder(into)]
    #[serde(rename = "faqIds")]
    pub r#faq_ids: Option<Vec<String>>,
}
