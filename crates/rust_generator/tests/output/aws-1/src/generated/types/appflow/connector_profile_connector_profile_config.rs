#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfig {
    /// The connector-specific credentials required by each connector. See Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "connectorProfileCredentials")]
    pub r#connector_profile_credentials: Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentials>,
    /// The connector-specific properties of the profile configuration. See Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "connectorProfileProperties")]
    pub r#connector_profile_properties: Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileProperties>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfig {
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
                "connector_profile_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connector_profile_credentials,
                )
                .await,
            );
            map.insert(
                "connector_profile_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connector_profile_properties,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfig {
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
                    r#connector_profile_credentials: {
                        let field_value = match fields_map.get("connector_profile_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_profile_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_profile_properties: {
                        let field_value = match fields_map.get("connector_profile_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_profile_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
