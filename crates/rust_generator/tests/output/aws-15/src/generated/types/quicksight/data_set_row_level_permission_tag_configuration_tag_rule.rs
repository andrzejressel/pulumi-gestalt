#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetRowLevelPermissionTagConfigurationTagRule {
    /// Column name that a tag key is assigned to.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: String,
    /// A string that you want to use to filter by all the values in a column in the dataset and don’t want to list the values one by one.
    #[builder(into)]
    #[serde(rename = "matchAllValue")]
    pub r#match_all_value: Option<String>,
    /// Unique key for a tag.
    #[builder(into)]
    #[serde(rename = "tagKey")]
    pub r#tag_key: String,
    /// A string that you want to use to delimit the values when you pass the values at run time.
    #[builder(into)]
    #[serde(rename = "tagMultiValueDelimiter")]
    pub r#tag_multi_value_delimiter: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSetRowLevelPermissionTagConfigurationTagRule {
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
                "column_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#column_name,
                )
                .await,
            );
            map.insert(
                "match_all_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#match_all_value,
                )
                .await,
            );
            map.insert(
                "tag_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_key,
                )
                .await,
            );
            map.insert(
                "tag_multi_value_delimiter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_multi_value_delimiter,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSetRowLevelPermissionTagConfigurationTagRule {
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
                    r#column_name: {
                        let field_value = match fields_map.get("column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_all_value: {
                        let field_value = match fields_map.get("match_all_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_all_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_key: {
                        let field_value = match fields_map.get("tag_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_multi_value_delimiter: {
                        let field_value = match fields_map.get("tag_multi_value_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_multi_value_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
