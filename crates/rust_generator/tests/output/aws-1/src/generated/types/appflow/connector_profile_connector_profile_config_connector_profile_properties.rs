#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileProperties {
    /// The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude>>,
    /// The connector-specific profile properties required when using the custom connector. See Custom Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector>>,
    /// Connector-specific properties required when using Datadog. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "datadog")]
    pub r#datadog: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog>>,
    /// The connector-specific properties required when using Dynatrace. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace>>,
    /// The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics>>,
    /// The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode>>,
    /// The connector-specific properties required when using Infor Nexus. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus>>,
    /// Connector-specific properties required when using Marketo. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "marketo")]
    pub r#marketo: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo>>,
    /// Connector-specific properties required when using Amazon Redshift. See Redshift Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift>>,
    /// The connector-specific properties required when using Salesforce. See Salesforce Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce>>,
    /// The connector-specific properties required when using SAPOData. See SAPOData Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData>>,
    /// The connector-specific properties required when using ServiceNow. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow>>,
    /// Connector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "singular")]
    pub r#singular: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular>>,
    /// Connector-specific properties required when using Slack. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "slack")]
    pub r#slack: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack>>,
    /// The connector-specific properties required when using Snowflake. See Snowflake Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake>>,
    /// The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro>>,
    /// Connector-specific properties required when using Veeva. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "veeva")]
    pub r#veeva: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva>>,
    /// Connector-specific properties required when using Zendesk. See Generic Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileProperties {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("amplitude".to_string(), self.r#amplitude.to_pulumi_value().await);
            map.insert("custom_connector".to_string(), self.r#custom_connector.to_pulumi_value().await);
            map.insert("datadog".to_string(), self.r#datadog.to_pulumi_value().await);
            map.insert("dynatrace".to_string(), self.r#dynatrace.to_pulumi_value().await);
            map.insert("google_analytics".to_string(), self.r#google_analytics.to_pulumi_value().await);
            map.insert("honeycode".to_string(), self.r#honeycode.to_pulumi_value().await);
            map.insert("infor_nexus".to_string(), self.r#infor_nexus.to_pulumi_value().await);
            map.insert("marketo".to_string(), self.r#marketo.to_pulumi_value().await);
            map.insert("redshift".to_string(), self.r#redshift.to_pulumi_value().await);
            map.insert("salesforce".to_string(), self.r#salesforce.to_pulumi_value().await);
            map.insert("sapo_data".to_string(), self.r#sapo_data.to_pulumi_value().await);
            map.insert("service_now".to_string(), self.r#service_now.to_pulumi_value().await);
            map.insert("singular".to_string(), self.r#singular.to_pulumi_value().await);
            map.insert("slack".to_string(), self.r#slack.to_pulumi_value().await);
            map.insert("snowflake".to_string(), self.r#snowflake.to_pulumi_value().await);
            map.insert("trendmicro".to_string(), self.r#trendmicro.to_pulumi_value().await);
            map.insert("veeva".to_string(), self.r#veeva.to_pulumi_value().await);
            map.insert("zendesk".to_string(), self.r#zendesk.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileProperties {
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
                    r#amplitude: {
                        let field_value = match fields_map.get("amplitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'amplitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#custom_connector: {
                        let field_value = match fields_map.get("custom_connector") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_connector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#datadog: {
                        let field_value = match fields_map.get("datadog") {
                            Some(value) => value,
                            None => bail!("Missing field 'datadog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dynatrace: {
                        let field_value = match fields_map.get("dynatrace") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynatrace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#google_analytics: {
                        let field_value = match fields_map.get("google_analytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_analytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#honeycode: {
                        let field_value = match fields_map.get("honeycode") {
                            Some(value) => value,
                            None => bail!("Missing field 'honeycode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#infor_nexus: {
                        let field_value = match fields_map.get("infor_nexus") {
                            Some(value) => value,
                            None => bail!("Missing field 'infor_nexus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#marketo: {
                        let field_value = match fields_map.get("marketo") {
                            Some(value) => value,
                            None => bail!("Missing field 'marketo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#redshift: {
                        let field_value = match fields_map.get("redshift") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#salesforce: {
                        let field_value = match fields_map.get("salesforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'salesforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sapo_data: {
                        let field_value = match fields_map.get("sapo_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'sapo_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_now: {
                        let field_value = match fields_map.get("service_now") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_now' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#singular: {
                        let field_value = match fields_map.get("singular") {
                            Some(value) => value,
                            None => bail!("Missing field 'singular' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#slack: {
                        let field_value = match fields_map.get("slack") {
                            Some(value) => value,
                            None => bail!("Missing field 'slack' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#snowflake: {
                        let field_value = match fields_map.get("snowflake") {
                            Some(value) => value,
                            None => bail!("Missing field 'snowflake' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trendmicro: {
                        let field_value = match fields_map.get("trendmicro") {
                            Some(value) => value,
                            None => bail!("Missing field 'trendmicro' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#veeva: {
                        let field_value = match fields_map.get("veeva") {
                            Some(value) => value,
                            None => bail!("Missing field 'veeva' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#zendesk: {
                        let field_value = match fields_map.get("zendesk") {
                            Some(value) => value,
                            None => bail!("Missing field 'zendesk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
