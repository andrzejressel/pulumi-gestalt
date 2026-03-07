#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderEndpoint {
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "accessanalyzer")]
    pub r#accessanalyzer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "account")]
    pub r#account: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "acm")]
    pub r#acm: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "acmpca")]
    pub r#acmpca: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "amg")]
    pub r#amg: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "amp")]
    pub r#amp: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "amplify")]
    pub r#amplify: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "apigateway")]
    pub r#apigateway: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "apigatewayv2")]
    pub r#apigatewayv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appautoscaling")]
    pub r#appautoscaling: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appconfig")]
    pub r#appconfig: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appfabric")]
    pub r#appfabric: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appflow")]
    pub r#appflow: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appintegrations")]
    pub r#appintegrations: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appintegrationsservice")]
    pub r#appintegrationsservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "applicationautoscaling")]
    pub r#applicationautoscaling: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "applicationinsights")]
    pub r#applicationinsights: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "applicationsignals")]
    pub r#applicationsignals: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appmesh")]
    pub r#appmesh: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appregistry")]
    pub r#appregistry: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "apprunner")]
    pub r#apprunner: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appstream")]
    pub r#appstream: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "appsync")]
    pub r#appsync: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "athena")]
    pub r#athena: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "auditmanager")]
    pub r#auditmanager: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "autoscalingplans")]
    pub r#autoscalingplans: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "backup")]
    pub r#backup: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "batch")]
    pub r#batch: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "bcmdataexports")]
    pub r#bcmdataexports: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "beanstalk")]
    pub r#beanstalk: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "bedrock")]
    pub r#bedrock: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "bedrockagent")]
    pub r#bedrockagent: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "budgets")]
    pub r#budgets: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ce")]
    pub r#ce: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "chatbot")]
    pub r#chatbot: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "chime")]
    pub r#chime: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "chimesdkmediapipelines")]
    pub r#chimesdkmediapipelines: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "chimesdkvoice")]
    pub r#chimesdkvoice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cleanrooms")]
    pub r#cleanrooms: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloud9")]
    pub r#cloud_9: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudcontrol")]
    pub r#cloudcontrol: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudcontrolapi")]
    pub r#cloudcontrolapi: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudformation")]
    pub r#cloudformation: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudfront")]
    pub r#cloudfront: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudfrontkeyvaluestore")]
    pub r#cloudfrontkeyvaluestore: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudhsm")]
    pub r#cloudhsm: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudhsmv2")]
    pub r#cloudhsmv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudsearch")]
    pub r#cloudsearch: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudtrail")]
    pub r#cloudtrail: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatch")]
    pub r#cloudwatch: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchevents")]
    pub r#cloudwatchevents: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchevidently")]
    pub r#cloudwatchevidently: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchlog")]
    pub r#cloudwatchlog: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchlogs")]
    pub r#cloudwatchlogs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchobservabilityaccessmanager")]
    pub r#cloudwatchobservabilityaccessmanager: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cloudwatchrum")]
    pub r#cloudwatchrum: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codeartifact")]
    pub r#codeartifact: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codebuild")]
    pub r#codebuild: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codecatalyst")]
    pub r#codecatalyst: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codecommit")]
    pub r#codecommit: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codeconnections")]
    pub r#codeconnections: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codedeploy")]
    pub r#codedeploy: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codeguruprofiler")]
    pub r#codeguruprofiler: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codegurureviewer")]
    pub r#codegurureviewer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codepipeline")]
    pub r#codepipeline: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codestarconnections")]
    pub r#codestarconnections: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "codestarnotifications")]
    pub r#codestarnotifications: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cognitoidentity")]
    pub r#cognitoidentity: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cognitoidentityprovider")]
    pub r#cognitoidentityprovider: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cognitoidp")]
    pub r#cognitoidp: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "comprehend")]
    pub r#comprehend: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "computeoptimizer")]
    pub r#computeoptimizer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "configservice")]
    pub r#configservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "connect")]
    pub r#connect: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "connectcases")]
    pub r#connectcases: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "controltower")]
    pub r#controltower: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "costandusagereportservice")]
    pub r#costandusagereportservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "costexplorer")]
    pub r#costexplorer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "costoptimizationhub")]
    pub r#costoptimizationhub: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "cur")]
    pub r#cur: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "customerprofiles")]
    pub r#customerprofiles: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "databasemigration")]
    pub r#databasemigration: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "databasemigrationservice")]
    pub r#databasemigrationservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "databrew")]
    pub r#databrew: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "dataexchange")]
    pub r#dataexchange: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "datapipeline")]
    pub r#datapipeline: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "datasync")]
    pub r#datasync: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "datazone")]
    pub r#datazone: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "dax")]
    pub r#dax: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "deploy")]
    pub r#deploy: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "detective")]
    pub r#detective: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "devicefarm")]
    pub r#devicefarm: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "devopsguru")]
    pub r#devopsguru: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "directconnect")]
    pub r#directconnect: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "directoryservice")]
    pub r#directoryservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "dlm")]
    pub r#dlm: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "dms")]
    pub r#dms: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "docdb")]
    pub r#docdb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "docdbelastic")]
    pub r#docdbelastic: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "drs")]
    pub r#drs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ds")]
    pub r#ds: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "dynamodb")]
    pub r#dynamodb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ec2")]
    pub r#ec_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ecr")]
    pub r#ecr: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ecrpublic")]
    pub r#ecrpublic: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ecs")]
    pub r#ecs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "efs")]
    pub r#efs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "eks")]
    pub r#eks: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticache")]
    pub r#elasticache: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticbeanstalk")]
    pub r#elasticbeanstalk: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticloadbalancing")]
    pub r#elasticloadbalancing: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticloadbalancingv2")]
    pub r#elasticloadbalancingv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticsearch")]
    pub r#elasticsearch: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elasticsearchservice")]
    pub r#elasticsearchservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elastictranscoder")]
    pub r#elastictranscoder: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elb")]
    pub r#elb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "elbv2")]
    pub r#elbv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "emr")]
    pub r#emr: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "emrcontainers")]
    pub r#emrcontainers: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "emrserverless")]
    pub r#emrserverless: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "es")]
    pub r#es: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "eventbridge")]
    pub r#eventbridge: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "evidently")]
    pub r#evidently: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "finspace")]
    pub r#finspace: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "fis")]
    pub r#fis: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "fms")]
    pub r#fms: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "fsx")]
    pub r#fsx: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "gamelift")]
    pub r#gamelift: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "glacier")]
    pub r#glacier: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "globalaccelerator")]
    pub r#globalaccelerator: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "glue")]
    pub r#glue: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "gluedatabrew")]
    pub r#gluedatabrew: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "grafana")]
    pub r#grafana: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "greengrass")]
    pub r#greengrass: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "groundstation")]
    pub r#groundstation: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "guardduty")]
    pub r#guardduty: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "healthlake")]
    pub r#healthlake: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "iam")]
    pub r#iam: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "identitystore")]
    pub r#identitystore: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "imagebuilder")]
    pub r#imagebuilder: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "inspector")]
    pub r#inspector: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "inspector2")]
    pub r#inspector_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "inspectorv2")]
    pub r#inspectorv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "internetmonitor")]
    pub r#internetmonitor: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "iot")]
    pub r#iot: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "iotanalytics")]
    pub r#iotanalytics: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "iotevents")]
    pub r#iotevents: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ivs")]
    pub r#ivs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ivschat")]
    pub r#ivschat: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kafka")]
    pub r#kafka: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kafkaconnect")]
    pub r#kafkaconnect: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kendra")]
    pub r#kendra: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "keyspaces")]
    pub r#keyspaces: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kinesis")]
    pub r#kinesis: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kinesisanalytics")]
    pub r#kinesisanalytics: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kinesisanalyticsv2")]
    pub r#kinesisanalyticsv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kinesisvideo")]
    pub r#kinesisvideo: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "kms")]
    pub r#kms: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lakeformation")]
    pub r#lakeformation: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "launchwizard")]
    pub r#launchwizard: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lex")]
    pub r#lex: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lexmodelbuilding")]
    pub r#lexmodelbuilding: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lexmodelbuildingservice")]
    pub r#lexmodelbuildingservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lexmodels")]
    pub r#lexmodels: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lexmodelsv2")]
    pub r#lexmodelsv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lexv2models")]
    pub r#lexv_2_models: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "licensemanager")]
    pub r#licensemanager: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lightsail")]
    pub r#lightsail: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "locationservice")]
    pub r#locationservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "logs")]
    pub r#logs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "lookoutmetrics")]
    pub r#lookoutmetrics: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "m2")]
    pub r#m_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "macie2")]
    pub r#macie_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "managedgrafana")]
    pub r#managedgrafana: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mediaconnect")]
    pub r#mediaconnect: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mediaconvert")]
    pub r#mediaconvert: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "medialive")]
    pub r#medialive: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mediapackage")]
    pub r#mediapackage: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mediapackagev2")]
    pub r#mediapackagev_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mediastore")]
    pub r#mediastore: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "memorydb")]
    pub r#memorydb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mgn")]
    pub r#mgn: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mq")]
    pub r#mq: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "msk")]
    pub r#msk: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "mwaa")]
    pub r#mwaa: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "neptune")]
    pub r#neptune: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "neptunegraph")]
    pub r#neptunegraph: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "networkfirewall")]
    pub r#networkfirewall: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "networkmanager")]
    pub r#networkmanager: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "networkmonitor")]
    pub r#networkmonitor: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "oam")]
    pub r#oam: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "opensearch")]
    pub r#opensearch: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "opensearchingestion")]
    pub r#opensearchingestion: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "opensearchserverless")]
    pub r#opensearchserverless: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "opensearchservice")]
    pub r#opensearchservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "opsworks")]
    pub r#opsworks: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "organizations")]
    pub r#organizations: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "osis")]
    pub r#osis: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "outposts")]
    pub r#outposts: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "paymentcryptography")]
    pub r#paymentcryptography: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pcaconnectorad")]
    pub r#pcaconnectorad: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pcs")]
    pub r#pcs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pinpoint")]
    pub r#pinpoint: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pinpointsmsvoicev2")]
    pub r#pinpointsmsvoicev_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pipes")]
    pub r#pipes: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "polly")]
    pub r#polly: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "pricing")]
    pub r#pricing: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "prometheus")]
    pub r#prometheus: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "prometheusservice")]
    pub r#prometheusservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "qbusiness")]
    pub r#qbusiness: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "qldb")]
    pub r#qldb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "quicksight")]
    pub r#quicksight: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ram")]
    pub r#ram: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "rbin")]
    pub r#rbin: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "rds")]
    pub r#rds: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "recyclebin")]
    pub r#recyclebin: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "redshift")]
    pub r#redshift: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "redshiftdata")]
    pub r#redshiftdata: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "redshiftdataapiservice")]
    pub r#redshiftdataapiservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "redshiftserverless")]
    pub r#redshiftserverless: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "rekognition")]
    pub r#rekognition: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "resiliencehub")]
    pub r#resiliencehub: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "resourceexplorer2")]
    pub r#resourceexplorer_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "resourcegroups")]
    pub r#resourcegroups: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "resourcegroupstagging")]
    pub r#resourcegroupstagging: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "resourcegroupstaggingapi")]
    pub r#resourcegroupstaggingapi: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "rolesanywhere")]
    pub r#rolesanywhere: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53")]
    pub r#route_53: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53domains")]
    pub r#route_53_domains: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53profiles")]
    pub r#route_53_profiles: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53recoverycontrolconfig")]
    pub r#route_53_recoverycontrolconfig: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53recoveryreadiness")]
    pub r#route_53_recoveryreadiness: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "route53resolver")]
    pub r#route_53_resolver: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "rum")]
    pub r#rum: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "s3api")]
    pub r#s_3_api: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "s3control")]
    pub r#s_3_control: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "s3outposts")]
    pub r#s_3_outposts: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "s3tables")]
    pub r#s_3_tables: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sagemaker")]
    pub r#sagemaker: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "scheduler")]
    pub r#scheduler: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "schemas")]
    pub r#schemas: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sdb")]
    pub r#sdb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "secretsmanager")]
    pub r#secretsmanager: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "securityhub")]
    pub r#securityhub: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "securitylake")]
    pub r#securitylake: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "serverlessapplicationrepository")]
    pub r#serverlessapplicationrepository: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "serverlessapprepo")]
    pub r#serverlessapprepo: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "serverlessrepo")]
    pub r#serverlessrepo: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "servicecatalog")]
    pub r#servicecatalog: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "servicecatalogappregistry")]
    pub r#servicecatalogappregistry: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "servicediscovery")]
    pub r#servicediscovery: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "servicequotas")]
    pub r#servicequotas: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ses")]
    pub r#ses: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sesv2")]
    pub r#sesv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sfn")]
    pub r#sfn: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "shield")]
    pub r#shield: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "signer")]
    pub r#signer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "simpledb")]
    pub r#simpledb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sns")]
    pub r#sns: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sqs")]
    pub r#sqs: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssm")]
    pub r#ssm: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssmcontacts")]
    pub r#ssmcontacts: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssmincidents")]
    pub r#ssmincidents: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssmquicksetup")]
    pub r#ssmquicksetup: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssmsap")]
    pub r#ssmsap: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sso")]
    pub r#sso: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "ssoadmin")]
    pub r#ssoadmin: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "stepfunctions")]
    pub r#stepfunctions: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "storagegateway")]
    pub r#storagegateway: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "sts")]
    pub r#sts: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "swf")]
    pub r#swf: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "synthetics")]
    pub r#synthetics: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "taxsettings")]
    pub r#taxsettings: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "timestreaminfluxdb")]
    pub r#timestreaminfluxdb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "timestreamquery")]
    pub r#timestreamquery: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "timestreamwrite")]
    pub r#timestreamwrite: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "transcribe")]
    pub r#transcribe: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "transcribeservice")]
    pub r#transcribeservice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "transfer")]
    pub r#transfer: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "verifiedpermissions")]
    pub r#verifiedpermissions: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "vpclattice")]
    pub r#vpclattice: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "waf")]
    pub r#waf: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "wafregional")]
    pub r#wafregional: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "wafv2")]
    pub r#wafv_2: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "wellarchitected")]
    pub r#wellarchitected: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "worklink")]
    pub r#worklink: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "workspaces")]
    pub r#workspaces: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "workspacesweb")]
    pub r#workspacesweb: Option<String>,
    /// Use this to override the default service endpoint URL
    #[builder(into)]
    #[serde(rename = "xray")]
    pub r#xray: Option<String>,
}
