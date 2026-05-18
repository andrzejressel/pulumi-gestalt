#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxDataviewSegmentConfiguration {
    /// The database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume.
    #[builder(into)]
    #[serde(rename = "dbPaths")]
    pub r#db_paths: Vec<String>,
    /// Enables on-demand caching on the selected database path when a particular file or a column of the database is accessed. When on demand caching is **True**, dataviews perform minimal loading of files on the filesystem as needed. When it is set to **False**, everything is cached. The default value is **False**.
    #[builder(into)]
    #[serde(rename = "onDemand")]
    pub r#on_demand: Option<bool>,
    /// The name of the volume that you want to attach to a dataview. This volume must be in the same availability zone as the dataview that you are attaching to.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KxDataviewSegmentConfiguration {
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
                    "db_paths",
                    &self.r#db_paths,
                ),
                to_pulumi_object_field(
                    "on_demand",
                    &self.r#on_demand,
                ),
                to_pulumi_object_field(
                    "volume_name",
                    &self.r#volume_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KxDataviewSegmentConfiguration {
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
                    r#db_paths: {
                        let field_value = match fields_map.get("db_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand: {
                        let field_value = match fields_map.get("on_demand") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_name: {
                        let field_value = match fields_map.get("volume_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
