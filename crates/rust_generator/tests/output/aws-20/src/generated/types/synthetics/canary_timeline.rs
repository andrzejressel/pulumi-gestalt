#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CanaryTimeline {
    /// Date and time the canary was created.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// Date and time the canary was most recently modified.
    #[builder(into)]
    #[serde(rename = "lastModified")]
    pub r#last_modified: Option<String>,
    /// Date and time that the canary's most recent run started.
    #[builder(into)]
    #[serde(rename = "lastStarted")]
    pub r#last_started: Option<String>,
    /// Date and time that the canary's most recent run ended.
    #[builder(into)]
    #[serde(rename = "lastStopped")]
    pub r#last_stopped: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CanaryTimeline {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("created".to_string(), self.r#created.to_pulumi_value().await);
            map.insert("last_modified".to_string(), self.r#last_modified.to_pulumi_value().await);
            map.insert("last_started".to_string(), self.r#last_started.to_pulumi_value().await);
            map.insert("last_stopped".to_string(), self.r#last_stopped.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CanaryTimeline {
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
                    r#created: {
                        let field_value = match fields_map.get("created") {
                            Some(value) => value,
                            None => bail!("Missing field 'created' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_modified: {
                        let field_value = match fields_map.get("last_modified") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_started: {
                        let field_value = match fields_map.get("last_started") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_started' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_stopped: {
                        let field_value = match fields_map.get("last_stopped") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_stopped' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
