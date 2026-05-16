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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("behavior".to_string(), self.r#behavior.to_pulumi_value().await);
            map.insert("conditions".to_string(), self.r#conditions.to_pulumi_value().await);
            map.insert("requirement".to_string(), self.r#requirement.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclLoggingConfigurationLoggingFilterFilter {
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
                    r#behavior: {
                        let field_value = match fields_map.get("behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterCondition> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#requirement: {
                        let field_value = match fields_map.get("requirement") {
                            Some(value) => value,
                            None => bail!("Missing field 'requirement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
