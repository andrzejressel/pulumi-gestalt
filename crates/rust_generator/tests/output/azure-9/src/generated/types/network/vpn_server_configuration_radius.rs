#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnServerConfigurationRadius {
    /// One or more `client_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "clientRootCertificates")]
    pub r#client_root_certificates: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusClientRootCertificate>>,
    /// One or more `server_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "serverRootCertificates")]
    pub r#server_root_certificates: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusServerRootCertificate>>,
    /// One or more `server` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusServer>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnServerConfigurationRadius {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "client_root_certificates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_root_certificates,
                )
                .await,
            );
            map.insert(
                "server_root_certificates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_root_certificates,
                )
                .await,
            );
            map.insert(
                "servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#servers,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnServerConfigurationRadius {
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
                    r#client_root_certificates: {
                        let field_value = match fields_map.get("client_root_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_root_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_root_certificates: {
                        let field_value = match fields_map.get("server_root_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_root_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servers: {
                        let field_value = match fields_map.get("servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
