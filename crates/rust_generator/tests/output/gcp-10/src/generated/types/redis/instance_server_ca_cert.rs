#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceServerCaCert {
    /// (Output)
    /// The certificate data in PEM format.
    #[builder(into)]
    pub r#cert: Option<String>,
    /// (Output)
    /// The time when the certificate was created.
    #[builder(into)]
    pub r#create_time: Option<String>,
    /// (Output)
    /// The time when the certificate expires.
    #[builder(into)]
    pub r#expire_time: Option<String>,
    /// (Output)
    /// Serial number, as extracted from the certificate.
    #[builder(into)]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// Sha1 Fingerprint of the certificate.
    #[builder(into)]
    pub r#sha_1_fingerprint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceServerCaCert {
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
                    "cert",
                    &self.r#cert,
                ),
                to_pulumi_object_field(
                    "createTime",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "expireTime",
                    &self.r#expire_time,
                ),
                to_pulumi_object_field(
                    "serialNumber",
                    &self.r#serial_number,
                ),
                to_pulumi_object_field(
                    "sha1Fingerprint",
                    &self.r#sha_1_fingerprint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceServerCaCert {
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
                    r#cert: {
                        let field_value = match fields_map.get("cert") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("createTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'createTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expire_time: {
                        let field_value = match fields_map.get("expireTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'expireTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serialNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'serialNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sha_1_fingerprint: {
                        let field_value = match fields_map.get("sha1Fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'sha1Fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
