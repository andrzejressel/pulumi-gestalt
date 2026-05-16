#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InsightFiltersFindingProviderFieldsCriticality {
    /// The equal-to condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "eq")]
    pub r#eq: Option<String>,
    /// The greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "gte")]
    pub r#gte: Option<String>,
    /// The less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into)]
    #[serde(rename = "lte")]
    pub r#lte: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InsightFiltersFindingProviderFieldsCriticality {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("eq".to_string(), self.r#eq.to_pulumi_value().await);
            map.insert("gte".to_string(), self.r#gte.to_pulumi_value().await);
            map.insert("lte".to_string(), self.r#lte.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InsightFiltersFindingProviderFieldsCriticality {
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
                    r#eq: {
                        let field_value = match fields_map.get("eq") {
                            Some(value) => value,
                            None => bail!("Missing field 'eq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#gte: {
                        let field_value = match fields_map.get("gte") {
                            Some(value) => value,
                            None => bail!("Missing field 'gte' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lte: {
                        let field_value = match fields_map.get("lte") {
                            Some(value) => value,
                            None => bail!("Missing field 'lte' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
