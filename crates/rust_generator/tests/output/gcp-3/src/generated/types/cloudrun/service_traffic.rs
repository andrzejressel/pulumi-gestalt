#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTraffic {
    /// LatestRevision may be optionally provided to indicate that the latest ready
    /// Revision of the Configuration should be used for this traffic target. When
    /// provided LatestRevision must be true if RevisionName is empty; it must be
    /// false when RevisionName is non-empty.
    #[builder(into)]
    #[serde(rename = "latestRevision")]
    pub r#latest_revision: Option<bool>,
    /// Percent specifies percent of the traffic to this Revision or Configuration.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: i32,
    /// RevisionName of a specific revision to which to send this portion of traffic.
    #[builder(into)]
    #[serde(rename = "revisionName")]
    pub r#revision_name: Option<String>,
    /// Tag is optionally used to expose a dedicated url for referencing this target exclusively.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    /// (Output)
    /// URL displays the URL for accessing tagged traffic targets. URL is displayed in status,
    /// and is disallowed on spec. URL must contain a scheme (e.g. http://) and a hostname,
    /// but may not contain anything else (e.g. basic auth, url path, etc.)
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTraffic {
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
                "latest_revision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#latest_revision,
                )
                .await,
            );
            map.insert(
                "percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#percent,
                )
                .await,
            );
            map.insert(
                "revision_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revision_name,
                )
                .await,
            );
            map.insert(
                "tag".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag,
                )
                .await,
            );
            map.insert(
                "url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTraffic {
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
                    r#latest_revision: {
                        let field_value = match fields_map.get("latest_revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#percent: {
                        let field_value = match fields_map.get("percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revision_name: {
                        let field_value = match fields_map.get("revision_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'revision_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag: {
                        let field_value = match fields_map.get("tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
