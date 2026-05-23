#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorOriginPrivateLink {
    /// Specifies the location where the Private Link resource should exist. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#location: String,
    /// The ID of the Azure Resource to connect to via the Private Link.
    /// 
    /// > **Note:** the `private_link_target_id` property must specify the Resource ID of the Private Link Service when using Load Balancer as an Origin.
    #[builder(into)]
    pub r#private_link_target_id: String,
    /// Specifies the request message that will be submitted to the `private_link_target_id` when requesting the private link endpoint connection. Values must be between `1` and `140` characters in length. Defaults to `Access request for CDN FrontDoor Private Link Origin`.
    #[builder(into)]
    pub r#request_message: Option<String>,
    /// Specifies the type of target for this Private Link Endpoint. Possible values are `blob`, `blob_secondary`, `web` and `sites`.
    /// 
    /// > **NOTE:** `target_type` cannot be specified when using a Load Balancer as an Origin.
    #[builder(into)]
    pub r#target_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorOriginPrivateLink {
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
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "privateLinkTargetId",
                    &self.r#private_link_target_id,
                ),
                to_pulumi_object_field(
                    "requestMessage",
                    &self.r#request_message,
                ),
                to_pulumi_object_field(
                    "targetType",
                    &self.r#target_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorOriginPrivateLink {
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
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_link_target_id: {
                        let field_value = match fields_map.get("privateLinkTargetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateLinkTargetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_message: {
                        let field_value = match fields_map.get("requestMessage") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestMessage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_type: {
                        let field_value = match fields_map.get("targetType") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
