#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDataSourcesSyslog {
    /// Specifies a list of facility names. Use a wildcard `*` to collect logs for all facility names. Possible values are `alert`, `*`, `audit`, `auth`, `authpriv`, `clock`, `cron`, `daemon`, `ftp`, `kern`, `local5`, `local4`, `local1`, `local7`, `local6`, `local3`, `local2`, `local0`, `lpr`, `mail`, `mark`, `news`, `nopri`, `ntp`, `syslog`, `user` and `uucp`.
    #[builder(into)]
    #[serde(rename = "facilityNames")]
    pub r#facility_names: Vec<String>,
    /// Specifies a list of log levels. Use a wildcard `*` to collect logs for all log levels. Possible values are `Debug`, `Info`, `Notice`, `Warning`, `Error`, `Critical`, `Alert`, `Emergency`,and `*`.
    #[builder(into)]
    #[serde(rename = "logLevels")]
    pub r#log_levels: Vec<String>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible values include but not limited to `Microsoft-Syslog`,and `Microsoft-CiscoAsa`, and `Microsoft-CommonSecurityLog`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCollectionRuleDataSourcesSyslog {
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
                "facility_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#facility_names,
                )
                .await,
            );
            map.insert(
                "log_levels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_levels,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "streams".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#streams,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCollectionRuleDataSourcesSyslog {
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
                    r#facility_names: {
                        let field_value = match fields_map.get("facility_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'facility_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_levels: {
                        let field_value = match fields_map.get("log_levels") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_levels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#streams: {
                        let field_value = match fields_map.get("streams") {
                            Some(value) => value,
                            None => bail!("Missing field 'streams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
