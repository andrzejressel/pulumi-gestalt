#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd {
    /// The fractional portion, in cents, of the amount. Valid value range between `0` and `99`.
    #[builder(into)]
    #[serde(rename = "cents")]
    pub r#cents: Option<i32>,
    /// The whole number of dollars in the amount. Valid value range between `0` and `2`.
    #[builder(into)]
    #[serde(rename = "dollars")]
    pub r#dollars: Option<i32>,
    /// Fractions of a cent, in tenths. Valid value range between `0` and `9`.
    #[builder(into)]
    #[serde(rename = "tenthFractionsOfACent")]
    pub r#tenth_fractions_of_a_cent: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cents".to_string(), self.r#cents.to_pulumi_value().await);
            map.insert("dollars".to_string(), self.r#dollars.to_pulumi_value().await);
            map.insert("tenth_fractions_of_a_cent".to_string(), self.r#tenth_fractions_of_a_cent.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd {
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
                    r#cents: {
                        let field_value = match fields_map.get("cents") {
                            Some(value) => value,
                            None => bail!("Missing field 'cents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dollars: {
                        let field_value = match fields_map.get("dollars") {
                            Some(value) => value,
                            None => bail!("Missing field 'dollars' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tenth_fractions_of_a_cent: {
                        let field_value = match fields_map.get("tenth_fractions_of_a_cent") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenth_fractions_of_a_cent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
