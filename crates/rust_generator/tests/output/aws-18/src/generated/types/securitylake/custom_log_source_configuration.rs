#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomLogSourceConfiguration {
    /// The configuration for the Glue Crawler for the third-party custom source.
    #[builder(into)]
    #[serde(rename = "crawlerConfiguration")]
    pub r#crawler_configuration: Option<Box<super::super::types::securitylake::CustomLogSourceConfigurationCrawlerConfiguration>>,
    /// The identity of the log provider for the third-party custom source.
    #[builder(into)]
    #[serde(rename = "providerIdentity")]
    pub r#provider_identity: Option<Box<super::super::types::securitylake::CustomLogSourceConfigurationProviderIdentity>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomLogSourceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("crawler_configuration".to_string(), self.r#crawler_configuration.to_pulumi_value().await);
            map.insert("provider_identity".to_string(), self.r#provider_identity.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomLogSourceConfiguration {
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
                    r#crawler_configuration: {
                        let field_value = match fields_map.get("crawler_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'crawler_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::CustomLogSourceConfigurationCrawlerConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#provider_identity: {
                        let field_value = match fields_map.get("provider_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provider_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::CustomLogSourceConfigurationProviderIdentity>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
