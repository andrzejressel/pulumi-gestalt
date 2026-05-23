#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyExplicitProxy {
    /// Whether the pac file port and url need to be provided.
    #[builder(into)]
    pub r#enable_pac_file: Option<bool>,
    /// Whether the explicit proxy is enabled for this Firewall Policy.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// The port number for explicit http protocol.
    #[builder(into)]
    pub r#http_port: Option<i32>,
    /// The port number for explicit proxy https protocol.
    #[builder(into)]
    pub r#https_port: Option<i32>,
    /// Specifies a SAS URL for PAC file.
    #[builder(into)]
    pub r#pac_file: Option<String>,
    /// Specifies a port number for firewall to serve PAC file.
    #[builder(into)]
    pub r#pac_file_port: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyExplicitProxy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enablePacFile",
                    &self.r#enable_pac_file,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "httpPort",
                    &self.r#http_port,
                ),
                to_pulumi_object_field(
                    "httpsPort",
                    &self.r#https_port,
                ),
                to_pulumi_object_field(
                    "pacFile",
                    &self.r#pac_file,
                ),
                to_pulumi_object_field(
                    "pacFilePort",
                    &self.r#pac_file_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("enablePacFile") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePacFile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("httpPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_port: {
                        let field_value = match fields_map.get("httpsPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpsPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pac_file: {
                        let field_value = match fields_map.get("pacFile") {
                            Some(value) => value,
                            None => bail!("Missing field 'pacFile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pac_file_port: {
                        let field_value = match fields_map.get("pacFilePort") {
                            Some(value) => value,
                            None => bail!("Missing field 'pacFilePort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
