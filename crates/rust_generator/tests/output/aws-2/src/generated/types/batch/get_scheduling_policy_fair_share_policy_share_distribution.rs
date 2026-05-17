#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSchedulingPolicyFairSharePolicyShareDistribution {
    /// Fair share identifier or fair share identifier prefix. For more information, see [ShareAttributes](https://docs.aws.amazon.com/batch/latest/APIReference/API_ShareAttributes.html).
    #[builder(into)]
    #[serde(rename = "shareIdentifier")]
    pub r#share_identifier: String,
    /// Weight factor for the fair share identifier. For more information, see [ShareAttributes](https://docs.aws.amazon.com/batch/latest/APIReference/API_ShareAttributes.html).
    #[builder(into)]
    #[serde(rename = "weightFactor")]
    pub r#weight_factor: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSchedulingPolicyFairSharePolicyShareDistribution {
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
                "share_identifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#share_identifier,
                )
                .await,
            );
            map.insert(
                "weight_factor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weight_factor,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSchedulingPolicyFairSharePolicyShareDistribution {
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
                    r#share_identifier: {
                        let field_value = match fields_map.get("share_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight_factor: {
                        let field_value = match fields_map.get("weight_factor") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight_factor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
