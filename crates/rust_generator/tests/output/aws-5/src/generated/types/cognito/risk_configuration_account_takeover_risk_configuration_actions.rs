#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationActions {
    /// Action to take for a high risk. See action block below.
    #[builder(into)]
    #[serde(rename = "highAction")]
    pub r#high_action: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsHighAction>>,
    /// Action to take for a low risk. See action block below.
    #[builder(into)]
    #[serde(rename = "lowAction")]
    pub r#low_action: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsLowAction>>,
    /// Action to take for a medium risk. See action block below.
    #[builder(into)]
    #[serde(rename = "mediumAction")]
    pub r#medium_action: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActionsMediumAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationActions {
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
                "high_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#high_action,
                )
                .await,
            );
            map.insert(
                "low_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#low_action,
                )
                .await,
            );
            map.insert(
                "medium_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#medium_action,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationActions {
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
                    r#high_action: {
                        let field_value = match fields_map.get("high_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'high_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#low_action: {
                        let field_value = match fields_map.get("low_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'low_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#medium_action: {
                        let field_value = match fields_map.get("medium_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'medium_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
