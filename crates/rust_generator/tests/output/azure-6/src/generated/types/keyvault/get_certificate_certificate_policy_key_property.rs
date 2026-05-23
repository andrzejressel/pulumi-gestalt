#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateCertificatePolicyKeyProperty {
    #[builder(into)]
    pub r#curve: String,
    /// Is this Certificate Exportable?
    #[builder(into)]
    pub r#exportable: bool,
    /// The size of the Key used in the Certificate.
    #[builder(into)]
    pub r#key_size: i32,
    /// Specifies the Type of Key, for example `RSA`.
    #[builder(into)]
    pub r#key_type: String,
    /// Is the key reusable?
    #[builder(into)]
    pub r#reuse_key: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateCertificatePolicyKeyProperty {
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
                    "curve",
                    &self.r#curve,
                ),
                to_pulumi_object_field(
                    "exportable",
                    &self.r#exportable,
                ),
                to_pulumi_object_field(
                    "keySize",
                    &self.r#key_size,
                ),
                to_pulumi_object_field(
                    "keyType",
                    &self.r#key_type,
                ),
                to_pulumi_object_field(
                    "reuseKey",
                    &self.r#reuse_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateCertificatePolicyKeyProperty {
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
                        let field_value = match fields_map.get("keySize") {
                            Some(value) => value,
                            None => bail!("Missing field 'keySize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_type: {
                        let field_value = match fields_map.get("keyType") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reuse_key: {
                        let field_value = match fields_map.get("reuseKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'reuseKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
