#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutonomousDatabasePropertiesConnectionStringAllConnectionString {
    /// The database service provides the highest level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "high")]
    pub r#high: Option<String>,
    /// The database service provides the least level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "low")]
    pub r#low: Option<String>,
    /// The database service provides a lower level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "medium")]
    pub r#medium: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutonomousDatabasePropertiesConnectionStringAllConnectionString {
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
                "high".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#high,
                )
                .await,
            );
            map.insert(
                "low".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#low,
                )
                .await,
            );
            map.insert(
                "medium".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#medium,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutonomousDatabasePropertiesConnectionStringAllConnectionString {
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
                    r#high: {
                        let field_value = match fields_map.get("high") {
                            Some(value) => value,
                            None => bail!("Missing field 'high' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#low: {
                        let field_value = match fields_map.get("low") {
                            Some(value) => value,
                            None => bail!("Missing field 'low' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#medium: {
                        let field_value = match fields_map.get("medium") {
                            Some(value) => value,
                            None => bail!("Missing field 'medium' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
