#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm {
    /// The operator to use in the condition.
    #[builder(into)]
    #[serde(rename = "comparator")]
    pub r#comparator: Option<String>,
    /// The tag key to use in the condition. The only valid value is `TAG`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The tag keys or tag key and value pairs to use in the condition.
    #[builder(into)]
    #[serde(rename = "tagValues")]
    pub r#tag_values: Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue>>,
    /// The type of object to apply the condition to. The only valid value is `S3_OBJECT`.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("comparator".to_string(), self.r#comparator.to_pulumi_value().await);
            map.insert("key".to_string(), self.r#key.to_pulumi_value().await);
            map.insert("tag_values".to_string(), self.r#tag_values.to_pulumi_value().await);
            map.insert("target".to_string(), self.r#target.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm {
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
                    r#comparator: {
                        let field_value = match fields_map.get("comparator") {
                            Some(value) => value,
                            None => bail!("Missing field 'comparator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tag_values: {
                        let field_value = match fields_map.get("tag_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
