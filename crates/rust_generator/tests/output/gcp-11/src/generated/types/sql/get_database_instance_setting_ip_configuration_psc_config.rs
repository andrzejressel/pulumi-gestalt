#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstanceSettingIpConfigurationPscConfig {
    /// List of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric).
    #[builder(into)]
    #[serde(rename = "allowedConsumerProjects")]
    pub r#allowed_consumer_projects: Vec<String>,
    /// A comma-separated list of networks or a comma-separated list of network-project pairs. Each project in this list is represented by a project number (numeric) or by a project ID (alphanumeric). This allows Private Service Connect connections to be created automatically for the specified networks.
    #[builder(into)]
    #[serde(rename = "pscAutoConnections")]
    pub r#psc_auto_connections: Vec<super::super::types::sql::GetDatabaseInstanceSettingIpConfigurationPscConfigPscAutoConnection>,
    /// Whether PSC connectivity is enabled for this instance.
    #[builder(into)]
    #[serde(rename = "pscEnabled")]
    pub r#psc_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceSettingIpConfigurationPscConfig {
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
                "allowed_consumer_projects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_consumer_projects,
                )
                .await,
            );
            map.insert(
                "psc_auto_connections".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#psc_auto_connections,
                )
                .await,
            );
            map.insert(
                "psc_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#psc_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceSettingIpConfigurationPscConfig {
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
                    r#allowed_consumer_projects: {
                        let field_value = match fields_map.get("allowed_consumer_projects") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_consumer_projects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_auto_connections: {
                        let field_value = match fields_map.get("psc_auto_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_auto_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_enabled: {
                        let field_value = match fields_map.get("psc_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
