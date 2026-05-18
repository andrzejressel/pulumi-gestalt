#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PerInstanceConfigPreservedState {
    /// Stateful disks for the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "disks")]
    pub r#disks: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateDisk>>,
    /// Preserved external IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "externalIps")]
    pub r#external_ips: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateExternalIp>>,
    /// Preserved internal IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "internalIps")]
    pub r#internal_ips: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateInternalIp>>,
    /// Preserved metadata defined for this instance. This is a list of key->value pairs.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PerInstanceConfigPreservedState {
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
                    "disks",
                    &self.r#disks,
                ),
                to_pulumi_object_field(
                    "external_ips",
                    &self.r#external_ips,
                ),
                to_pulumi_object_field(
                    "internal_ips",
                    &self.r#internal_ips,
                ),
                to_pulumi_object_field(
                    "metadata",
                    &self.r#metadata,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PerInstanceConfigPreservedState {
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
                    r#disks: {
                        let field_value = match fields_map.get("disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_ips: {
                        let field_value = match fields_map.get("external_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internal_ips: {
                        let field_value = match fields_map.get("internal_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'internal_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
