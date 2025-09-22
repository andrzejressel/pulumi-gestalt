#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainIndexField {
    /// The analysis scheme you want to use for a `text` field. The analysis scheme specifies the language-specific text processing options that are used during indexing.
    #[builder(into)]
    #[serde(rename = "analysisScheme")]
    pub r#analysis_scheme: Option<String>,
    /// The default value for the field. This value is used when no value is specified for the field in the document data.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// You can get facet information by enabling this.
    #[builder(into)]
    #[serde(rename = "facet")]
    pub r#facet: Option<bool>,
    /// You can highlight information.
    #[builder(into)]
    #[serde(rename = "highlight")]
    pub r#highlight: Option<bool>,
    /// A unique name for the field. Field names must begin with a letter and be at least 1 and no more than 64 characters long. The allowed characters are: `a`-`z` (lower-case letters), `0`-`9`, and `_` (underscore). The name `score` is reserved and cannot be used as a field name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// You can enable returning the value of all searchable fields.
    #[builder(into)]
    #[serde(rename = "return")]
    pub r#return_: Option<bool>,
    /// You can set whether this index should be searchable or not.
    #[builder(into)]
    #[serde(rename = "search")]
    pub r#search: Option<bool>,
    /// You can enable the property to be sortable.
    #[builder(into)]
    #[serde(rename = "sort")]
    pub r#sort: Option<bool>,
    /// A comma-separated list of source fields to map to the field. Specifying a source field copies data from one field to another, enabling you to use the same source data in different ways by configuring different options for the fields.
    #[builder(into)]
    #[serde(rename = "sourceFields")]
    pub r#source_fields: Option<String>,
    /// The field type. Valid values: `date`, `date-array`, `double`, `double-array`, `int`, `int-array`, `literal`, `literal-array`, `text`, `text-array`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
