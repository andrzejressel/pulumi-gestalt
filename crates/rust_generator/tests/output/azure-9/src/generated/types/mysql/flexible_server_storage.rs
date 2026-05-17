#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleServerStorage {
    /// Should Storage Auto Grow be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: Option<bool>,
    /// Should IOPS be scaled automatically? If `true`, `iops` can not be set. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: Option<bool>,
    /// The storage IOPS for the MySQL Flexible Server. Possible values are between `360` and `20000`.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// The max storage allowed for the MySQL Flexible Server. Possible values are between `20` and `16384`.
    /// 
    /// > **Note:** Decreasing `size_gb` forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleServerStorage {
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
                    "auto_grow_enabled",
                    &self.r#auto_grow_enabled,
                ),
                to_pulumi_object_field(
                    "io_scaling_enabled",
                    &self.r#io_scaling_enabled,
                ),
                to_pulumi_object_field(
                    "iops",
                    &self.r#iops,
                ),
                to_pulumi_object_field(
                    "size_gb",
                    &self.r#size_gb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleServerStorage {
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
                    r#auto_grow_enabled: {
                        let field_value = match fields_map.get("auto_grow_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_grow_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#io_scaling_enabled: {
                        let field_value = match fields_map.get("io_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'io_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#size_gb: {
                        let field_value = match fields_map.get("size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
