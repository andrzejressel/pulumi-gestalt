#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInsightsAnalysisReturnPathComponentAdditionalDetail {
    #[builder(into)]
    #[serde(rename = "additionalDetailType")]
    pub r#additional_detail_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAdditionalDetailComponent>>,
}
