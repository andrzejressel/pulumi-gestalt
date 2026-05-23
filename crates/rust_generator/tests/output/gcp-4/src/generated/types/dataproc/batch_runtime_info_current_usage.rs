#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchRuntimeInfoCurrentUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) accelerator..
    #[builder(into)]
    #[serde(rename = "milliAccelerator")]
    pub r#milli_accelerator: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs).
    #[builder(into)]
    #[serde(rename = "milliDcu")]
    pub r#milli_dcu: Option<String>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs) charged at premium tier.
    #[builder(into)]
    #[serde(rename = "milliDcuPremium")]
    pub r#milli_dcu_premium: Option<String>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB).
    #[builder(into)]
    #[serde(rename = "shuffleStorageGb")]
    pub r#shuffle_storage_gb: Option<String>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB) charged at premium tier.
    #[builder(into)]
    #[serde(rename = "shuffleStorageGbPremium")]
    pub r#shuffle_storage_gb_premium: Option<String>,
    /// (Output)
    /// The timestamp of the usage snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotTime")]
    pub r#snapshot_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchRuntimeInfoCurrentUsage {
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
                    "acceleratorType",
                    &self.r#accelerator_type,
                ),
                to_pulumi_object_field(
                    "milliAccelerator",
                    &self.r#milli_accelerator,
                ),
                to_pulumi_object_field(
                    "milliDcu",
                    &self.r#milli_dcu,
                ),
                to_pulumi_object_field(
                    "milliDcuPremium",
                    &self.r#milli_dcu_premium,
                ),
                to_pulumi_object_field(
                    "shuffleStorageGb",
                    &self.r#shuffle_storage_gb,
                ),
                to_pulumi_object_field(
                    "shuffleStorageGbPremium",
                    &self.r#shuffle_storage_gb_premium,
                ),
                to_pulumi_object_field(
                    "snapshotTime",
                    &self.r#snapshot_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchRuntimeInfoCurrentUsage {
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
                    r#accelerator_type: {
                        let field_value = match fields_map.get("acceleratorType") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#milli_accelerator: {
                        let field_value = match fields_map.get("milliAccelerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'milliAccelerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#milli_dcu: {
                        let field_value = match fields_map.get("milliDcu") {
                            Some(value) => value,
                            None => bail!("Missing field 'milliDcu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#milli_dcu_premium: {
                        let field_value = match fields_map.get("milliDcuPremium") {
                            Some(value) => value,
                            None => bail!("Missing field 'milliDcuPremium' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shuffle_storage_gb: {
                        let field_value = match fields_map.get("shuffleStorageGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'shuffleStorageGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shuffle_storage_gb_premium: {
                        let field_value = match fields_map.get("shuffleStorageGbPremium") {
                            Some(value) => value,
                            None => bail!("Missing field 'shuffleStorageGbPremium' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_time: {
                        let field_value = match fields_map.get("snapshotTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshotTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
