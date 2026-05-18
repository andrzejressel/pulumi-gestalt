#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTemplatesTemplate {
    /// Indicates whether the quota is global.
    #[builder(into)]
    #[serde(rename = "globalQuota")]
    pub r#global_quota: bool,
    /// Quota identifier.
    #[builder(into)]
    #[serde(rename = "quotaCode")]
    pub r#quota_code: String,
    /// Quota name.
    #[builder(into)]
    #[serde(rename = "quotaName")]
    pub r#quota_name: String,
    /// AWS Region to which the quota increases apply.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// (Required) Service identifier.
    #[builder(into)]
    #[serde(rename = "serviceCode")]
    pub r#service_code: String,
    /// Service name.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
    /// Unit of measurement.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
    /// (Required) The new, increased value for the quota.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTemplatesTemplate {
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
                    "global_quota",
                    &self.r#global_quota,
                ),
                to_pulumi_object_field(
                    "quota_code",
                    &self.r#quota_code,
                ),
                to_pulumi_object_field(
                    "quota_name",
                    &self.r#quota_name,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "service_code",
                    &self.r#service_code,
                ),
                to_pulumi_object_field(
                    "service_name",
                    &self.r#service_name,
                ),
                to_pulumi_object_field(
                    "unit",
                    &self.r#unit,
                ),
                to_pulumi_object_field(
                    "value",
                    &self.r#value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTemplatesTemplate {
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
                    r#global_quota: {
                        let field_value = match fields_map.get("global_quota") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_quota' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_code: {
                        let field_value = match fields_map.get("quota_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_name: {
                        let field_value = match fields_map.get("quota_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_code: {
                        let field_value = match fields_map.get("service_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_name: {
                        let field_value = match fields_map.get("service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unit: {
                        let field_value = match fields_map.get("unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
