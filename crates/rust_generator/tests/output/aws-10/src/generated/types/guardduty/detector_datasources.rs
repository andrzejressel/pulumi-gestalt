#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DetectorDatasources {
    /// Configures [Kubernetes protection](https://docs.aws.amazon.com/guardduty/latest/ug/kubernetes-protection.html).
    /// See Kubernetes and Kubernetes Audit Logs below for more details.
    #[builder(into)]
    #[serde(rename = "kubernetes")]
    pub r#kubernetes: Option<Box<super::super::types::guardduty::DetectorDatasourcesKubernetes>>,
    /// Configures [Malware Protection](https://docs.aws.amazon.com/guardduty/latest/ug/malware-protection.html).
    /// See Malware Protection, Scan EC2 instance with findings and EBS volumes below for more details.
    #[builder(into)]
    #[serde(rename = "malwareProtection")]
    pub r#malware_protection: Option<Box<super::super::types::guardduty::DetectorDatasourcesMalwareProtection>>,
    /// Configures [S3 protection](https://docs.aws.amazon.com/guardduty/latest/ug/s3-protection.html).
    /// See S3 Logs below for more details.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Option<Box<super::super::types::guardduty::DetectorDatasourcesS3Logs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DetectorDatasources {
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
                    "kubernetes",
                    &self.r#kubernetes,
                ),
                to_pulumi_object_field(
                    "malware_protection",
                    &self.r#malware_protection,
                ),
                to_pulumi_object_field(
                    "s_3_logs",
                    &self.r#s_3_logs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DetectorDatasources {
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
                    r#kubernetes: {
                        let field_value = match fields_map.get("kubernetes") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubernetes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#malware_protection: {
                        let field_value = match fields_map.get("malware_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_logs: {
                        let field_value = match fields_map.get("s_3_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
