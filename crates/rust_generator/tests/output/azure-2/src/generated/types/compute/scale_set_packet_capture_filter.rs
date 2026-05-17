#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetPacketCaptureFilter {
    /// The local IP Address to be filtered on. Specify `127.0.0.1` for a single address entry, `127.0.0.1-127.0.0.255` for a range and `127.0.0.1;127.0.0.5` for multiple entries. Multiple ranges and mixing ranges with multiple entries are currently not supported. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "localIpAddress")]
    pub r#local_ip_address: Option<String>,
    /// The local port to be filtered on. Specify `80` for single port entry, `80-85` for a range and `80;443;` for multiple entries. Multiple ranges and mixing ranges with multiple entries are currently not supported. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "localPort")]
    pub r#local_port: Option<String>,
    /// The Protocol to be filtered on. Possible values include `Any`, `TCP` and `UDP`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The remote IP Address to be filtered on. Specify `127.0.0.1` for a single address entry, `127.0.0.1-127.0.0.255` for a range and `127.0.0.1;127.0.0.5` for multiple entries. Multiple ranges and mixing ranges with multiple entries are currently not supported. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "remoteIpAddress")]
    pub r#remote_ip_address: Option<String>,
    /// The remote port to be filtered on. Specify `80` for single port entry, `80-85` for a range and `80;443;` for multiple entries. Multiple ranges and mixing ranges with multiple entries are currently not supported. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "remotePort")]
    pub r#remote_port: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScaleSetPacketCaptureFilter {
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
                    "local_ip_address",
                    &self.r#local_ip_address,
                ),
                to_pulumi_object_field(
                    "local_port",
                    &self.r#local_port,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "remote_ip_address",
                    &self.r#remote_ip_address,
                ),
                to_pulumi_object_field(
                    "remote_port",
                    &self.r#remote_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScaleSetPacketCaptureFilter {
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
                    r#local_ip_address: {
                        let field_value = match fields_map.get("local_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_port: {
                        let field_value = match fields_map.get("local_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_ip_address: {
                        let field_value = match fields_map.get("remote_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_port: {
                        let field_value = match fields_map.get("remote_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
