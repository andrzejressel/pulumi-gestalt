#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInterfaceAttachment {
    #[builder(into)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: String,
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: i32,
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: String,
    #[builder(into)]
    #[serde(rename = "instanceOwnerId")]
    pub r#instance_owner_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInterfaceAttachment {
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
                    "attachmentId",
                    &self.r#attachment_id,
                ),
                to_pulumi_object_field(
                    "deviceIndex",
                    &self.r#device_index,
                ),
                to_pulumi_object_field(
                    "instanceId",
                    &self.r#instance_id,
                ),
                to_pulumi_object_field(
                    "instanceOwnerId",
                    &self.r#instance_owner_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInterfaceAttachment {
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
                    r#attachment_id: {
                        let field_value = match fields_map.get("attachmentId") {
                            Some(value) => value,
                            None => bail!("Missing field 'attachmentId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_index: {
                        let field_value = match fields_map.get("deviceIndex") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceIndex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_id: {
                        let field_value = match fields_map.get("instanceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_owner_id: {
                        let field_value = match fields_map.get("instanceOwnerId") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceOwnerId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
