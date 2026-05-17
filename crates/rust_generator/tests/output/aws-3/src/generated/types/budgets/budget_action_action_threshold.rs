#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetActionActionThreshold {
    /// The type of threshold for a notification. Valid values are `PERCENTAGE` or `ABSOLUTE_VALUE`.
    #[builder(into)]
    #[serde(rename = "actionThresholdType")]
    pub r#action_threshold_type: String,
    /// The threshold of a notification.
    #[builder(into)]
    #[serde(rename = "actionThresholdValue")]
    pub r#action_threshold_value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetActionActionThreshold {
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
                "action_threshold_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_threshold_type,
                )
                .await,
            );
            map.insert(
                "action_threshold_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_threshold_value,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetActionActionThreshold {
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
                    r#action_threshold_type: {
                        let field_value = match fields_map.get("action_threshold_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_threshold_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#action_threshold_value: {
                        let field_value = match fields_map.get("action_threshold_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_threshold_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
