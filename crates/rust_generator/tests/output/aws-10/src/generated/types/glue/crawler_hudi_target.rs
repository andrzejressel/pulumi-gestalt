#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerHudiTarget {
    /// The name of the connection to use to connect to the Hudi target.
    #[builder(into)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Option<String>,
    /// A list of glob patterns used to exclude from the crawl.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<String>>,
    /// The maximum depth of Amazon S3 paths that the crawler can traverse to discover the Hudi metadata folder in your Amazon S3 path. Used to limit the crawler run time. Valid values are between `1` and `20`.
    #[builder(into)]
    #[serde(rename = "maximumTraversalDepth")]
    pub r#maximum_traversal_depth: i32,
    /// One or more Amazon S3 paths that contains Hudi metadata folders as s3://bucket/prefix.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CrawlerHudiTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connection_name".to_string(), self.r#connection_name.to_pulumi_value().await);
            map.insert("exclusions".to_string(), self.r#exclusions.to_pulumi_value().await);
            map.insert("maximum_traversal_depth".to_string(), self.r#maximum_traversal_depth.to_pulumi_value().await);
            map.insert("paths".to_string(), self.r#paths.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CrawlerHudiTarget {
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
                    r#connection_name: {
                        let field_value = match fields_map.get("connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#exclusions: {
                        let field_value = match fields_map.get("exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maximum_traversal_depth: {
                        let field_value = match fields_map.get("maximum_traversal_depth") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_traversal_depth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#paths: {
                        let field_value = match fields_map.get("paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
