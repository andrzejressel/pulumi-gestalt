#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclLoggingConfigurationLoggingFilterFilter {
    /// Parameter that determines how to handle logs that meet the conditions and requirements of the filter. The valid values for `behavior` are `KEEP` or `DROP`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: String,
    /// Match condition(s) for the filter. See Condition below for more details.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Vec<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterCondition>,
    /// Logic to apply to the filtering conditions. You can specify that a log must match all conditions or at least one condition in order to satisfy the filter. Valid values for `requirement` are `MEETS_ALL` or `MEETS_ANY`.
    #[builder(into)]
    #[serde(rename = "requirement")]
    pub r#requirement: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclLoggingConfigurationLoggingFilterFilter {
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
                "behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#behavior,
                )
                .await,
            );
            map.insert(
                "conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conditions,
                )
                .await,
            );
            map.insert(
                "requirement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requirement,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclLoggingConfigurationLoggingFilterFilter {
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
                    r#behavior: {
                        let field_value = match fields_map.get("behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requirement: {
                        let field_value = match fields_map.get("requirement") {
                            Some(value) => value,
                            None => bail!("Missing field 'requirement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
