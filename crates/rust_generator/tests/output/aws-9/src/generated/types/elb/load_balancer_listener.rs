#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerListener {
    /// The port on the instance to route to
    #[builder(into)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: i32,
    /// The protocol to use to the instance. Valid
    /// values are `HTTP`, `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "instanceProtocol")]
    pub r#instance_protocol: String,
    /// The port to listen on for the load balancer
    #[builder(into)]
    #[serde(rename = "lbPort")]
    pub r#lb_port: i32,
    /// The protocol to listen on. Valid values are `HTTP`,
    /// `HTTPS`, `TCP`, or `SSL`
    #[builder(into)]
    #[serde(rename = "lbProtocol")]
    pub r#lb_protocol: String,
    /// The ARN of an SSL certificate you have
    /// uploaded to AWS IAM. **Note ECDSA-specific restrictions below.  Only valid when `lb_protocol` is either HTTPS or SSL**
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerListener {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "instance_port",
                    &self.r#instance_port,
                ),
                to_pulumi_object_field(
                    "instance_protocol",
                    &self.r#instance_protocol,
                ),
                to_pulumi_object_field(
                    "lb_port",
                    &self.r#lb_port,
                ),
                to_pulumi_object_field(
                    "lb_protocol",
                    &self.r#lb_protocol,
                ),
                to_pulumi_object_field(
                    "ssl_certificate_id",
                    &self.r#ssl_certificate_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerListener {
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
                    r#instance_port: {
                        let field_value = match fields_map.get("instance_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_protocol: {
                        let field_value = match fields_map.get("instance_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_port: {
                        let field_value = match fields_map.get("lb_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'lb_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_protocol: {
                        let field_value = match fields_map.get("lb_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'lb_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate_id: {
                        let field_value = match fields_map.get("ssl_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
