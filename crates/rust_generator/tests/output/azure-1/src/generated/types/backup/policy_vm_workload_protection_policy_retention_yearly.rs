#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmWorkloadProtectionPolicyRetentionYearly {
    /// The number of yearly backups to keep. Possible values are between `1` and `99`
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The retention schedule format type for yearly retention policy. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "formatType")]
    pub r#format_type: String,
    /// The monthday backups to retain. Possible values are between `0` and `28`.
    #[builder(into)]
    #[serde(rename = "monthdays")]
    pub r#monthdays: Option<Vec<i32>>,
    /// The months of the year to retain backups of. Possible values are `January`, `February`, `March`, `April`, `May`, `June`, `July`, `August`, `September`, `October`, `November` and `December`.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Vec<String>,
    /// The weekday backups to retain. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` or `Saturday`.
    #[builder(into)]
    #[serde(rename = "weekdays")]
    pub r#weekdays: Option<Vec<String>>,
    /// The weeks of the month to retain backups of. Possible values are `First`, `Second`, `Third`, `Fourth`, `Last`.
    #[builder(into)]
    #[serde(rename = "weeks")]
    pub r#weeks: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyVmWorkloadProtectionPolicyRetentionYearly {
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
                "count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count,
                )
                .await,
            );
            map.insert(
                "format_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#format_type,
                )
                .await,
            );
            map.insert(
                "monthdays".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthdays,
                )
                .await,
            );
            map.insert(
                "months".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#months,
                )
                .await,
            );
            map.insert(
                "weekdays".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weekdays,
                )
                .await,
            );
            map.insert(
                "weeks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weeks,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyVmWorkloadProtectionPolicyRetentionYearly {
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
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#format_type: {
                        let field_value = match fields_map.get("format_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'format_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthdays: {
                        let field_value = match fields_map.get("monthdays") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthdays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#months: {
                        let field_value = match fields_map.get("months") {
                            Some(value) => value,
                            None => bail!("Missing field 'months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekdays: {
                        let field_value = match fields_map.get("weekdays") {
                            Some(value) => value,
                            None => bail!("Missing field 'weekdays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weeks: {
                        let field_value = match fields_map.get("weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
