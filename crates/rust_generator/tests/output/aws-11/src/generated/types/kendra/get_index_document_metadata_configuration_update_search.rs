#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetIndexDocumentMetadataConfigurationUpdateSearch {
    /// Determines whether the field is returned in the query response. The default is `true`.
    #[builder(into)]
    #[serde(rename = "displayable")]
    pub r#displayable: bool,
    /// Whether the field can be used to create search facets, a count of results for each value in the field. The default is `false`.
    #[builder(into)]
    #[serde(rename = "facetable")]
    pub r#facetable: bool,
    /// Determines whether the field is used in the search. If the Searchable field is true, you can use relevance tuning to manually tune how Amazon Kendra weights the field in the search. The default is `true` for `string` fields and `false` for `number` and `date` fields.
    #[builder(into)]
    #[serde(rename = "searchable")]
    pub r#searchable: bool,
    /// Determines whether the field can be used to sort the results of a query. If you specify sorting on a field that does not have Sortable set to true, Amazon Kendra returns an exception. The default is `false`.
    #[builder(into)]
    #[serde(rename = "sortable")]
    pub r#sortable: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetIndexDocumentMetadataConfigurationUpdateSearch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("displayable".to_string(), self.r#displayable.to_pulumi_value().await);
            map.insert("facetable".to_string(), self.r#facetable.to_pulumi_value().await);
            map.insert("searchable".to_string(), self.r#searchable.to_pulumi_value().await);
            map.insert("sortable".to_string(), self.r#sortable.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetIndexDocumentMetadataConfigurationUpdateSearch {
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
                    r#displayable: {
                        let field_value = match fields_map.get("displayable") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#facetable: {
                        let field_value = match fields_map.get("facetable") {
                            Some(value) => value,
                            None => bail!("Missing field 'facetable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#searchable: {
                        let field_value = match fields_map.get("searchable") {
                            Some(value) => value,
                            None => bail!("Missing field 'searchable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sortable: {
                        let field_value = match fields_map.get("sortable") {
                            Some(value) => value,
                            None => bail!("Missing field 'sortable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
