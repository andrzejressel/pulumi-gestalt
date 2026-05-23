#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatorReplicationInfoListTopicReplication {
    /// Whether to periodically configure remote topic ACLs to match their corresponding upstream topics.
    #[builder(into)]
    pub r#copy_access_control_lists_for_topics: Option<bool>,
    /// Whether to periodically configure remote topics to match their corresponding upstream topics.
    #[builder(into)]
    pub r#copy_topic_configurations: Option<bool>,
    /// Whether to periodically check for new topics and partitions.
    #[builder(into)]
    pub r#detect_and_copy_new_topics: Option<bool>,
    /// Configuration for specifying the position in the topics to start replicating from.
    #[builder(into)]
    pub r#starting_position: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationStartingPosition>>,
    #[builder(into)]
    pub r#topic_name_configuration: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration>>,
    /// List of regular expression patterns indicating the topics that should not be replica.
    #[builder(into)]
    pub r#topics_to_excludes: Option<Vec<String>>,
    /// List of regular expression patterns indicating the topics to copy.
    #[builder(into)]
    pub r#topics_to_replicates: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatorReplicationInfoListTopicReplication {
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
                    "copyAccessControlListsForTopics",
                    &self.r#copy_access_control_lists_for_topics,
                ),
                to_pulumi_object_field(
                    "copyTopicConfigurations",
                    &self.r#copy_topic_configurations,
                ),
                to_pulumi_object_field(
                    "detectAndCopyNewTopics",
                    &self.r#detect_and_copy_new_topics,
                ),
                to_pulumi_object_field(
                    "startingPosition",
                    &self.r#starting_position,
                ),
                to_pulumi_object_field(
                    "topicNameConfiguration",
                    &self.r#topic_name_configuration,
                ),
                to_pulumi_object_field(
                    "topicsToExcludes",
                    &self.r#topics_to_excludes,
                ),
                to_pulumi_object_field(
                    "topicsToReplicates",
                    &self.r#topics_to_replicates,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatorReplicationInfoListTopicReplication {
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
                    r#copy_access_control_lists_for_topics: {
                        let field_value = match fields_map.get("copyAccessControlListsForTopics") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyAccessControlListsForTopics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copy_topic_configurations: {
                        let field_value = match fields_map.get("copyTopicConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyTopicConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detect_and_copy_new_topics: {
                        let field_value = match fields_map.get("detectAndCopyNewTopics") {
                            Some(value) => value,
                            None => bail!("Missing field 'detectAndCopyNewTopics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#starting_position: {
                        let field_value = match fields_map.get("startingPosition") {
                            Some(value) => value,
                            None => bail!("Missing field 'startingPosition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_name_configuration: {
                        let field_value = match fields_map.get("topicNameConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'topicNameConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topics_to_excludes: {
                        let field_value = match fields_map.get("topicsToExcludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'topicsToExcludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topics_to_replicates: {
                        let field_value = match fields_map.get("topicsToReplicates") {
                            Some(value) => value,
                            None => bail!("Missing field 'topicsToReplicates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
