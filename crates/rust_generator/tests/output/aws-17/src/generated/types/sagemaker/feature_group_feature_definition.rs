#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureGroupFeatureDefinition {
    #[builder(into)]
    #[serde(rename = "collectionConfig")]
    pub r#collection_config: Option<Box<super::super::types::sagemaker::FeatureGroupFeatureDefinitionCollectionConfig>>,
    #[builder(into)]
    #[serde(rename = "collectionType")]
    pub r#collection_type: Option<String>,
    /// The name of a feature. `feature_name` cannot be any of the following: `is_deleted`, `write_time`, `api_invocation_time`.
    #[builder(into)]
    #[serde(rename = "featureName")]
    pub r#feature_name: Option<String>,
    /// The value type of a feature. Valid values are `Integral`, `Fractional`, or `String`.
    #[builder(into)]
    #[serde(rename = "featureType")]
    pub r#feature_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureGroupFeatureDefinition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "collection_config",
                    &self.r#collection_config,
                ),
                to_pulumi_object_field(
                    "collection_type",
                    &self.r#collection_type,
                ),
                to_pulumi_object_field(
                    "feature_name",
                    &self.r#feature_name,
                ),
                to_pulumi_object_field(
                    "feature_type",
                    &self.r#feature_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureGroupFeatureDefinition {
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
                    r#collection_config: {
                        let field_value = match fields_map.get("collection_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'collection_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collection_type: {
                        let field_value = match fields_map.get("collection_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'collection_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#feature_name: {
                        let field_value = match fields_map.get("feature_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'feature_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#feature_type: {
                        let field_value = match fields_map.get("feature_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'feature_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
