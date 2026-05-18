#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition {
    /// The minimum data risk score that triggers the condition.
    /// Possible values are: `HIGH`, `MEDIUM_OR_HIGH`.
    #[builder(into)]
    #[serde(rename = "minimumRiskScore")]
    pub r#minimum_risk_score: Option<String>,
    /// The minimum sensitivity level that triggers the condition.
    /// Possible values are: `HIGH`, `MEDIUM_OR_HIGH`.
    #[builder(into)]
    #[serde(rename = "minimumSensitivityScore")]
    pub r#minimum_sensitivity_score: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition {
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
                    "minimum_risk_score",
                    &self.r#minimum_risk_score,
                ),
                to_pulumi_object_field(
                    "minimum_sensitivity_score",
                    &self.r#minimum_sensitivity_score,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigActionPubSubNotificationPubsubConditionExpressionsCondition {
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
                    r#minimum_risk_score: {
                        let field_value = match fields_map.get("minimum_risk_score") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_risk_score' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_sensitivity_score: {
                        let field_value = match fields_map.get("minimum_sensitivity_score") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_sensitivity_score' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
