#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedDatabaseLongTermRetentionPolicy {
    #[builder(into)]
    #[serde(rename = "immutableBackupsEnabled")]
    pub r#immutable_backups_enabled: Option<bool>,
    /// The monthly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 120 months. e.g. `P1Y`, `P1M`, `P4W` or `P30D`. Defaults to `PT0S`.
    #[builder(into)]
    #[serde(rename = "monthlyRetention")]
    pub r#monthly_retention: Option<String>,
    /// The week of year to take the yearly backup. Value has to be between `1` and `52`.
    #[builder(into)]
    #[serde(rename = "weekOfYear")]
    pub r#week_of_year: Option<i32>,
    /// The weekly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 520 weeks. e.g. `P1Y`, `P1M`, `P1W` or `P7D`. Defaults to `PT0S`.
    #[builder(into)]
    #[serde(rename = "weeklyRetention")]
    pub r#weekly_retention: Option<String>,
    /// The yearly retention policy for an LTR backup in an ISO 8601 format. Valid value is between 1 to 10 years. e.g. `P1Y`, `P12M`, `P52W` or `P365D`. Defaults to `PT0S`.
    #[builder(into)]
    #[serde(rename = "yearlyRetention")]
    pub r#yearly_retention: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedDatabaseLongTermRetentionPolicy {
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
                "immutable_backups_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#immutable_backups_enabled,
                )
                .await,
            );
            map.insert(
                "monthly_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthly_retention,
                )
                .await,
            );
            map.insert(
                "week_of_year".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#week_of_year,
                )
                .await,
            );
            map.insert(
                "weekly_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekly_retention,
                )
                .await,
            );
            map.insert(
                "yearly_retention".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#yearly_retention,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedDatabaseLongTermRetentionPolicy {
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
                    r#immutable_backups_enabled: {
                        let field_value = match fields_map.get("immutable_backups_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'immutable_backups_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_retention: {
                        let field_value = match fields_map.get("monthly_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#week_of_year: {
                        let field_value = match fields_map.get("week_of_year") {
                            Some(value) => value,
                            None => bail!("Missing field 'week_of_year' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_retention: {
                        let field_value = match fields_map.get("weekly_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekly_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yearly_retention: {
                        let field_value = match fields_map.get("yearly_retention") {
                            Some(value) => value,
                            None => bail!("Missing field 'yearly_retention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
