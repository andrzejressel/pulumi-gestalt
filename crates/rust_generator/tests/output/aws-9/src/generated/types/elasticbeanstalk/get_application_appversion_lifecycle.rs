#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationAppversionLifecycle {
    /// Specifies whether delete a version's source bundle from S3 when the application version is deleted.
    #[builder(into)]
    #[serde(rename = "deleteSourceFromS3")]
    pub r#delete_source_from_s_3: bool,
    /// Number of days to retain an application version.
    #[builder(into)]
    #[serde(rename = "maxAgeInDays")]
    pub r#max_age_in_days: i32,
    /// Maximum number of application versions to retain.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: i32,
    /// ARN of an IAM service role under which the application version is deleted.  Elastic Beanstalk must have permission to assume this role.
    #[builder(into)]
    #[serde(rename = "serviceRole")]
    pub r#service_role: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationAppversionLifecycle {
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
                    "delete_source_from_s_3",
                    &self.r#delete_source_from_s_3,
                ),
                to_pulumi_object_field(
                    "max_age_in_days",
                    &self.r#max_age_in_days,
                ),
                to_pulumi_object_field(
                    "max_count",
                    &self.r#max_count,
                ),
                to_pulumi_object_field(
                    "service_role",
                    &self.r#service_role,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationAppversionLifecycle {
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
                    r#delete_source_from_s_3: {
                        let field_value = match fields_map.get("delete_source_from_s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_source_from_s_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_age_in_days: {
                        let field_value = match fields_map.get("max_age_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("max_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_role: {
                        let field_value = match fields_map.get("service_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
