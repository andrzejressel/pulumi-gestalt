#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostingVersionConfig {
    /// An array of objects, where each object specifies a URL pattern that, if matched to the request URL path,
    /// triggers Hosting to apply the specified custom response headers.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::firebase::HostingVersionConfigHeader>>,
    /// An array of objects (called redirect rules), where each rule specifies a URL pattern that, if matched to the request URL path,
    /// triggers Hosting to respond with a redirect to the specified destination path.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redirects")]
    pub r#redirects: Option<Vec<super::super::types::firebase::HostingVersionConfigRedirect>>,
    /// An array of objects (called rewrite rules), where each rule specifies a URL pattern that, if matched to the
    /// request URL path, triggers Hosting to respond as if the service were given the specified destination URL.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rewrites")]
    pub r#rewrites: Option<Vec<super::super::types::firebase::HostingVersionConfigRewrite>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HostingVersionConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "redirects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirects,
                )
                .await,
            );
            map.insert(
                "rewrites".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rewrites,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HostingVersionConfig {
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
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirects: {
                        let field_value = match fields_map.get("redirects") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rewrites: {
                        let field_value = match fields_map.get("rewrites") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewrites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
