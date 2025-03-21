#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestoreTestingSelectionProtectedResourceConditionsStringNotEqual {
    /// The Tag name, must start with one of the following prefixes: [aws:ResourceTag/] with a Minimum length of 1. Maximum length of 128, and can contain characters that are letters, white space, and numbers that can be represented in UTF-8 and the following characters: `+ - = . _ : /`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the Tag. Maximum length of 256.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
