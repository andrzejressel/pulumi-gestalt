#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationProvidersApplicationProvider {
    /// ARN of the application provider.
    #[builder(into)]
    #[serde(rename = "applicationProviderArn")]
    pub r#application_provider_arn: String,
    /// An object describing how IAM Identity Center represents the application provider in the portal. See `display_data` below.
    #[builder(into)]
    #[serde(rename = "displayDatas")]
    pub r#display_datas: Option<Vec<super::super::types::ssoadmin::GetApplicationProvidersApplicationProviderDisplayData>>,
    /// Protocol that the application provider uses to perform federation. Valid values are `SAML` and `OAUTH`.
    #[builder(into)]
    #[serde(rename = "federationProtocol")]
    pub r#federation_protocol: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationProvidersApplicationProvider {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("application_provider_arn".to_string(), self.r#application_provider_arn.to_pulumi_value().await);
            map.insert("display_datas".to_string(), self.r#display_datas.to_pulumi_value().await);
            map.insert("federation_protocol".to_string(), self.r#federation_protocol.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationProvidersApplicationProvider {
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
                    r#application_provider_arn: {
                        let field_value = match fields_map.get("application_provider_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_provider_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#display_datas: {
                        let field_value = match fields_map.get("display_datas") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_datas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::ssoadmin::GetApplicationProvidersApplicationProviderDisplayData>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#federation_protocol: {
                        let field_value = match fields_map.get("federation_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'federation_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
