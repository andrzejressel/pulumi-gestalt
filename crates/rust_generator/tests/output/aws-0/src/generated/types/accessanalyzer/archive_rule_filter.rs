#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ArchiveRuleFilter {
    /// Contains comparator.
    #[builder(into)]
    #[serde(rename = "contains")]
    pub r#contains: Option<Vec<String>>,
    /// Filter criteria.
    #[builder(into)]
    #[serde(rename = "criteria")]
    pub r#criteria: String,
    /// Equals comparator.
    #[builder(into)]
    #[serde(rename = "eqs")]
    pub r#eqs: Option<Vec<String>>,
    /// Boolean comparator.
    #[builder(into)]
    #[serde(rename = "exists")]
    pub r#exists: Option<String>,
    /// Not Equals comparator.
    #[builder(into)]
    #[serde(rename = "neqs")]
    pub r#neqs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ArchiveRuleFilter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("contains".to_string(), self.r#contains.to_pulumi_value().await);
            map.insert("criteria".to_string(), self.r#criteria.to_pulumi_value().await);
            map.insert("eqs".to_string(), self.r#eqs.to_pulumi_value().await);
            map.insert("exists".to_string(), self.r#exists.to_pulumi_value().await);
            map.insert("neqs".to_string(), self.r#neqs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ArchiveRuleFilter {
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
                    r#contains: {
                        let field_value = match fields_map.get("contains") {
                            Some(value) => value,
                            None => bail!("Missing field 'contains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#criteria: {
                        let field_value = match fields_map.get("criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#eqs: {
                        let field_value = match fields_map.get("eqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'eqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exists: {
                        let field_value = match fields_map.get("exists") {
                            Some(value) => value,
                            None => bail!("Missing field 'exists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#neqs: {
                        let field_value = match fields_map.get("neqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'neqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
