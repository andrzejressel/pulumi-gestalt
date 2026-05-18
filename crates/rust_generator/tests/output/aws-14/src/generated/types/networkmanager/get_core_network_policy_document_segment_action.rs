#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegmentAction {
    /// Action to take for the chosen segment. Valid values: `create-route`, `share`, `send-via` and `send-to`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// A user-defined string describing the segment action.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// List of strings containing CIDRs. You can define the IPv4 and IPv6 CIDR notation for each AWS Region. For example, `10.1.0.0/16` or `2001:db8::/56`. This is an array of CIDR notation strings.
    #[builder(into)]
    #[serde(rename = "destinationCidrBlocks")]
    pub r#destination_cidr_blocks: Option<Vec<String>>,
    /// A list of strings. Valid values include `["blackhole"]` or a list of attachment ids.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<String>>,
    /// String. When `action` is `share`, a `mode` value of `attachment-route` places the attachment and return routes in each of the `share_with` segments. When `action` is `send-via`, indicates the mode used for packets. Valid values: `attachment-route`, `single-hop`, `dual-hop`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Name of the segment.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: String,
    /// A set subtraction of segments to not share with.
    #[builder(into)]
    #[serde(rename = "shareWithExcepts")]
    pub r#share_with_excepts: Option<Vec<String>>,
    /// A list of strings to share with. Must be a substring is all segments. Valid values include: `["*"]` or `["<segment-names>"]`.
    #[builder(into)]
    #[serde(rename = "shareWiths")]
    pub r#share_withs: Option<Vec<String>>,
    /// The network function groups and any edge overrides associated with the action.
    #[builder(into)]
    #[serde(rename = "via")]
    pub r#via: Option<Box<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionVia>>,
    /// The destination segments for the `send-via` or `send-to` `action`.
    #[builder(into)]
    #[serde(rename = "whenSentTo")]
    pub r#when_sent_to: Option<Box<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionWhenSentTo>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentSegmentAction {
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
                    "action",
                    &self.r#action,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "destination_cidr_blocks",
                    &self.r#destination_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "destinations",
                    &self.r#destinations,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "segment",
                    &self.r#segment,
                ),
                to_pulumi_object_field(
                    "share_with_excepts",
                    &self.r#share_with_excepts,
                ),
                to_pulumi_object_field(
                    "share_withs",
                    &self.r#share_withs,
                ),
                to_pulumi_object_field(
                    "via",
                    &self.r#via,
                ),
                to_pulumi_object_field(
                    "when_sent_to",
                    &self.r#when_sent_to,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentSegmentAction {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_cidr_blocks: {
                        let field_value = match fields_map.get("destination_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destinations: {
                        let field_value = match fields_map.get("destinations") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#segment: {
                        let field_value = match fields_map.get("segment") {
                            Some(value) => value,
                            None => bail!("Missing field 'segment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_with_excepts: {
                        let field_value = match fields_map.get("share_with_excepts") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_with_excepts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_withs: {
                        let field_value = match fields_map.get("share_withs") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_withs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#via: {
                        let field_value = match fields_map.get("via") {
                            Some(value) => value,
                            None => bail!("Missing field 'via' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#when_sent_to: {
                        let field_value = match fields_map.get("when_sent_to") {
                            Some(value) => value,
                            None => bail!("Missing field 'when_sent_to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
