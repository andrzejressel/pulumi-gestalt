#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceS3Import {
    /// The bucket name where your backup is stored
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// Can be blank, but is the path to your backup
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Option<String>,
    /// Role applied to load the data.
    #[builder(into)]
    #[serde(rename = "ingestionRole")]
    pub r#ingestion_role: String,
    /// Source engine for the backup
    #[builder(into)]
    #[serde(rename = "sourceEngine")]
    pub r#source_engine: String,
    /// Version of the source engine used to make the backup
    /// 
    /// This will not recreate the resource if the S3 object changes in some way.  It's only used to initialize the database.
    #[builder(into)]
    #[serde(rename = "sourceEngineVersion")]
    pub r#source_engine_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceS3Import {
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
                    "bucketName",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "bucketPrefix",
                    &self.r#bucket_prefix,
                ),
                to_pulumi_object_field(
                    "ingestionRole",
                    &self.r#ingestion_role,
                ),
                to_pulumi_object_field(
                    "sourceEngine",
                    &self.r#source_engine,
                ),
                to_pulumi_object_field(
                    "sourceEngineVersion",
                    &self.r#source_engine_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceS3Import {
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
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucketName") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_prefix: {
                        let field_value = match fields_map.get("bucketPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingestion_role: {
                        let field_value = match fields_map.get("ingestionRole") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingestionRole' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_engine: {
                        let field_value = match fields_map.get("sourceEngine") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceEngine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_engine_version: {
                        let field_value = match fields_map.get("sourceEngineVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceEngineVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
