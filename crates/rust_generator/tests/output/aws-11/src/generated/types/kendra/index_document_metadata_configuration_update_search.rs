#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexDocumentMetadataConfigurationUpdateSearch {
    /// Determines whether the field is returned in the query response. The default is `true`.
    #[builder(into)]
    #[serde(rename = "displayable")]
    pub r#displayable: Option<bool>,
    /// Indicates that the field can be used to create search facets, a count of results for each value in the field. The default is `false`.
    #[builder(into)]
    #[serde(rename = "facetable")]
    pub r#facetable: Option<bool>,
    /// Determines whether the field is used in the search. If the Searchable field is true, you can use relevance tuning to manually tune how Amazon Kendra weights the field in the search. The default is `true` for `string` fields and `false` for `number` and `date` fields.
    #[builder(into)]
    #[serde(rename = "searchable")]
    pub r#searchable: Option<bool>,
    /// Determines whether the field can be used to sort the results of a query. If you specify sorting on a field that does not have Sortable set to true, Amazon Kendra returns an exception. The default is `false`.
    #[builder(into)]
    #[serde(rename = "sortable")]
    pub r#sortable: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexDocumentMetadataConfigurationUpdateSearch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "displayable",
                    &self.r#displayable,
                ),
                to_pulumi_object_field(
                    "facetable",
                    &self.r#facetable,
                ),
                to_pulumi_object_field(
                    "searchable",
                    &self.r#searchable,
                ),
                to_pulumi_object_field(
                    "sortable",
                    &self.r#sortable,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexDocumentMetadataConfigurationUpdateSearch {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#displayable: {
                        let field_value = match fields_map.get("displayable") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#facetable: {
                        let field_value = match fields_map.get("facetable") {
                            Some(value) => value,
                            None => bail!("Missing field 'facetable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#searchable: {
                        let field_value = match fields_map.get("searchable") {
                            Some(value) => value,
                            None => bail!("Missing field 'searchable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sortable: {
                        let field_value = match fields_map.get("sortable") {
                            Some(value) => value,
                            None => bail!("Missing field 'sortable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
