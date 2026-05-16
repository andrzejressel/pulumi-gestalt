#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCostCategorySplitChargeRule {
    /// Method that's used to define how to split your source costs across your targets. Valid values are `FIXED`, `PROPORTIONAL`, `EVEN`
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: String,
    /// Configuration block for the parameters for a split charge method. This is only required for the `FIXED` method. See below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Vec<super::super::types::costexplorer::GetCostCategorySplitChargeRuleParameter>,
    /// Cost Category value that you want to split.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// Cost Category values that you want to split costs across. These values can't be used as a source in other split charge rules.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCostCategorySplitChargeRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("method".to_string(), self.r#method.to_pulumi_value().await);
            map.insert("parameters".to_string(), self.r#parameters.to_pulumi_value().await);
            map.insert("source".to_string(), self.r#source.to_pulumi_value().await);
            map.insert("targets".to_string(), self.r#targets.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCostCategorySplitChargeRule {
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
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::costexplorer::GetCostCategorySplitChargeRuleParameter> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#targets: {
                        let field_value = match fields_map.get("targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
