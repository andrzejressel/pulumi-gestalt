#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperimentTemplateTarget {
    /// Filter(s) for the target. Filters can be used to select resources based on specific attributes returned by the respective describe action of the resource type. For more information, see [Targets for AWS FIS](https://docs.aws.amazon.com/fis/latest/userguide/targets.html#target-filters). See below.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Option<Vec<super::super::types::fis::ExperimentTemplateTargetFilter>>,
    /// Friendly name given to the target.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The resource type parameters.
    /// 
    /// > **NOTE:** The `target` configuration block requires either `resource_arns` or `resource_tag`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Set of ARNs of the resources to target with an action. Conflicts with `resource_tag`.
    #[builder(into)]
    #[serde(rename = "resourceArns")]
    pub r#resource_arns: Option<Vec<String>>,
    /// Tag(s) the resources need to have to be considered a valid target for an action. Conflicts with `resource_arns`. See below.
    #[builder(into)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Option<Vec<super::super::types::fis::ExperimentTemplateTargetResourceTag>>,
    /// AWS resource type. The resource type must be supported for the specified action. To find out what resource types are supported, see [Targets for AWS FIS](https://docs.aws.amazon.com/fis/latest/userguide/targets.html#resource-types).
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
    /// Scopes the identified resources. Valid values are `ALL` (all identified resources), `COUNT(n)` (randomly select `n` of the identified resources), `PERCENT(n)` (randomly select `n` percent of the identified resources).
    #[builder(into)]
    #[serde(rename = "selectionMode")]
    pub r#selection_mode: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExperimentTemplateTarget {
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
                "filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filters,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "resource_arns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_arns,
                )
                .await,
            );
            map.insert(
                "resource_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_tags,
                )
                .await,
            );
            map.insert(
                "resource_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_type,
                )
                .await,
            );
            map.insert(
                "selection_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selection_mode,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExperimentTemplateTarget {
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
                    r#filters: {
                        let field_value = match fields_map.get("filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_arns: {
                        let field_value = match fields_map.get("resource_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_tags: {
                        let field_value = match fields_map.get("resource_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selection_mode: {
                        let field_value = match fields_map.get("selection_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'selection_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
