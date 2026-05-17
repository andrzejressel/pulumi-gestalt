#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLifecyclePolicyDocumentRuleSelection {
    /// Specify a count number. If the `count_type` used is "imageCountMoreThan", then the value is the maximum number of images that you want to retain in your repository. If the `count_type` used is "sinceImagePushed", then the value is the maximum age limit for your images.
    #[builder(into)]
    #[serde(rename = "countNumber")]
    pub r#count_number: i32,
    /// Specify a count type to apply to the images. If `count_type` is set to "imageCountMoreThan", you also specify `count_number` to create a rule that sets a limit on the number of images that exist in your repository. If `count_type` is set to "sinceImagePushed", you also specify `count_unit` and `count_number` to specify a time limit on the images that exist in your repository.
    #[builder(into)]
    #[serde(rename = "countType")]
    pub r#count_type: String,
    /// Specify a count unit of days to indicate that as the unit of time, in addition to `count_number`, which is the number of days.
    #[builder(into)]
    #[serde(rename = "countUnit")]
    pub r#count_unit: Option<String>,
    /// You must specify a comma-separated list of image tag patterns that may contain wildcards (\*) on which to take action with your lifecycle policy. For example, if your images are tagged as `prod`, `prod1`, `prod2`, and so on, you would use the tag pattern list `["prod\*"]` to specify all of them. If you specify multiple tags, only the images with all specified tags are selected. There is a maximum limit of four wildcards (\*) per string. For example, `["*test*1*2*3", "test*1*2*3*"]` is valid but `["test*1*2*3*4*5*6"]` is invalid.
    #[builder(into)]
    #[serde(rename = "tagPatternLists")]
    pub r#tag_pattern_lists: Option<Vec<String>>,
    /// You must specify a comma-separated list of image tag prefixes on which to take action with your lifecycle policy. For example, if your images are tagged as `prod`, `prod1`, `prod2`, and so on, you would use the tag prefix "prod" to specify all of them. If you specify multiple tags, only images with all specified tags are selected.
    #[builder(into)]
    #[serde(rename = "tagPrefixLists")]
    pub r#tag_prefix_lists: Option<Vec<String>>,
    /// Determines whether the lifecycle policy rule that you are adding specifies a tag for an image. Acceptable options are "tagged", "untagged", or "any". If you specify "any", then all images have the rule applied to them. If you specify "tagged", then you must also specify a `tag_prefix_list` value. If you specify "untagged", then you must omit `tag_prefix_list`.
    #[builder(into)]
    #[serde(rename = "tagStatus")]
    pub r#tag_status: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLifecyclePolicyDocumentRuleSelection {
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
                "count_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count_number,
                )
                .await,
            );
            map.insert(
                "count_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count_type,
                )
                .await,
            );
            map.insert(
                "count_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count_unit,
                )
                .await,
            );
            map.insert(
                "tag_pattern_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_pattern_lists,
                )
                .await,
            );
            map.insert(
                "tag_prefix_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_prefix_lists,
                )
                .await,
            );
            map.insert(
                "tag_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_status,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLifecyclePolicyDocumentRuleSelection {
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
                    r#count_number: {
                        let field_value = match fields_map.get("count_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'count_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#count_type: {
                        let field_value = match fields_map.get("count_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'count_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#count_unit: {
                        let field_value = match fields_map.get("count_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'count_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_pattern_lists: {
                        let field_value = match fields_map.get("tag_pattern_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_pattern_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_prefix_lists: {
                        let field_value = match fields_map.get("tag_prefix_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_prefix_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_status: {
                        let field_value = match fields_map.get("tag_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
