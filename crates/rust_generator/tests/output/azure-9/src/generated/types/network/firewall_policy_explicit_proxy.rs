#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyExplicitProxy {
    /// Whether the pac file port and url need to be provided.
    #[builder(into)]
    #[serde(rename = "enablePacFile")]
    pub r#enable_pac_file: Option<bool>,
    /// Whether the explicit proxy is enabled for this Firewall Policy.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The port number for explicit http protocol.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Option<i32>,
    /// The port number for explicit proxy https protocol.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Option<i32>,
    /// Specifies a SAS URL for PAC file.
    #[builder(into)]
    #[serde(rename = "pacFile")]
    pub r#pac_file: Option<String>,
    /// Specifies a port number for firewall to serve PAC file.
    #[builder(into)]
    #[serde(rename = "pacFilePort")]
    pub r#pac_file_port: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyExplicitProxy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enable_pac_file",
                    &self.r#enable_pac_file,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "http_port",
                    &self.r#http_port,
                ),
                to_pulumi_object_field(
                    "https_port",
                    &self.r#https_port,
                ),
                to_pulumi_object_field(
                    "pac_file",
                    &self.r#pac_file,
                ),
                to_pulumi_object_field(
                    "pac_file_port",
                    &self.r#pac_file_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyExplicitProxy {
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
                    r#enable_pac_file: {
                        let field_value = match fields_map.get("enable_pac_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_pac_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_port: {
                        let field_value = match fields_map.get("http_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_port: {
                        let field_value = match fields_map.get("https_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pac_file: {
                        let field_value = match fields_map.get("pac_file") {
                            Some(value) => value,
                            None => bail!("Missing field 'pac_file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pac_file_port: {
                        let field_value = match fields_map.get("pac_file_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'pac_file_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
