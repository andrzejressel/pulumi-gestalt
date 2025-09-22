#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SelectionCondition {
    #[builder(into)]
    #[serde(rename = "stringEquals")]
    pub r#string_equals: Option<Vec<super::super::types::backup::SelectionConditionStringEqual>>,
    #[builder(into)]
    #[serde(rename = "stringLikes")]
    pub r#string_likes: Option<Vec<super::super::types::backup::SelectionConditionStringLike>>,
    #[builder(into)]
    #[serde(rename = "stringNotEquals")]
    pub r#string_not_equals: Option<Vec<super::super::types::backup::SelectionConditionStringNotEqual>>,
    #[builder(into)]
    #[serde(rename = "stringNotLikes")]
    pub r#string_not_likes: Option<Vec<super::super::types::backup::SelectionConditionStringNotLike>>,
}
