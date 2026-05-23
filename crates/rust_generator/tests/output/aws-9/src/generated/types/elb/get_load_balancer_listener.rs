#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerListener {
    #[builder(into)]
    pub r#instance_port: i32,
    #[builder(into)]
    pub r#instance_protocol: String,
    #[builder(into)]
    pub r#lb_port: i32,
    #[builder(into)]
    pub r#lb_protocol: String,
    #[builder(into)]
    pub r#ssl_certificate_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLoadBalancerListener {
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
                    "instancePort",
                    &self.r#instance_port,
                ),
                to_pulumi_object_field(
                    "instanceProtocol",
                    &self.r#instance_protocol,
                ),
                to_pulumi_object_field(
                    "lbPort",
                    &self.r#lb_port,
                ),
                to_pulumi_object_field(
                    "lbProtocol",
                    &self.r#lb_protocol,
                ),
                to_pulumi_object_field(
                    "sslCertificateId",
                    &self.r#ssl_certificate_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLoadBalancerListener {
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
                        let field_value = match fields_map.get("instancePort") {
                            Some(value) => value,
                            None => bail!("Missing field 'instancePort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_protocol: {
                        let field_value = match fields_map.get("instanceProtocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceProtocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_port: {
                        let field_value = match fields_map.get("lbPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'lbPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lb_protocol: {
                        let field_value = match fields_map.get("lbProtocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'lbProtocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate_id: {
                        let field_value = match fields_map.get("sslCertificateId") {
                            Some(value) => value,
                            None => bail!("Missing field 'sslCertificateId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
