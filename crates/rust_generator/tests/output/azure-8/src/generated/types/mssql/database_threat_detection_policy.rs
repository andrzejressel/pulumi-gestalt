#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseThreatDetectionPolicy {
    /// Specifies a list of alerts which should be disabled. Possible values include `Access_Anomaly`, `Sql_Injection` and `Sql_Injection_Vulnerability`.
    #[builder(into)]
    #[serde(rename = "disabledAlerts")]
    pub r#disabled_alerts: Option<Vec<String>>,
    /// Should the account administrators be emailed when this alert is triggered? Possible values are `Enabled` or `Disabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "emailAccountAdmins")]
    pub r#email_account_admins: Option<String>,
    /// A list of email addresses which alerts should be sent to.
    #[builder(into)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Option<Vec<String>>,
    /// Specifies the number of days to keep in the Threat Detection audit logs.
    #[builder(into)]
    #[serde(rename = "retentionDays")]
    pub r#retention_days: Option<i32>,
    /// The State of the Policy. Possible values are `Enabled` or `Disabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Specifies the identifier key of the Threat Detection audit storage account. Required if `state` is `Enabled`.
    #[builder(into)]
    #[serde(rename = "storageAccountAccessKey")]
    pub r#storage_account_access_key: Option<String>,
    /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs. Required if `state` is `Enabled`.
    #[builder(into)]
    #[serde(rename = "storageEndpoint")]
    pub r#storage_endpoint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseThreatDetectionPolicy {
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
                "disabled_alerts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_alerts,
                )
                .await,
            );
            map.insert(
                "email_account_admins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_account_admins,
                )
                .await,
            );
            map.insert(
                "email_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_addresses,
                )
                .await,
            );
            map.insert(
                "retention_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retention_days,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "storage_account_access_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_access_key,
                )
                .await,
            );
            map.insert(
                "storage_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_endpoint,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseThreatDetectionPolicy {
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
                    r#disabled_alerts: {
                        let field_value = match fields_map.get("disabled_alerts") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_alerts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_account_admins: {
                        let field_value = match fields_map.get("email_account_admins") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_account_admins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_addresses: {
                        let field_value = match fields_map.get("email_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_days: {
                        let field_value = match fields_map.get("retention_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_access_key: {
                        let field_value = match fields_map.get("storage_account_access_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_access_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_endpoint: {
                        let field_value = match fields_map.get("storage_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
