#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SnapshotImportClientData {
    /// A user-defined comment about the disk upload.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Option<String>,
    /// The time that the disk upload ends.
    #[builder(into)]
    #[serde(rename = "uploadEnd")]
    pub r#upload_end: Option<String>,
    /// The size of the uploaded disk image, in GiB.
    #[builder(into)]
    #[serde(rename = "uploadSize")]
    pub r#upload_size: Option<f64>,
    /// The time that the disk upload starts.
    #[builder(into)]
    #[serde(rename = "uploadStart")]
    pub r#upload_start: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SnapshotImportClientData {
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
                "comment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#comment,
                )
                .await,
            );
            map.insert(
                "upload_end".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upload_end,
                )
                .await,
            );
            map.insert(
                "upload_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upload_size,
                )
                .await,
            );
            map.insert(
                "upload_start".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upload_start,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SnapshotImportClientData {
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
                    r#comment: {
                        let field_value = match fields_map.get("comment") {
                            Some(value) => value,
                            None => bail!("Missing field 'comment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_end: {
                        let field_value = match fields_map.get("upload_end") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_end' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_size: {
                        let field_value = match fields_map.get("upload_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_start: {
                        let field_value = match fields_map.get("upload_start") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
