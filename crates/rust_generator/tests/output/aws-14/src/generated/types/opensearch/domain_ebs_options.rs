#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainEbsOptions {
    /// Whether EBS volumes are attached to data nodes in the domain.
    #[builder(into)]
    #[serde(rename = "ebsEnabled")]
    pub r#ebs_enabled: bool,
    /// Baseline input/output (I/O) performance of EBS volumes attached to data nodes. Applicable only for the GP3 and Provisioned IOPS EBS volume types.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Specifies the throughput (in MiB/s) of the EBS volumes attached to data nodes. Applicable only for the gp3 volume type.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Size of EBS volumes attached to data nodes (in GiB).
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    /// Type of EBS volumes attached to data nodes.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainEbsOptions {
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
                    "ebs_enabled",
                    &self.r#ebs_enabled,
                ),
                to_pulumi_object_field(
                    "iops",
                    &self.r#iops,
                ),
                to_pulumi_object_field(
                    "throughput",
                    &self.r#throughput,
                ),
                to_pulumi_object_field(
                    "volume_size",
                    &self.r#volume_size,
                ),
                to_pulumi_object_field(
                    "volume_type",
                    &self.r#volume_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainEbsOptions {
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
                    r#ebs_enabled: {
                        let field_value = match fields_map.get("ebs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ebs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iops: {
                        let field_value = match fields_map.get("iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#throughput: {
                        let field_value = match fields_map.get("throughput") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_size: {
                        let field_value = match fields_map.get("volume_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_type: {
                        let field_value = match fields_map.get("volume_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
