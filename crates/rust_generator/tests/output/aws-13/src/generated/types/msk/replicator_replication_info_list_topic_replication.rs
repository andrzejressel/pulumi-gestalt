#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatorReplicationInfoListTopicReplication {
    /// Whether to periodically configure remote topic ACLs to match their corresponding upstream topics.
    #[builder(into)]
    #[serde(rename = "copyAccessControlListsForTopics")]
    pub r#copy_access_control_lists_for_topics: Option<bool>,
    /// Whether to periodically configure remote topics to match their corresponding upstream topics.
    #[builder(into)]
    #[serde(rename = "copyTopicConfigurations")]
    pub r#copy_topic_configurations: Option<bool>,
    /// Whether to periodically check for new topics and partitions.
    #[builder(into)]
    #[serde(rename = "detectAndCopyNewTopics")]
    pub r#detect_and_copy_new_topics: Option<bool>,
    /// Configuration for specifying the position in the topics to start replicating from.
    #[builder(into)]
    #[serde(rename = "startingPosition")]
    pub r#starting_position: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationStartingPosition>>,
    #[builder(into)]
    #[serde(rename = "topicNameConfiguration")]
    pub r#topic_name_configuration: Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration>>,
    /// List of regular expression patterns indicating the topics that should not be replica.
    #[builder(into)]
    #[serde(rename = "topicsToExcludes")]
    pub r#topics_to_excludes: Option<Vec<String>>,
    /// List of regular expression patterns indicating the topics to copy.
    #[builder(into)]
    #[serde(rename = "topicsToReplicates")]
    pub r#topics_to_replicates: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatorReplicationInfoListTopicReplication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("copy_access_control_lists_for_topics".to_string(), self.r#copy_access_control_lists_for_topics.to_pulumi_value().await);
            map.insert("copy_topic_configurations".to_string(), self.r#copy_topic_configurations.to_pulumi_value().await);
            map.insert("detect_and_copy_new_topics".to_string(), self.r#detect_and_copy_new_topics.to_pulumi_value().await);
            map.insert("starting_position".to_string(), self.r#starting_position.to_pulumi_value().await);
            map.insert("topic_name_configuration".to_string(), self.r#topic_name_configuration.to_pulumi_value().await);
            map.insert("topics_to_excludes".to_string(), self.r#topics_to_excludes.to_pulumi_value().await);
            map.insert("topics_to_replicates".to_string(), self.r#topics_to_replicates.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatorReplicationInfoListTopicReplication {
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
                    r#copy_access_control_lists_for_topics: {
                        let field_value = match fields_map.get("copy_access_control_lists_for_topics") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_access_control_lists_for_topics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#copy_topic_configurations: {
                        let field_value = match fields_map.get("copy_topic_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_topic_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#detect_and_copy_new_topics: {
                        let field_value = match fields_map.get("detect_and_copy_new_topics") {
                            Some(value) => value,
                            None => bail!("Missing field 'detect_and_copy_new_topics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#starting_position: {
                        let field_value = match fields_map.get("starting_position") {
                            Some(value) => value,
                            None => bail!("Missing field 'starting_position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationStartingPosition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#topic_name_configuration: {
                        let field_value = match fields_map.get("topic_name_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_name_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ReplicatorReplicationInfoListTopicReplicationTopicNameConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#topics_to_excludes: {
                        let field_value = match fields_map.get("topics_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'topics_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#topics_to_replicates: {
                        let field_value = match fields_map.get("topics_to_replicates") {
                            Some(value) => value,
                            None => bail!("Missing field 'topics_to_replicates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
