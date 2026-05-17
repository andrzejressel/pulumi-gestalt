#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResponseHeadersPolicySecurityHeadersConfigXssProtection {
    /// Whether CloudFront includes the `mode=block` directive in the `X-XSS-Protection` header.
    #[builder(into)]
    #[serde(rename = "modeBlock")]
    pub r#mode_block: Option<bool>,
    /// Whether CloudFront overrides the `X-XSS-Protection` HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: bool,
    /// A Boolean value that determines the value of the `X-XSS-Protection` HTTP response header. When this setting is `true`, the value of the `X-XSS-Protection` header is `1`. When this setting is `false`, the value of the `X-XSS-Protection` header is `0`.
    #[builder(into)]
    #[serde(rename = "protection")]
    pub r#protection: bool,
    /// A reporting URI, which CloudFront uses as the value of the report directive in the `X-XSS-Protection` header. You cannot specify a `report_uri` when `mode_block` is `true`.
    #[builder(into)]
    #[serde(rename = "reportUri")]
    pub r#report_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResponseHeadersPolicySecurityHeadersConfigXssProtection {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "mode_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode_block,
                )
                .await,
            );
            map.insert(
                "override_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#override_,
                )
                .await,
            );
            map.insert(
                "protection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protection,
                )
                .await,
            );
            map.insert(
                "report_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#report_uri,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResponseHeadersPolicySecurityHeadersConfigXssProtection {
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
                    r#mode_block: {
                        let field_value = match fields_map.get("mode_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#override_: {
                        let field_value = match fields_map.get("override_") {
                            Some(value) => value,
                            None => bail!("Missing field 'override_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protection: {
                        let field_value = match fields_map.get("protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#report_uri: {
                        let field_value = match fields_map.get("report_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'report_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
