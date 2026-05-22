#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetManagedDatabaseLongTermRetentionPolicy {
    /// Specifies if the backups are immutable.
    #[builder(into)]
    #[serde(rename = "immutableBackupsEnabled")]
    pub r#immutable_backups_enabled: bool,
    /// The monthly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "monthlyRetention")]
    pub r#monthly_retention: String,
    /// The week of year to take the yearly backup.
    #[builder(into)]
    #[serde(rename = "weekOfYear")]
    pub r#week_of_year: i32,
    /// The weekly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "weeklyRetention")]
    pub r#weekly_retention: String,
    /// The yearly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "yearlyRetention")]
    pub r#yearly_retention: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetManagedDatabaseLongTermRetentionPolicy {
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
                    "immutableBackupsEnabled",
                    &self.r#immutable_backups_enabled,
                ),
                to_pulumi_object_field(
                    "monthlyRetention",
                    &self.r#monthly_retention,
                ),
                to_pulumi_object_field(
                    "weekOfYear",
                    &self.r#week_of_year,
                ),
                to_pulumi_object_field(
                    "weeklyRetention",
                    &self.r#weekly_retention,
                ),
                to_pulumi_object_field(
                    "yearlyRetention",
                    &self.r#yearly_retention,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetManagedDatabaseLongTermRetentionPolicy {
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
                        let field_value = match fields_map.get("immutableBackupsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'immutableBackupsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_retention: {
                        let field_value = match fields_map.get("monthlyRetention") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyRetention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#week_of_year: {
                        let field_value = match fields_map.get("weekOfYear") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekOfYear' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_retention: {
                        let field_value = match fields_map.get("weeklyRetention") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklyRetention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#yearly_retention: {
                        let field_value = match fields_map.get("yearlyRetention") {
                            Some(value) => value,
                            None => bail!("Missing field 'yearlyRetention' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
