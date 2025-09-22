#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Typ {
    #[builder(into)]
    #[serde(rename = "mod1")]
    pub r#mod_1: Box<Option<super::types::mod1::Typ>>,
    #[builder(into)]
    #[serde(rename = "mod2")]
    pub r#mod_2: Box<Option<super::types::mod2::Typ>>,
    #[builder(into)]
    #[serde(rename = "val")]
    pub r#val: Option<String>,
}
