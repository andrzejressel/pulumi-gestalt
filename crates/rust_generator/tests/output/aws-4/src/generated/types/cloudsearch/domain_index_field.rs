#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainIndexField {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("analysis_scheme".to_string(), self.r#analysis_scheme.to_pulumi_value().await);
            map.insert("default_value".to_string(), self.r#default_value.to_pulumi_value().await);
            map.insert("facet".to_string(), self.r#facet.to_pulumi_value().await);
            map.insert("highlight".to_string(), self.r#highlight.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("return_".to_string(), self.r#return_.to_pulumi_value().await);
            map.insert("search".to_string(), self.r#search.to_pulumi_value().await);
            map.insert("sort".to_string(), self.r#sort.to_pulumi_value().await);
            map.insert("source_fields".to_string(), self.r#source_fields.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainIndexField {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#analysis_scheme: {
                        let field_value = match fields_map.get("analysis_scheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'analysis_scheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_value: {
                        let field_value = match fields_map.get("default_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#facet: {
                        let field_value = match fields_map.get("facet") {
                            Some(value) => value,
                            None => bail!("Missing field 'facet' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#highlight: {
                        let field_value = match fields_map.get("highlight") {
                            Some(value) => value,
                            None => bail!("Missing field 'highlight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#return_: {
                        let field_value = match fields_map.get("return_") {
                            Some(value) => value,
                            None => bail!("Missing field 'return_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#search: {
                        let field_value = match fields_map.get("search") {
                            Some(value) => value,
                            None => bail!("Missing field 'search' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sort: {
                        let field_value = match fields_map.get("sort") {
                            Some(value) => value,
                            None => bail!("Missing field 'sort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source_fields: {
                        let field_value = match fields_map.get("source_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
