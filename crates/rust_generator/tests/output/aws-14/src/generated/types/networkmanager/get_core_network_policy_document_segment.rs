#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegment {
    /// List of strings of segment names that explicitly allows only routes from the segments that are listed in the array. Use the `allow_filter` setting if a segment has a well-defined group of other segments that connectivity should be restricted to. It is applied after routes have been shared in `segment_actions`. If a segment is listed in `allow_filter`, attachments between the two segments will have routes if they are also shared in the segment-actions area. For example, you might have a segment named "video-producer" that should only ever share routes with a "video-distributor" segment, no matter how many other share statements are created.
    #[builder(into)]
    #[serde(rename = "allowFilters")]
    pub r#allow_filters: Option<Vec<String>>,
    /// An array of segments that disallows routes from the segments listed in the array. It is applied only after routes have been shared in `segment_actions`. If a segment is listed in the `deny_filter`, attachments between the two segments will never have routes shared across them. For example, you might have a "financial" payment segment that should never share routes with a "development" segment, regardless of how many other share statements are created. Adding the payments segment to the deny-filter parameter prevents any shared routes from being created with other segments.
    #[builder(into)]
    #[serde(rename = "denyFilters")]
    pub r#deny_filters: Option<Vec<String>>,
    /// A user-defined string describing the segment.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A list of strings of AWS Region names. Allows you to define a more restrictive set of Regions for a segment. The edge location must be a subset of the locations that are defined for `edge_locations` in the `core_network_configuration`.
    #[builder(into)]
    #[serde(rename = "edgeLocations")]
    pub r#edge_locations: Option<Vec<String>>,
    /// This Boolean setting determines whether attachments on the same segment can communicate with each other. If set to `true`, the only routes available will be either shared routes through the share actions, which are attachments in other segments, or static routes. The default value is `false`. For example, you might have a segment dedicated to "development" that should never allow VPCs to talk to each other, even if they’re on the same segment. In this example, you would keep the default parameter of `false`.
    #[builder(into)]
    #[serde(rename = "isolateAttachments")]
    pub r#isolate_attachments: Option<bool>,
    /// Unique name for a segment. The name is a string used in other parts of the policy document, as well as in the console for metrics and other reference points. Valid characters are a–z, and 0–9.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// This Boolean setting determines whether attachment requests are automatically approved or require acceptance. The default is `true`, indicating that attachment requests require acceptance. For example, you might use this setting to allow a "sandbox" segment to allow any attachment request so that a core network or attachment administrator does not need to review and approve attachment requests. In this example, `require_attachment_acceptance` is set to `false`.
    #[builder(into)]
    #[serde(rename = "requireAttachmentAcceptance")]
    pub r#require_attachment_acceptance: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentSegment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_filters".to_string(), self.r#allow_filters.to_pulumi_value().await);
            map.insert("deny_filters".to_string(), self.r#deny_filters.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("edge_locations".to_string(), self.r#edge_locations.to_pulumi_value().await);
            map.insert("isolate_attachments".to_string(), self.r#isolate_attachments.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("require_attachment_acceptance".to_string(), self.r#require_attachment_acceptance.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentSegment {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#allow_filters: {
                        let field_value = match fields_map.get("allow_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#deny_filters: {
                        let field_value = match fields_map.get("deny_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'deny_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#edge_locations: {
                        let field_value = match fields_map.get("edge_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#isolate_attachments: {
                        let field_value = match fields_map.get("isolate_attachments") {
                            Some(value) => value,
                            None => bail!("Missing field 'isolate_attachments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_attachment_acceptance: {
                        let field_value = match fields_map.get("require_attachment_acceptance") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_attachment_acceptance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
