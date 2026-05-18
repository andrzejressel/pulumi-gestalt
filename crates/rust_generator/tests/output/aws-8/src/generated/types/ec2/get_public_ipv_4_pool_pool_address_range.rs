#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPublicIpv4PoolPoolAddressRange {
    /// Number of addresses in the range.
    #[builder(into)]
    #[serde(rename = "addressCount")]
    pub r#address_count: i32,
    /// Number of available addresses in the range.
    #[builder(into)]
    #[serde(rename = "availableAddressCount")]
    pub r#available_address_count: i32,
    /// First address in the range.
    #[builder(into)]
    #[serde(rename = "firstAddress")]
    pub r#first_address: String,
    /// Last address in the range.
    #[builder(into)]
    #[serde(rename = "lastAddress")]
    pub r#last_address: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPublicIpv4PoolPoolAddressRange {
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
                    "address_count",
                    &self.r#address_count,
                ),
                to_pulumi_object_field(
                    "available_address_count",
                    &self.r#available_address_count,
                ),
                to_pulumi_object_field(
                    "first_address",
                    &self.r#first_address,
                ),
                to_pulumi_object_field(
                    "last_address",
                    &self.r#last_address,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPublicIpv4PoolPoolAddressRange {
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
                    r#address_count: {
                        let field_value = match fields_map.get("address_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_address_count: {
                        let field_value = match fields_map.get("available_address_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_address_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#first_address: {
                        let field_value = match fields_map.get("first_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_address: {
                        let field_value = match fields_map.get("last_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
