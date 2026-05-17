#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationV2RuleFilter {
    /// Configuration block used to apply a logical `AND` to two or more predicates. See below. The Lifecycle Rule will apply to any object matching all the predicates configured inside the `and` block.
    #[builder(into)]
    #[serde(rename = "and")]
    pub r#and: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterAnd>>,
    /// Minimum object size (in bytes) to which the rule applies.
    #[builder(into)]
    #[serde(rename = "objectSizeGreaterThan")]
    pub r#object_size_greater_than: Option<String>,
    /// Maximum object size (in bytes) to which the rule applies.
    #[builder(into)]
    #[serde(rename = "objectSizeLessThan")]
    pub r#object_size_less_than: Option<String>,
    /// Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if not specified.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Configuration block for specifying a tag key and value. See below.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterTag>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLifecycleConfigurationV2RuleFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "and".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#and,
                )
                .await,
            );
            map.insert(
                "object_size_greater_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object_size_greater_than,
                )
                .await,
            );
            map.insert(
                "object_size_less_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object_size_less_than,
                )
                .await,
            );
            map.insert(
                "prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix,
                )
                .await,
            );
            map.insert(
                "tag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLifecycleConfigurationV2RuleFilter {
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
                    r#and: {
                        let field_value = match fields_map.get("and") {
                            Some(value) => value,
                            None => bail!("Missing field 'and' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_size_greater_than: {
                        let field_value = match fields_map.get("object_size_greater_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_size_greater_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_size_less_than: {
                        let field_value = match fields_map.get("object_size_less_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_size_less_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag: {
                        let field_value = match fields_map.get("tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
