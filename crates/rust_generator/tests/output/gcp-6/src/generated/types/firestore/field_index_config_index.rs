#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FieldIndexConfigIndex {
    /// Indicates that this field supports operations on arrayValues. Only one of `order` and `arrayConfig` can
    /// be specified.
    /// Possible values are: `CONTAINS`.
    #[builder(into)]
    #[serde(rename = "arrayConfig")]
    pub r#array_config: Option<String>,
    /// Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=, !=.
    /// Only one of `order` and `arrayConfig` can be specified.
    /// Possible values are: `ASCENDING`, `DESCENDING`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// The scope at which a query is run. Collection scoped queries require you specify
    /// the collection at query time. Collection group scope allows queries across all
    /// collections with the same id.
    /// Default value is `COLLECTION`.
    /// Possible values are: `COLLECTION`, `COLLECTION_GROUP`.
    #[builder(into)]
    #[serde(rename = "queryScope")]
    pub r#query_scope: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FieldIndexConfigIndex {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "array_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#array_config,
                )
                .await,
            );
            map.insert(
                "order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#order,
                )
                .await,
            );
            map.insert(
                "query_scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_scope,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FieldIndexConfigIndex {
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
                    r#array_config: {
                        let field_value = match fields_map.get("array_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'array_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_scope: {
                        let field_value = match fields_map.get("query_scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
