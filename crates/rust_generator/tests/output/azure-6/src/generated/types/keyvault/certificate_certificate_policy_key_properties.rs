#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificatePolicyKeyProperties {
    /// Specifies the curve to use when creating an `EC` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field will be required in a future release if `key_type` is `EC` or `EC-HSM`.
    #[builder(into)]
    #[serde(rename = "curve")]
    pub r#curve: Option<String>,
    /// Is this certificate exportable?
    #[builder(into)]
    #[serde(rename = "exportable")]
    pub r#exportable: bool,
    /// The size of the key used in the certificate. Possible values include `2048`, `3072`, and `4096` for `RSA` keys, or `256`, `384`, and `521` for `EC` keys. This property is required when using RSA keys.
    #[builder(into)]
    #[serde(rename = "keySize")]
    pub r#key_size: Option<i32>,
    /// Specifies the type of key. Possible values are `EC`, `EC-HSM`, `RSA`, `RSA-HSM` and `oct`.
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: String,
    /// Is the key reusable?
    #[builder(into)]
    #[serde(rename = "reuseKey")]
    pub r#reuse_key: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificatePolicyKeyProperties {
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
                    "curve",
                    &self.r#curve,
                ),
                to_pulumi_object_field(
                    "exportable",
                    &self.r#exportable,
                ),
                to_pulumi_object_field(
                    "key_size",
                    &self.r#key_size,
                ),
                to_pulumi_object_field(
                    "key_type",
                    &self.r#key_type,
                ),
                to_pulumi_object_field(
                    "reuse_key",
                    &self.r#reuse_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificatePolicyKeyProperties {
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
                    r#curve: {
                        let field_value = match fields_map.get("curve") {
                            Some(value) => value,
                            None => bail!("Missing field 'curve' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exportable: {
                        let field_value = match fields_map.get("exportable") {
                            Some(value) => value,
                            None => bail!("Missing field 'exportable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_size: {
                        let field_value = match fields_map.get("key_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_type: {
                        let field_value = match fields_map.get("key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reuse_key: {
                        let field_value = match fields_map.get("reuse_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'reuse_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
