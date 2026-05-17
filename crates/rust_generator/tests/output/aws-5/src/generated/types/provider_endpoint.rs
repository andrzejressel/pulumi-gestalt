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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "accessanalyzer",
                    &self.r#accessanalyzer,
                ),
                to_pulumi_object_field(
                    "account",
                    &self.r#account,
                ),
                to_pulumi_object_field(
                    "acm",
                    &self.r#acm,
                ),
                to_pulumi_object_field(
                    "acmpca",
                    &self.r#acmpca,
                ),
                to_pulumi_object_field(
                    "amg",
                    &self.r#amg,
                ),
                to_pulumi_object_field(
                    "amp",
                    &self.r#amp,
                ),
                to_pulumi_object_field(
                    "amplify",
                    &self.r#amplify,
                ),
                to_pulumi_object_field(
                    "apigateway",
                    &self.r#apigateway,
                ),
                to_pulumi_object_field(
                    "apigatewayv_2",
                    &self.r#apigatewayv_2,
                ),
                to_pulumi_object_field(
                    "appautoscaling",
                    &self.r#appautoscaling,
                ),
                to_pulumi_object_field(
                    "appconfig",
                    &self.r#appconfig,
                ),
                to_pulumi_object_field(
                    "appfabric",
                    &self.r#appfabric,
                ),
                to_pulumi_object_field(
                    "appflow",
                    &self.r#appflow,
                ),
                to_pulumi_object_field(
                    "appintegrations",
                    &self.r#appintegrations,
                ),
                to_pulumi_object_field(
                    "appintegrationsservice",
                    &self.r#appintegrationsservice,
                ),
                to_pulumi_object_field(
                    "applicationautoscaling",
                    &self.r#applicationautoscaling,
                ),
                to_pulumi_object_field(
                    "applicationinsights",
                    &self.r#applicationinsights,
                ),
                to_pulumi_object_field(
                    "applicationsignals",
                    &self.r#applicationsignals,
                ),
                to_pulumi_object_field(
                    "appmesh",
                    &self.r#appmesh,
                ),
                to_pulumi_object_field(
                    "appregistry",
                    &self.r#appregistry,
                ),
                to_pulumi_object_field(
                    "apprunner",
                    &self.r#apprunner,
                ),
                to_pulumi_object_field(
                    "appstream",
                    &self.r#appstream,
                ),
                to_pulumi_object_field(
                    "appsync",
                    &self.r#appsync,
                ),
                to_pulumi_object_field(
                    "athena",
                    &self.r#athena,
                ),
                to_pulumi_object_field(
                    "auditmanager",
                    &self.r#auditmanager,
                ),
                to_pulumi_object_field(
                    "autoscaling",
                    &self.r#autoscaling,
                ),
                to_pulumi_object_field(
                    "autoscalingplans",
                    &self.r#autoscalingplans,
                ),
                to_pulumi_object_field(
                    "backup",
                    &self.r#backup,
                ),
                to_pulumi_object_field(
                    "batch",
                    &self.r#batch,
                ),
                to_pulumi_object_field(
                    "bcmdataexports",
                    &self.r#bcmdataexports,
                ),
                to_pulumi_object_field(
                    "beanstalk",
                    &self.r#beanstalk,
                ),
                to_pulumi_object_field(
                    "bedrock",
                    &self.r#bedrock,
                ),
                to_pulumi_object_field(
                    "bedrockagent",
                    &self.r#bedrockagent,
                ),
                to_pulumi_object_field(
                    "budgets",
                    &self.r#budgets,
                ),
                to_pulumi_object_field(
                    "ce",
                    &self.r#ce,
                ),
                to_pulumi_object_field(
                    "chatbot",
                    &self.r#chatbot,
                ),
                to_pulumi_object_field(
                    "chime",
                    &self.r#chime,
                ),
                to_pulumi_object_field(
                    "chimesdkmediapipelines",
                    &self.r#chimesdkmediapipelines,
                ),
                to_pulumi_object_field(
                    "chimesdkvoice",
                    &self.r#chimesdkvoice,
                ),
                to_pulumi_object_field(
                    "cleanrooms",
                    &self.r#cleanrooms,
                ),
                to_pulumi_object_field(
                    "cloud_9",
                    &self.r#cloud_9,
                ),
                to_pulumi_object_field(
                    "cloudcontrol",
                    &self.r#cloudcontrol,
                ),
                to_pulumi_object_field(
                    "cloudcontrolapi",
                    &self.r#cloudcontrolapi,
                ),
                to_pulumi_object_field(
                    "cloudformation",
                    &self.r#cloudformation,
                ),
                to_pulumi_object_field(
                    "cloudfront",
                    &self.r#cloudfront,
                ),
                to_pulumi_object_field(
                    "cloudfrontkeyvaluestore",
                    &self.r#cloudfrontkeyvaluestore,
                ),
                to_pulumi_object_field(
                    "cloudhsm",
                    &self.r#cloudhsm,
                ),
                to_pulumi_object_field(
                    "cloudhsmv_2",
                    &self.r#cloudhsmv_2,
                ),
                to_pulumi_object_field(
                    "cloudsearch",
                    &self.r#cloudsearch,
                ),
                to_pulumi_object_field(
                    "cloudtrail",
                    &self.r#cloudtrail,
                ),
                to_pulumi_object_field(
                    "cloudwatch",
                    &self.r#cloudwatch,
                ),
                to_pulumi_object_field(
                    "cloudwatchevents",
                    &self.r#cloudwatchevents,
                ),
                to_pulumi_object_field(
                    "cloudwatchevidently",
                    &self.r#cloudwatchevidently,
                ),
                to_pulumi_object_field(
                    "cloudwatchlog",
                    &self.r#cloudwatchlog,
                ),
                to_pulumi_object_field(
                    "cloudwatchlogs",
                    &self.r#cloudwatchlogs,
                ),
                to_pulumi_object_field(
                    "cloudwatchobservabilityaccessmanager",
                    &self.r#cloudwatchobservabilityaccessmanager,
                ),
                to_pulumi_object_field(
                    "cloudwatchrum",
                    &self.r#cloudwatchrum,
                ),
                to_pulumi_object_field(
                    "codeartifact",
                    &self.r#codeartifact,
                ),
                to_pulumi_object_field(
                    "codebuild",
                    &self.r#codebuild,
                ),
                to_pulumi_object_field(
                    "codecatalyst",
                    &self.r#codecatalyst,
                ),
                to_pulumi_object_field(
                    "codecommit",
                    &self.r#codecommit,
                ),
                to_pulumi_object_field(
                    "codeconnections",
                    &self.r#codeconnections,
                ),
                to_pulumi_object_field(
                    "codedeploy",
                    &self.r#codedeploy,
                ),
                to_pulumi_object_field(
                    "codeguruprofiler",
                    &self.r#codeguruprofiler,
                ),
                to_pulumi_object_field(
                    "codegurureviewer",
                    &self.r#codegurureviewer,
                ),
                to_pulumi_object_field(
                    "codepipeline",
                    &self.r#codepipeline,
                ),
                to_pulumi_object_field(
                    "codestarconnections",
                    &self.r#codestarconnections,
                ),
                to_pulumi_object_field(
                    "codestarnotifications",
                    &self.r#codestarnotifications,
                ),
                to_pulumi_object_field(
                    "cognitoidentity",
                    &self.r#cognitoidentity,
                ),
                to_pulumi_object_field(
                    "cognitoidentityprovider",
                    &self.r#cognitoidentityprovider,
                ),
                to_pulumi_object_field(
                    "cognitoidp",
                    &self.r#cognitoidp,
                ),
                to_pulumi_object_field(
                    "comprehend",
                    &self.r#comprehend,
                ),
                to_pulumi_object_field(
                    "computeoptimizer",
                    &self.r#computeoptimizer,
                ),
                to_pulumi_object_field(
                    "config",
                    &self.r#config,
                ),
                to_pulumi_object_field(
                    "configservice",
                    &self.r#configservice,
                ),
                to_pulumi_object_field(
                    "connect",
                    &self.r#connect,
                ),
                to_pulumi_object_field(
                    "connectcases",
                    &self.r#connectcases,
                ),
                to_pulumi_object_field(
                    "controltower",
                    &self.r#controltower,
                ),
                to_pulumi_object_field(
                    "costandusagereportservice",
                    &self.r#costandusagereportservice,
                ),
                to_pulumi_object_field(
                    "costexplorer",
                    &self.r#costexplorer,
                ),
                to_pulumi_object_field(
                    "costoptimizationhub",
                    &self.r#costoptimizationhub,
                ),
                to_pulumi_object_field(
                    "cur",
                    &self.r#cur,
                ),
                to_pulumi_object_field(
                    "customerprofiles",
                    &self.r#customerprofiles,
                ),
                to_pulumi_object_field(
                    "databasemigration",
                    &self.r#databasemigration,
                ),
                to_pulumi_object_field(
                    "databasemigrationservice",
                    &self.r#databasemigrationservice,
                ),
                to_pulumi_object_field(
                    "databrew",
                    &self.r#databrew,
                ),
                to_pulumi_object_field(
                    "dataexchange",
                    &self.r#dataexchange,
                ),
                to_pulumi_object_field(
                    "datapipeline",
                    &self.r#datapipeline,
                ),
                to_pulumi_object_field(
                    "datasync",
                    &self.r#datasync,
                ),
                to_pulumi_object_field(
                    "datazone",
                    &self.r#datazone,
                ),
                to_pulumi_object_field(
                    "dax",
                    &self.r#dax,
                ),
                to_pulumi_object_field(
                    "deploy",
                    &self.r#deploy,
                ),
                to_pulumi_object_field(
                    "detective",
                    &self.r#detective,
                ),
                to_pulumi_object_field(
                    "devicefarm",
                    &self.r#devicefarm,
                ),
                to_pulumi_object_field(
                    "devopsguru",
                    &self.r#devopsguru,
                ),
                to_pulumi_object_field(
                    "directconnect",
                    &self.r#directconnect,
                ),
                to_pulumi_object_field(
                    "directoryservice",
                    &self.r#directoryservice,
                ),
                to_pulumi_object_field(
                    "dlm",
                    &self.r#dlm,
                ),
                to_pulumi_object_field(
                    "dms",
                    &self.r#dms,
                ),
                to_pulumi_object_field(
                    "docdb",
                    &self.r#docdb,
                ),
                to_pulumi_object_field(
                    "docdbelastic",
                    &self.r#docdbelastic,
                ),
                to_pulumi_object_field(
                    "drs",
                    &self.r#drs,
                ),
                to_pulumi_object_field(
                    "ds",
                    &self.r#ds,
                ),
                to_pulumi_object_field(
                    "dynamodb",
                    &self.r#dynamodb,
                ),
                to_pulumi_object_field(
                    "ec_2",
                    &self.r#ec_2,
                ),
                to_pulumi_object_field(
                    "ecr",
                    &self.r#ecr,
                ),
                to_pulumi_object_field(
                    "ecrpublic",
                    &self.r#ecrpublic,
                ),
                to_pulumi_object_field(
                    "ecs",
                    &self.r#ecs,
                ),
                to_pulumi_object_field(
                    "efs",
                    &self.r#efs,
                ),
                to_pulumi_object_field(
                    "eks",
                    &self.r#eks,
                ),
                to_pulumi_object_field(
                    "elasticache",
                    &self.r#elasticache,
                ),
                to_pulumi_object_field(
                    "elasticbeanstalk",
                    &self.r#elasticbeanstalk,
                ),
                to_pulumi_object_field(
                    "elasticloadbalancing",
                    &self.r#elasticloadbalancing,
                ),
                to_pulumi_object_field(
                    "elasticloadbalancingv_2",
                    &self.r#elasticloadbalancingv_2,
                ),
                to_pulumi_object_field(
                    "elasticsearch",
                    &self.r#elasticsearch,
                ),
                to_pulumi_object_field(
                    "elasticsearchservice",
                    &self.r#elasticsearchservice,
                ),
                to_pulumi_object_field(
                    "elastictranscoder",
                    &self.r#elastictranscoder,
                ),
                to_pulumi_object_field(
                    "elb",
                    &self.r#elb,
                ),
                to_pulumi_object_field(
                    "elbv_2",
                    &self.r#elbv_2,
                ),
                to_pulumi_object_field(
                    "emr",
                    &self.r#emr,
                ),
                to_pulumi_object_field(
                    "emrcontainers",
                    &self.r#emrcontainers,
                ),
                to_pulumi_object_field(
                    "emrserverless",
                    &self.r#emrserverless,
                ),
                to_pulumi_object_field(
                    "es",
                    &self.r#es,
                ),
                to_pulumi_object_field(
                    "eventbridge",
                    &self.r#eventbridge,
                ),
                to_pulumi_object_field(
                    "events",
                    &self.r#events,
                ),
                to_pulumi_object_field(
                    "evidently",
                    &self.r#evidently,
                ),
                to_pulumi_object_field(
                    "finspace",
                    &self.r#finspace,
                ),
                to_pulumi_object_field(
                    "firehose",
                    &self.r#firehose,
                ),
                to_pulumi_object_field(
                    "fis",
                    &self.r#fis,
                ),
                to_pulumi_object_field(
                    "fms",
                    &self.r#fms,
                ),
                to_pulumi_object_field(
                    "fsx",
                    &self.r#fsx,
                ),
                to_pulumi_object_field(
                    "gamelift",
                    &self.r#gamelift,
                ),
                to_pulumi_object_field(
                    "glacier",
                    &self.r#glacier,
                ),
                to_pulumi_object_field(
                    "globalaccelerator",
                    &self.r#globalaccelerator,
                ),
                to_pulumi_object_field(
                    "glue",
                    &self.r#glue,
                ),
                to_pulumi_object_field(
                    "gluedatabrew",
                    &self.r#gluedatabrew,
                ),
                to_pulumi_object_field(
                    "grafana",
                    &self.r#grafana,
                ),
                to_pulumi_object_field(
                    "greengrass",
                    &self.r#greengrass,
                ),
                to_pulumi_object_field(
                    "groundstation",
                    &self.r#groundstation,
                ),
                to_pulumi_object_field(
                    "guardduty",
                    &self.r#guardduty,
                ),
                to_pulumi_object_field(
                    "healthlake",
                    &self.r#healthlake,
                ),
                to_pulumi_object_field(
                    "iam",
                    &self.r#iam,
                ),
                to_pulumi_object_field(
                    "identitystore",
                    &self.r#identitystore,
                ),
                to_pulumi_object_field(
                    "imagebuilder",
                    &self.r#imagebuilder,
                ),
                to_pulumi_object_field(
                    "inspector",
                    &self.r#inspector,
                ),
                to_pulumi_object_field(
                    "inspector_2",
                    &self.r#inspector_2,
                ),
                to_pulumi_object_field(
                    "inspectorv_2",
                    &self.r#inspectorv_2,
                ),
                to_pulumi_object_field(
                    "internetmonitor",
                    &self.r#internetmonitor,
                ),
                to_pulumi_object_field(
                    "iot",
                    &self.r#iot,
                ),
                to_pulumi_object_field(
                    "iotanalytics",
                    &self.r#iotanalytics,
                ),
                to_pulumi_object_field(
                    "iotevents",
                    &self.r#iotevents,
                ),
                to_pulumi_object_field(
                    "ivs",
                    &self.r#ivs,
                ),
                to_pulumi_object_field(
                    "ivschat",
                    &self.r#ivschat,
                ),
                to_pulumi_object_field(
                    "kafka",
                    &self.r#kafka,
                ),
                to_pulumi_object_field(
                    "kafkaconnect",
                    &self.r#kafkaconnect,
                ),
                to_pulumi_object_field(
                    "kendra",
                    &self.r#kendra,
                ),
                to_pulumi_object_field(
                    "keyspaces",
                    &self.r#keyspaces,
                ),
                to_pulumi_object_field(
                    "kinesis",
                    &self.r#kinesis,
                ),
                to_pulumi_object_field(
                    "kinesisanalytics",
                    &self.r#kinesisanalytics,
                ),
                to_pulumi_object_field(
                    "kinesisanalyticsv_2",
                    &self.r#kinesisanalyticsv_2,
                ),
                to_pulumi_object_field(
                    "kinesisvideo",
                    &self.r#kinesisvideo,
                ),
                to_pulumi_object_field(
                    "kms",
                    &self.r#kms,
                ),
                to_pulumi_object_field(
                    "lakeformation",
                    &self.r#lakeformation,
                ),
                to_pulumi_object_field(
                    "lambda",
                    &self.r#lambda,
                ),
                to_pulumi_object_field(
                    "launchwizard",
                    &self.r#launchwizard,
                ),
                to_pulumi_object_field(
                    "lex",
                    &self.r#lex,
                ),
                to_pulumi_object_field(
                    "lexmodelbuilding",
                    &self.r#lexmodelbuilding,
                ),
                to_pulumi_object_field(
                    "lexmodelbuildingservice",
                    &self.r#lexmodelbuildingservice,
                ),
                to_pulumi_object_field(
                    "lexmodels",
                    &self.r#lexmodels,
                ),
                to_pulumi_object_field(
                    "lexmodelsv_2",
                    &self.r#lexmodelsv_2,
                ),
                to_pulumi_object_field(
                    "lexv_2_models",
                    &self.r#lexv_2_models,
                ),
                to_pulumi_object_field(
                    "licensemanager",
                    &self.r#licensemanager,
                ),
                to_pulumi_object_field(
                    "lightsail",
                    &self.r#lightsail,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "locationservice",
                    &self.r#locationservice,
                ),
                to_pulumi_object_field(
                    "logs",
                    &self.r#logs,
                ),
                to_pulumi_object_field(
                    "lookoutmetrics",
                    &self.r#lookoutmetrics,
                ),
                to_pulumi_object_field(
                    "m_2",
                    &self.r#m_2,
                ),
                to_pulumi_object_field(
                    "macie_2",
                    &self.r#macie_2,
                ),
                to_pulumi_object_field(
                    "managedgrafana",
                    &self.r#managedgrafana,
                ),
                to_pulumi_object_field(
                    "mediaconnect",
                    &self.r#mediaconnect,
                ),
                to_pulumi_object_field(
                    "mediaconvert",
                    &self.r#mediaconvert,
                ),
                to_pulumi_object_field(
                    "medialive",
                    &self.r#medialive,
                ),
                to_pulumi_object_field(
                    "mediapackage",
                    &self.r#mediapackage,
                ),
                to_pulumi_object_field(
                    "mediapackagev_2",
                    &self.r#mediapackagev_2,
                ),
                to_pulumi_object_field(
                    "mediastore",
                    &self.r#mediastore,
                ),
                to_pulumi_object_field(
                    "memorydb",
                    &self.r#memorydb,
                ),
                to_pulumi_object_field(
                    "mgn",
                    &self.r#mgn,
                ),
                to_pulumi_object_field(
                    "mq",
                    &self.r#mq,
                ),
                to_pulumi_object_field(
                    "msk",
                    &self.r#msk,
                ),
                to_pulumi_object_field(
                    "mwaa",
                    &self.r#mwaa,
                ),
                to_pulumi_object_field(
                    "neptune",
                    &self.r#neptune,
                ),
                to_pulumi_object_field(
                    "neptunegraph",
                    &self.r#neptunegraph,
                ),
                to_pulumi_object_field(
                    "networkfirewall",
                    &self.r#networkfirewall,
                ),
                to_pulumi_object_field(
                    "networkmanager",
                    &self.r#networkmanager,
                ),
                to_pulumi_object_field(
                    "networkmonitor",
                    &self.r#networkmonitor,
                ),
                to_pulumi_object_field(
                    "oam",
                    &self.r#oam,
                ),
                to_pulumi_object_field(
                    "opensearch",
                    &self.r#opensearch,
                ),
                to_pulumi_object_field(
                    "opensearchingestion",
                    &self.r#opensearchingestion,
                ),
                to_pulumi_object_field(
                    "opensearchserverless",
                    &self.r#opensearchserverless,
                ),
                to_pulumi_object_field(
                    "opensearchservice",
                    &self.r#opensearchservice,
                ),
                to_pulumi_object_field(
                    "opsworks",
                    &self.r#opsworks,
                ),
                to_pulumi_object_field(
                    "organizations",
                    &self.r#organizations,
                ),
                to_pulumi_object_field(
                    "osis",
                    &self.r#osis,
                ),
                to_pulumi_object_field(
                    "outposts",
                    &self.r#outposts,
                ),
                to_pulumi_object_field(
                    "paymentcryptography",
                    &self.r#paymentcryptography,
                ),
                to_pulumi_object_field(
                    "pcaconnectorad",
                    &self.r#pcaconnectorad,
                ),
                to_pulumi_object_field(
                    "pcs",
                    &self.r#pcs,
                ),
                to_pulumi_object_field(
                    "pinpoint",
                    &self.r#pinpoint,
                ),
                to_pulumi_object_field(
                    "pinpointsmsvoicev_2",
                    &self.r#pinpointsmsvoicev_2,
                ),
                to_pulumi_object_field(
                    "pipes",
                    &self.r#pipes,
                ),
                to_pulumi_object_field(
                    "polly",
                    &self.r#polly,
                ),
                to_pulumi_object_field(
                    "pricing",
                    &self.r#pricing,
                ),
                to_pulumi_object_field(
                    "prometheus",
                    &self.r#prometheus,
                ),
                to_pulumi_object_field(
                    "prometheusservice",
                    &self.r#prometheusservice,
                ),
                to_pulumi_object_field(
                    "qbusiness",
                    &self.r#qbusiness,
                ),
                to_pulumi_object_field(
                    "qldb",
                    &self.r#qldb,
                ),
                to_pulumi_object_field(
                    "quicksight",
                    &self.r#quicksight,
                ),
                to_pulumi_object_field(
                    "ram",
                    &self.r#ram,
                ),
                to_pulumi_object_field(
                    "rbin",
                    &self.r#rbin,
                ),
                to_pulumi_object_field(
                    "rds",
                    &self.r#rds,
                ),
                to_pulumi_object_field(
                    "recyclebin",
                    &self.r#recyclebin,
                ),
                to_pulumi_object_field(
                    "redshift",
                    &self.r#redshift,
                ),
                to_pulumi_object_field(
                    "redshiftdata",
                    &self.r#redshiftdata,
                ),
                to_pulumi_object_field(
                    "redshiftdataapiservice",
                    &self.r#redshiftdataapiservice,
                ),
                to_pulumi_object_field(
                    "redshiftserverless",
                    &self.r#redshiftserverless,
                ),
                to_pulumi_object_field(
                    "rekognition",
                    &self.r#rekognition,
                ),
                to_pulumi_object_field(
                    "resiliencehub",
                    &self.r#resiliencehub,
                ),
                to_pulumi_object_field(
                    "resourceexplorer_2",
                    &self.r#resourceexplorer_2,
                ),
                to_pulumi_object_field(
                    "resourcegroups",
                    &self.r#resourcegroups,
                ),
                to_pulumi_object_field(
                    "resourcegroupstagging",
                    &self.r#resourcegroupstagging,
                ),
                to_pulumi_object_field(
                    "resourcegroupstaggingapi",
                    &self.r#resourcegroupstaggingapi,
                ),
                to_pulumi_object_field(
                    "rolesanywhere",
                    &self.r#rolesanywhere,
                ),
                to_pulumi_object_field(
                    "route_53",
                    &self.r#route_53,
                ),
                to_pulumi_object_field(
                    "route_53_domains",
                    &self.r#route_53_domains,
                ),
                to_pulumi_object_field(
                    "route_53_profiles",
                    &self.r#route_53_profiles,
                ),
                to_pulumi_object_field(
                    "route_53_recoverycontrolconfig",
                    &self.r#route_53_recoverycontrolconfig,
                ),
                to_pulumi_object_field(
                    "route_53_recoveryreadiness",
                    &self.r#route_53_recoveryreadiness,
                ),
                to_pulumi_object_field(
                    "route_53_resolver",
                    &self.r#route_53_resolver,
                ),
                to_pulumi_object_field(
                    "rum",
                    &self.r#rum,
                ),
                to_pulumi_object_field(
                    "s_3",
                    &self.r#s_3,
                ),
                to_pulumi_object_field(
                    "s_3_api",
                    &self.r#s_3_api,
                ),
                to_pulumi_object_field(
                    "s_3_control",
                    &self.r#s_3_control,
                ),
                to_pulumi_object_field(
                    "s_3_outposts",
                    &self.r#s_3_outposts,
                ),
                to_pulumi_object_field(
                    "s_3_tables",
                    &self.r#s_3_tables,
                ),
                to_pulumi_object_field(
                    "sagemaker",
                    &self.r#sagemaker,
                ),
                to_pulumi_object_field(
                    "scheduler",
                    &self.r#scheduler,
                ),
                to_pulumi_object_field(
                    "schemas",
                    &self.r#schemas,
                ),
                to_pulumi_object_field(
                    "sdb",
                    &self.r#sdb,
                ),
                to_pulumi_object_field(
                    "secretsmanager",
                    &self.r#secretsmanager,
                ),
                to_pulumi_object_field(
                    "securityhub",
                    &self.r#securityhub,
                ),
                to_pulumi_object_field(
                    "securitylake",
                    &self.r#securitylake,
                ),
                to_pulumi_object_field(
                    "serverlessapplicationrepository",
                    &self.r#serverlessapplicationrepository,
                ),
                to_pulumi_object_field(
                    "serverlessapprepo",
                    &self.r#serverlessapprepo,
                ),
                to_pulumi_object_field(
                    "serverlessrepo",
                    &self.r#serverlessrepo,
                ),
                to_pulumi_object_field(
                    "servicecatalog",
                    &self.r#servicecatalog,
                ),
                to_pulumi_object_field(
                    "servicecatalogappregistry",
                    &self.r#servicecatalogappregistry,
                ),
                to_pulumi_object_field(
                    "servicediscovery",
                    &self.r#servicediscovery,
                ),
                to_pulumi_object_field(
                    "servicequotas",
                    &self.r#servicequotas,
                ),
                to_pulumi_object_field(
                    "ses",
                    &self.r#ses,
                ),
                to_pulumi_object_field(
                    "sesv_2",
                    &self.r#sesv_2,
                ),
                to_pulumi_object_field(
                    "sfn",
                    &self.r#sfn,
                ),
                to_pulumi_object_field(
                    "shield",
                    &self.r#shield,
                ),
                to_pulumi_object_field(
                    "signer",
                    &self.r#signer,
                ),
                to_pulumi_object_field(
                    "simpledb",
                    &self.r#simpledb,
                ),
                to_pulumi_object_field(
                    "sns",
                    &self.r#sns,
                ),
                to_pulumi_object_field(
                    "sqs",
                    &self.r#sqs,
                ),
                to_pulumi_object_field(
                    "ssm",
                    &self.r#ssm,
                ),
                to_pulumi_object_field(
                    "ssmcontacts",
                    &self.r#ssmcontacts,
                ),
                to_pulumi_object_field(
                    "ssmincidents",
                    &self.r#ssmincidents,
                ),
                to_pulumi_object_field(
                    "ssmquicksetup",
                    &self.r#ssmquicksetup,
                ),
                to_pulumi_object_field(
                    "ssmsap",
                    &self.r#ssmsap,
                ),
                to_pulumi_object_field(
                    "sso",
                    &self.r#sso,
                ),
                to_pulumi_object_field(
                    "ssoadmin",
                    &self.r#ssoadmin,
                ),
                to_pulumi_object_field(
                    "stepfunctions",
                    &self.r#stepfunctions,
                ),
                to_pulumi_object_field(
                    "storagegateway",
                    &self.r#storagegateway,
                ),
                to_pulumi_object_field(
                    "sts",
                    &self.r#sts,
                ),
                to_pulumi_object_field(
                    "swf",
                    &self.r#swf,
                ),
                to_pulumi_object_field(
                    "synthetics",
                    &self.r#synthetics,
                ),
                to_pulumi_object_field(
                    "taxsettings",
                    &self.r#taxsettings,
                ),
                to_pulumi_object_field(
                    "timestreaminfluxdb",
                    &self.r#timestreaminfluxdb,
                ),
                to_pulumi_object_field(
                    "timestreamquery",
                    &self.r#timestreamquery,
                ),
                to_pulumi_object_field(
                    "timestreamwrite",
                    &self.r#timestreamwrite,
                ),
                to_pulumi_object_field(
                    "transcribe",
                    &self.r#transcribe,
                ),
                to_pulumi_object_field(
                    "transcribeservice",
                    &self.r#transcribeservice,
                ),
                to_pulumi_object_field(
                    "transfer",
                    &self.r#transfer,
                ),
                to_pulumi_object_field(
                    "verifiedpermissions",
                    &self.r#verifiedpermissions,
                ),
                to_pulumi_object_field(
                    "vpclattice",
                    &self.r#vpclattice,
                ),
                to_pulumi_object_field(
                    "waf",
                    &self.r#waf,
                ),
                to_pulumi_object_field(
                    "wafregional",
                    &self.r#wafregional,
                ),
                to_pulumi_object_field(
                    "wafv_2",
                    &self.r#wafv_2,
                ),
                to_pulumi_object_field(
                    "wellarchitected",
                    &self.r#wellarchitected,
                ),
                to_pulumi_object_field(
                    "worklink",
                    &self.r#worklink,
                ),
                to_pulumi_object_field(
                    "workspaces",
                    &self.r#workspaces,
                ),
                to_pulumi_object_field(
                    "workspacesweb",
                    &self.r#workspacesweb,
                ),
                to_pulumi_object_field(
                    "xray",
                    &self.r#xray,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderEndpoint {
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
                    r#accessanalyzer: {
                        let field_value = match fields_map.get("accessanalyzer") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessanalyzer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account: {
                        let field_value = match fields_map.get("account") {
                            Some(value) => value,
                            None => bail!("Missing field 'account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#acm: {
                        let field_value = match fields_map.get("acm") {
                            Some(value) => value,
                            None => bail!("Missing field 'acm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#acmpca: {
                        let field_value = match fields_map.get("acmpca") {
                            Some(value) => value,
                            None => bail!("Missing field 'acmpca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#amg: {
                        let field_value = match fields_map.get("amg") {
                            Some(value) => value,
                            None => bail!("Missing field 'amg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#amp: {
                        let field_value = match fields_map.get("amp") {
                            Some(value) => value,
                            None => bail!("Missing field 'amp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#amplify: {
                        let field_value = match fields_map.get("amplify") {
                            Some(value) => value,
                            None => bail!("Missing field 'amplify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apigateway: {
                        let field_value = match fields_map.get("apigateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'apigateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apigatewayv_2: {
                        let field_value = match fields_map.get("apigatewayv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'apigatewayv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appautoscaling: {
                        let field_value = match fields_map.get("appautoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'appautoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appconfig: {
                        let field_value = match fields_map.get("appconfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'appconfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appfabric: {
                        let field_value = match fields_map.get("appfabric") {
                            Some(value) => value,
                            None => bail!("Missing field 'appfabric' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appflow: {
                        let field_value = match fields_map.get("appflow") {
                            Some(value) => value,
                            None => bail!("Missing field 'appflow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appintegrations: {
                        let field_value = match fields_map.get("appintegrations") {
                            Some(value) => value,
                            None => bail!("Missing field 'appintegrations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appintegrationsservice: {
                        let field_value = match fields_map.get("appintegrationsservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'appintegrationsservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#applicationautoscaling: {
                        let field_value = match fields_map.get("applicationautoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationautoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#applicationinsights: {
                        let field_value = match fields_map.get("applicationinsights") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationinsights' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#applicationsignals: {
                        let field_value = match fields_map.get("applicationsignals") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationsignals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appmesh: {
                        let field_value = match fields_map.get("appmesh") {
                            Some(value) => value,
                            None => bail!("Missing field 'appmesh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appregistry: {
                        let field_value = match fields_map.get("appregistry") {
                            Some(value) => value,
                            None => bail!("Missing field 'appregistry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#apprunner: {
                        let field_value = match fields_map.get("apprunner") {
                            Some(value) => value,
                            None => bail!("Missing field 'apprunner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appstream: {
                        let field_value = match fields_map.get("appstream") {
                            Some(value) => value,
                            None => bail!("Missing field 'appstream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#appsync: {
                        let field_value = match fields_map.get("appsync") {
                            Some(value) => value,
                            None => bail!("Missing field 'appsync' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#athena: {
                        let field_value = match fields_map.get("athena") {
                            Some(value) => value,
                            None => bail!("Missing field 'athena' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auditmanager: {
                        let field_value = match fields_map.get("auditmanager") {
                            Some(value) => value,
                            None => bail!("Missing field 'auditmanager' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autoscaling: {
                        let field_value = match fields_map.get("autoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autoscalingplans: {
                        let field_value = match fields_map.get("autoscalingplans") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscalingplans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backup: {
                        let field_value = match fields_map.get("backup") {
                            Some(value) => value,
                            None => bail!("Missing field 'backup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch: {
                        let field_value = match fields_map.get("batch") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bcmdataexports: {
                        let field_value = match fields_map.get("bcmdataexports") {
                            Some(value) => value,
                            None => bail!("Missing field 'bcmdataexports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#beanstalk: {
                        let field_value = match fields_map.get("beanstalk") {
                            Some(value) => value,
                            None => bail!("Missing field 'beanstalk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bedrock: {
                        let field_value = match fields_map.get("bedrock") {
                            Some(value) => value,
                            None => bail!("Missing field 'bedrock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bedrockagent: {
                        let field_value = match fields_map.get("bedrockagent") {
                            Some(value) => value,
                            None => bail!("Missing field 'bedrockagent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#budgets: {
                        let field_value = match fields_map.get("budgets") {
                            Some(value) => value,
                            None => bail!("Missing field 'budgets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ce: {
                        let field_value = match fields_map.get("ce") {
                            Some(value) => value,
                            None => bail!("Missing field 'ce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#chatbot: {
                        let field_value = match fields_map.get("chatbot") {
                            Some(value) => value,
                            None => bail!("Missing field 'chatbot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#chime: {
                        let field_value = match fields_map.get("chime") {
                            Some(value) => value,
                            None => bail!("Missing field 'chime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#chimesdkmediapipelines: {
                        let field_value = match fields_map.get("chimesdkmediapipelines") {
                            Some(value) => value,
                            None => bail!("Missing field 'chimesdkmediapipelines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#chimesdkvoice: {
                        let field_value = match fields_map.get("chimesdkvoice") {
                            Some(value) => value,
                            None => bail!("Missing field 'chimesdkvoice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cleanrooms: {
                        let field_value = match fields_map.get("cleanrooms") {
                            Some(value) => value,
                            None => bail!("Missing field 'cleanrooms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_9: {
                        let field_value = match fields_map.get("cloud_9") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_9' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudcontrol: {
                        let field_value = match fields_map.get("cloudcontrol") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudcontrol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudcontrolapi: {
                        let field_value = match fields_map.get("cloudcontrolapi") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudcontrolapi' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudformation: {
                        let field_value = match fields_map.get("cloudformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudfront: {
                        let field_value = match fields_map.get("cloudfront") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudfront' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudfrontkeyvaluestore: {
                        let field_value = match fields_map.get("cloudfrontkeyvaluestore") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudfrontkeyvaluestore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudhsm: {
                        let field_value = match fields_map.get("cloudhsm") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudhsm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudhsmv_2: {
                        let field_value = match fields_map.get("cloudhsmv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudhsmv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudsearch: {
                        let field_value = match fields_map.get("cloudsearch") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudsearch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudtrail: {
                        let field_value = match fields_map.get("cloudtrail") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudtrail' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch: {
                        let field_value = match fields_map.get("cloudwatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchevents: {
                        let field_value = match fields_map.get("cloudwatchevents") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchevents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchevidently: {
                        let field_value = match fields_map.get("cloudwatchevidently") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchevidently' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchlog: {
                        let field_value = match fields_map.get("cloudwatchlog") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchlog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchlogs: {
                        let field_value = match fields_map.get("cloudwatchlogs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchlogs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchobservabilityaccessmanager: {
                        let field_value = match fields_map.get("cloudwatchobservabilityaccessmanager") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchobservabilityaccessmanager' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatchrum: {
                        let field_value = match fields_map.get("cloudwatchrum") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatchrum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codeartifact: {
                        let field_value = match fields_map.get("codeartifact") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeartifact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codebuild: {
                        let field_value = match fields_map.get("codebuild") {
                            Some(value) => value,
                            None => bail!("Missing field 'codebuild' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codecatalyst: {
                        let field_value = match fields_map.get("codecatalyst") {
                            Some(value) => value,
                            None => bail!("Missing field 'codecatalyst' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codecommit: {
                        let field_value = match fields_map.get("codecommit") {
                            Some(value) => value,
                            None => bail!("Missing field 'codecommit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codeconnections: {
                        let field_value = match fields_map.get("codeconnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeconnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codedeploy: {
                        let field_value = match fields_map.get("codedeploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'codedeploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codeguruprofiler: {
                        let field_value = match fields_map.get("codeguruprofiler") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeguruprofiler' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codegurureviewer: {
                        let field_value = match fields_map.get("codegurureviewer") {
                            Some(value) => value,
                            None => bail!("Missing field 'codegurureviewer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codepipeline: {
                        let field_value = match fields_map.get("codepipeline") {
                            Some(value) => value,
                            None => bail!("Missing field 'codepipeline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codestarconnections: {
                        let field_value = match fields_map.get("codestarconnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'codestarconnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#codestarnotifications: {
                        let field_value = match fields_map.get("codestarnotifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'codestarnotifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognitoidentity: {
                        let field_value = match fields_map.get("cognitoidentity") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognitoidentity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognitoidentityprovider: {
                        let field_value = match fields_map.get("cognitoidentityprovider") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognitoidentityprovider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognitoidp: {
                        let field_value = match fields_map.get("cognitoidp") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognitoidp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#comprehend: {
                        let field_value = match fields_map.get("comprehend") {
                            Some(value) => value,
                            None => bail!("Missing field 'comprehend' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#computeoptimizer: {
                        let field_value = match fields_map.get("computeoptimizer") {
                            Some(value) => value,
                            None => bail!("Missing field 'computeoptimizer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config: {
                        let field_value = match fields_map.get("config") {
                            Some(value) => value,
                            None => bail!("Missing field 'config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configservice: {
                        let field_value = match fields_map.get("configservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'configservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connect: {
                        let field_value = match fields_map.get("connect") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connectcases: {
                        let field_value = match fields_map.get("connectcases") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectcases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#controltower: {
                        let field_value = match fields_map.get("controltower") {
                            Some(value) => value,
                            None => bail!("Missing field 'controltower' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#costandusagereportservice: {
                        let field_value = match fields_map.get("costandusagereportservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'costandusagereportservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#costexplorer: {
                        let field_value = match fields_map.get("costexplorer") {
                            Some(value) => value,
                            None => bail!("Missing field 'costexplorer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#costoptimizationhub: {
                        let field_value = match fields_map.get("costoptimizationhub") {
                            Some(value) => value,
                            None => bail!("Missing field 'costoptimizationhub' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cur: {
                        let field_value = match fields_map.get("cur") {
                            Some(value) => value,
                            None => bail!("Missing field 'cur' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customerprofiles: {
                        let field_value = match fields_map.get("customerprofiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'customerprofiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#databasemigration: {
                        let field_value = match fields_map.get("databasemigration") {
                            Some(value) => value,
                            None => bail!("Missing field 'databasemigration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#databasemigrationservice: {
                        let field_value = match fields_map.get("databasemigrationservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'databasemigrationservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#databrew: {
                        let field_value = match fields_map.get("databrew") {
                            Some(value) => value,
                            None => bail!("Missing field 'databrew' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dataexchange: {
                        let field_value = match fields_map.get("dataexchange") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataexchange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datapipeline: {
                        let field_value = match fields_map.get("datapipeline") {
                            Some(value) => value,
                            None => bail!("Missing field 'datapipeline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datasync: {
                        let field_value = match fields_map.get("datasync") {
                            Some(value) => value,
                            None => bail!("Missing field 'datasync' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datazone: {
                        let field_value = match fields_map.get("datazone") {
                            Some(value) => value,
                            None => bail!("Missing field 'datazone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dax: {
                        let field_value = match fields_map.get("dax") {
                            Some(value) => value,
                            None => bail!("Missing field 'dax' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deploy: {
                        let field_value = match fields_map.get("deploy") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detective: {
                        let field_value = match fields_map.get("detective") {
                            Some(value) => value,
                            None => bail!("Missing field 'detective' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#devicefarm: {
                        let field_value = match fields_map.get("devicefarm") {
                            Some(value) => value,
                            None => bail!("Missing field 'devicefarm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#devopsguru: {
                        let field_value = match fields_map.get("devopsguru") {
                            Some(value) => value,
                            None => bail!("Missing field 'devopsguru' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directconnect: {
                        let field_value = match fields_map.get("directconnect") {
                            Some(value) => value,
                            None => bail!("Missing field 'directconnect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directoryservice: {
                        let field_value = match fields_map.get("directoryservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'directoryservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dlm: {
                        let field_value = match fields_map.get("dlm") {
                            Some(value) => value,
                            None => bail!("Missing field 'dlm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dms: {
                        let field_value = match fields_map.get("dms") {
                            Some(value) => value,
                            None => bail!("Missing field 'dms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docdb: {
                        let field_value = match fields_map.get("docdb") {
                            Some(value) => value,
                            None => bail!("Missing field 'docdb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docdbelastic: {
                        let field_value = match fields_map.get("docdbelastic") {
                            Some(value) => value,
                            None => bail!("Missing field 'docdbelastic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drs: {
                        let field_value = match fields_map.get("drs") {
                            Some(value) => value,
                            None => bail!("Missing field 'drs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ds: {
                        let field_value = match fields_map.get("ds") {
                            Some(value) => value,
                            None => bail!("Missing field 'ds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamodb: {
                        let field_value = match fields_map.get("dynamodb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamodb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ec_2: {
                        let field_value = match fields_map.get("ec_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'ec_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecr: {
                        let field_value = match fields_map.get("ecr") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecrpublic: {
                        let field_value = match fields_map.get("ecrpublic") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecrpublic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecs: {
                        let field_value = match fields_map.get("ecs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#efs: {
                        let field_value = match fields_map.get("efs") {
                            Some(value) => value,
                            None => bail!("Missing field 'efs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eks: {
                        let field_value = match fields_map.get("eks") {
                            Some(value) => value,
                            None => bail!("Missing field 'eks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticache: {
                        let field_value = match fields_map.get("elasticache") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticache' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticbeanstalk: {
                        let field_value = match fields_map.get("elasticbeanstalk") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticbeanstalk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticloadbalancing: {
                        let field_value = match fields_map.get("elasticloadbalancing") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticloadbalancing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticloadbalancingv_2: {
                        let field_value = match fields_map.get("elasticloadbalancingv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticloadbalancingv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticsearch: {
                        let field_value = match fields_map.get("elasticsearch") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticsearch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elasticsearchservice: {
                        let field_value = match fields_map.get("elasticsearchservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticsearchservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elastictranscoder: {
                        let field_value = match fields_map.get("elastictranscoder") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastictranscoder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elb: {
                        let field_value = match fields_map.get("elb") {
                            Some(value) => value,
                            None => bail!("Missing field 'elb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elbv_2: {
                        let field_value = match fields_map.get("elbv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'elbv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emr: {
                        let field_value = match fields_map.get("emr") {
                            Some(value) => value,
                            None => bail!("Missing field 'emr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emrcontainers: {
                        let field_value = match fields_map.get("emrcontainers") {
                            Some(value) => value,
                            None => bail!("Missing field 'emrcontainers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emrserverless: {
                        let field_value = match fields_map.get("emrserverless") {
                            Some(value) => value,
                            None => bail!("Missing field 'emrserverless' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#es: {
                        let field_value = match fields_map.get("es") {
                            Some(value) => value,
                            None => bail!("Missing field 'es' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eventbridge: {
                        let field_value = match fields_map.get("eventbridge") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventbridge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#events: {
                        let field_value = match fields_map.get("events") {
                            Some(value) => value,
                            None => bail!("Missing field 'events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evidently: {
                        let field_value = match fields_map.get("evidently") {
                            Some(value) => value,
                            None => bail!("Missing field 'evidently' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finspace: {
                        let field_value = match fields_map.get("finspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'finspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firehose: {
                        let field_value = match fields_map.get("firehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'firehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fis: {
                        let field_value = match fields_map.get("fis") {
                            Some(value) => value,
                            None => bail!("Missing field 'fis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fms: {
                        let field_value = match fields_map.get("fms") {
                            Some(value) => value,
                            None => bail!("Missing field 'fms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fsx: {
                        let field_value = match fields_map.get("fsx") {
                            Some(value) => value,
                            None => bail!("Missing field 'fsx' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gamelift: {
                        let field_value = match fields_map.get("gamelift") {
                            Some(value) => value,
                            None => bail!("Missing field 'gamelift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#glacier: {
                        let field_value = match fields_map.get("glacier") {
                            Some(value) => value,
                            None => bail!("Missing field 'glacier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#globalaccelerator: {
                        let field_value = match fields_map.get("globalaccelerator") {
                            Some(value) => value,
                            None => bail!("Missing field 'globalaccelerator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#glue: {
                        let field_value = match fields_map.get("glue") {
                            Some(value) => value,
                            None => bail!("Missing field 'glue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gluedatabrew: {
                        let field_value = match fields_map.get("gluedatabrew") {
                            Some(value) => value,
                            None => bail!("Missing field 'gluedatabrew' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grafana: {
                        let field_value = match fields_map.get("grafana") {
                            Some(value) => value,
                            None => bail!("Missing field 'grafana' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#greengrass: {
                        let field_value = match fields_map.get("greengrass") {
                            Some(value) => value,
                            None => bail!("Missing field 'greengrass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groundstation: {
                        let field_value = match fields_map.get("groundstation") {
                            Some(value) => value,
                            None => bail!("Missing field 'groundstation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guardduty: {
                        let field_value = match fields_map.get("guardduty") {
                            Some(value) => value,
                            None => bail!("Missing field 'guardduty' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#healthlake: {
                        let field_value = match fields_map.get("healthlake") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthlake' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam: {
                        let field_value = match fields_map.get("iam") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identitystore: {
                        let field_value = match fields_map.get("identitystore") {
                            Some(value) => value,
                            None => bail!("Missing field 'identitystore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#imagebuilder: {
                        let field_value = match fields_map.get("imagebuilder") {
                            Some(value) => value,
                            None => bail!("Missing field 'imagebuilder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inspector: {
                        let field_value = match fields_map.get("inspector") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inspector_2: {
                        let field_value = match fields_map.get("inspector_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspector_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inspectorv_2: {
                        let field_value = match fields_map.get("inspectorv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'inspectorv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#internetmonitor: {
                        let field_value = match fields_map.get("internetmonitor") {
                            Some(value) => value,
                            None => bail!("Missing field 'internetmonitor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iot: {
                        let field_value = match fields_map.get("iot") {
                            Some(value) => value,
                            None => bail!("Missing field 'iot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iotanalytics: {
                        let field_value = match fields_map.get("iotanalytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'iotanalytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iotevents: {
                        let field_value = match fields_map.get("iotevents") {
                            Some(value) => value,
                            None => bail!("Missing field 'iotevents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ivs: {
                        let field_value = match fields_map.get("ivs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ivs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ivschat: {
                        let field_value = match fields_map.get("ivschat") {
                            Some(value) => value,
                            None => bail!("Missing field 'ivschat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kafka: {
                        let field_value = match fields_map.get("kafka") {
                            Some(value) => value,
                            None => bail!("Missing field 'kafka' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kafkaconnect: {
                        let field_value = match fields_map.get("kafkaconnect") {
                            Some(value) => value,
                            None => bail!("Missing field 'kafkaconnect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kendra: {
                        let field_value = match fields_map.get("kendra") {
                            Some(value) => value,
                            None => bail!("Missing field 'kendra' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keyspaces: {
                        let field_value = match fields_map.get("keyspaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyspaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis: {
                        let field_value = match fields_map.get("kinesis") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesisanalytics: {
                        let field_value = match fields_map.get("kinesisanalytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisanalytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesisanalyticsv_2: {
                        let field_value = match fields_map.get("kinesisanalyticsv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisanalyticsv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesisvideo: {
                        let field_value = match fields_map.get("kinesisvideo") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisvideo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms: {
                        let field_value = match fields_map.get("kms") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lakeformation: {
                        let field_value = match fields_map.get("lakeformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'lakeformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda: {
                        let field_value = match fields_map.get("lambda") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launchwizard: {
                        let field_value = match fields_map.get("launchwizard") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchwizard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lex: {
                        let field_value = match fields_map.get("lex") {
                            Some(value) => value,
                            None => bail!("Missing field 'lex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lexmodelbuilding: {
                        let field_value = match fields_map.get("lexmodelbuilding") {
                            Some(value) => value,
                            None => bail!("Missing field 'lexmodelbuilding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lexmodelbuildingservice: {
                        let field_value = match fields_map.get("lexmodelbuildingservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'lexmodelbuildingservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lexmodels: {
                        let field_value = match fields_map.get("lexmodels") {
                            Some(value) => value,
                            None => bail!("Missing field 'lexmodels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lexmodelsv_2: {
                        let field_value = match fields_map.get("lexmodelsv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'lexmodelsv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lexv_2_models: {
                        let field_value = match fields_map.get("lexv_2_models") {
                            Some(value) => value,
                            None => bail!("Missing field 'lexv_2_models' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#licensemanager: {
                        let field_value = match fields_map.get("licensemanager") {
                            Some(value) => value,
                            None => bail!("Missing field 'licensemanager' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lightsail: {
                        let field_value = match fields_map.get("lightsail") {
                            Some(value) => value,
                            None => bail!("Missing field 'lightsail' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locationservice: {
                        let field_value = match fields_map.get("locationservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'locationservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#logs: {
                        let field_value = match fields_map.get("logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lookoutmetrics: {
                        let field_value = match fields_map.get("lookoutmetrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'lookoutmetrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#m_2: {
                        let field_value = match fields_map.get("m_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'm_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macie_2: {
                        let field_value = match fields_map.get("macie_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'macie_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managedgrafana: {
                        let field_value = match fields_map.get("managedgrafana") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedgrafana' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mediaconnect: {
                        let field_value = match fields_map.get("mediaconnect") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediaconnect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mediaconvert: {
                        let field_value = match fields_map.get("mediaconvert") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediaconvert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#medialive: {
                        let field_value = match fields_map.get("medialive") {
                            Some(value) => value,
                            None => bail!("Missing field 'medialive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mediapackage: {
                        let field_value = match fields_map.get("mediapackage") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediapackage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mediapackagev_2: {
                        let field_value = match fields_map.get("mediapackagev_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediapackagev_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mediastore: {
                        let field_value = match fields_map.get("mediastore") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediastore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memorydb: {
                        let field_value = match fields_map.get("memorydb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memorydb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mgn: {
                        let field_value = match fields_map.get("mgn") {
                            Some(value) => value,
                            None => bail!("Missing field 'mgn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mq: {
                        let field_value = match fields_map.get("mq") {
                            Some(value) => value,
                            None => bail!("Missing field 'mq' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#msk: {
                        let field_value = match fields_map.get("msk") {
                            Some(value) => value,
                            None => bail!("Missing field 'msk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mwaa: {
                        let field_value = match fields_map.get("mwaa") {
                            Some(value) => value,
                            None => bail!("Missing field 'mwaa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#neptune: {
                        let field_value = match fields_map.get("neptune") {
                            Some(value) => value,
                            None => bail!("Missing field 'neptune' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#neptunegraph: {
                        let field_value = match fields_map.get("neptunegraph") {
                            Some(value) => value,
                            None => bail!("Missing field 'neptunegraph' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#networkfirewall: {
                        let field_value = match fields_map.get("networkfirewall") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkfirewall' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#networkmanager: {
                        let field_value = match fields_map.get("networkmanager") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkmanager' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#networkmonitor: {
                        let field_value = match fields_map.get("networkmonitor") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkmonitor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oam: {
                        let field_value = match fields_map.get("oam") {
                            Some(value) => value,
                            None => bail!("Missing field 'oam' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opensearch: {
                        let field_value = match fields_map.get("opensearch") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opensearchingestion: {
                        let field_value = match fields_map.get("opensearchingestion") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearchingestion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opensearchserverless: {
                        let field_value = match fields_map.get("opensearchserverless") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearchserverless' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opensearchservice: {
                        let field_value = match fields_map.get("opensearchservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearchservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#opsworks: {
                        let field_value = match fields_map.get("opsworks") {
                            Some(value) => value,
                            None => bail!("Missing field 'opsworks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizations: {
                        let field_value = match fields_map.get("organizations") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#osis: {
                        let field_value = match fields_map.get("osis") {
                            Some(value) => value,
                            None => bail!("Missing field 'osis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outposts: {
                        let field_value = match fields_map.get("outposts") {
                            Some(value) => value,
                            None => bail!("Missing field 'outposts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#paymentcryptography: {
                        let field_value = match fields_map.get("paymentcryptography") {
                            Some(value) => value,
                            None => bail!("Missing field 'paymentcryptography' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcaconnectorad: {
                        let field_value = match fields_map.get("pcaconnectorad") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcaconnectorad' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pcs: {
                        let field_value = match fields_map.get("pcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'pcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinpoint: {
                        let field_value = match fields_map.get("pinpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinpointsmsvoicev_2: {
                        let field_value = match fields_map.get("pinpointsmsvoicev_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinpointsmsvoicev_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pipes: {
                        let field_value = match fields_map.get("pipes") {
                            Some(value) => value,
                            None => bail!("Missing field 'pipes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#polly: {
                        let field_value = match fields_map.get("polly") {
                            Some(value) => value,
                            None => bail!("Missing field 'polly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pricing: {
                        let field_value = match fields_map.get("pricing") {
                            Some(value) => value,
                            None => bail!("Missing field 'pricing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prometheus: {
                        let field_value = match fields_map.get("prometheus") {
                            Some(value) => value,
                            None => bail!("Missing field 'prometheus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prometheusservice: {
                        let field_value = match fields_map.get("prometheusservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'prometheusservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qbusiness: {
                        let field_value = match fields_map.get("qbusiness") {
                            Some(value) => value,
                            None => bail!("Missing field 'qbusiness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qldb: {
                        let field_value = match fields_map.get("qldb") {
                            Some(value) => value,
                            None => bail!("Missing field 'qldb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quicksight: {
                        let field_value = match fields_map.get("quicksight") {
                            Some(value) => value,
                            None => bail!("Missing field 'quicksight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ram: {
                        let field_value = match fields_map.get("ram") {
                            Some(value) => value,
                            None => bail!("Missing field 'ram' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rbin: {
                        let field_value = match fields_map.get("rbin") {
                            Some(value) => value,
                            None => bail!("Missing field 'rbin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rds: {
                        let field_value = match fields_map.get("rds") {
                            Some(value) => value,
                            None => bail!("Missing field 'rds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recyclebin: {
                        let field_value = match fields_map.get("recyclebin") {
                            Some(value) => value,
                            None => bail!("Missing field 'recyclebin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshift: {
                        let field_value = match fields_map.get("redshift") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshift' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshiftdata: {
                        let field_value = match fields_map.get("redshiftdata") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshiftdata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshiftdataapiservice: {
                        let field_value = match fields_map.get("redshiftdataapiservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshiftdataapiservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshiftserverless: {
                        let field_value = match fields_map.get("redshiftserverless") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshiftserverless' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rekognition: {
                        let field_value = match fields_map.get("rekognition") {
                            Some(value) => value,
                            None => bail!("Missing field 'rekognition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resiliencehub: {
                        let field_value = match fields_map.get("resiliencehub") {
                            Some(value) => value,
                            None => bail!("Missing field 'resiliencehub' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resourceexplorer_2: {
                        let field_value = match fields_map.get("resourceexplorer_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceexplorer_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resourcegroups: {
                        let field_value = match fields_map.get("resourcegroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourcegroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resourcegroupstagging: {
                        let field_value = match fields_map.get("resourcegroupstagging") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourcegroupstagging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resourcegroupstaggingapi: {
                        let field_value = match fields_map.get("resourcegroupstaggingapi") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourcegroupstaggingapi' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rolesanywhere: {
                        let field_value = match fields_map.get("rolesanywhere") {
                            Some(value) => value,
                            None => bail!("Missing field 'rolesanywhere' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53: {
                        let field_value = match fields_map.get("route_53") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53_domains: {
                        let field_value = match fields_map.get("route_53_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53_profiles: {
                        let field_value = match fields_map.get("route_53_profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53_profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53_recoverycontrolconfig: {
                        let field_value = match fields_map.get("route_53_recoverycontrolconfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53_recoverycontrolconfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53_recoveryreadiness: {
                        let field_value = match fields_map.get("route_53_recoveryreadiness") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53_recoveryreadiness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_53_resolver: {
                        let field_value = match fields_map.get("route_53_resolver") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_53_resolver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rum: {
                        let field_value = match fields_map.get("rum") {
                            Some(value) => value,
                            None => bail!("Missing field 'rum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_api: {
                        let field_value = match fields_map.get("s_3_api") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_api' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_control: {
                        let field_value = match fields_map.get("s_3_control") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_control' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_outposts: {
                        let field_value = match fields_map.get("s_3_outposts") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_outposts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_tables: {
                        let field_value = match fields_map.get("s_3_tables") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_tables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sagemaker: {
                        let field_value = match fields_map.get("sagemaker") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemaker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduler: {
                        let field_value = match fields_map.get("scheduler") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduler' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schemas: {
                        let field_value = match fields_map.get("schemas") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sdb: {
                        let field_value = match fields_map.get("sdb") {
                            Some(value) => value,
                            None => bail!("Missing field 'sdb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secretsmanager: {
                        let field_value = match fields_map.get("secretsmanager") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretsmanager' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#securityhub: {
                        let field_value = match fields_map.get("securityhub") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityhub' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#securitylake: {
                        let field_value = match fields_map.get("securitylake") {
                            Some(value) => value,
                            None => bail!("Missing field 'securitylake' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serverlessapplicationrepository: {
                        let field_value = match fields_map.get("serverlessapplicationrepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverlessapplicationrepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serverlessapprepo: {
                        let field_value = match fields_map.get("serverlessapprepo") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverlessapprepo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serverlessrepo: {
                        let field_value = match fields_map.get("serverlessrepo") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverlessrepo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servicecatalog: {
                        let field_value = match fields_map.get("servicecatalog") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicecatalog' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servicecatalogappregistry: {
                        let field_value = match fields_map.get("servicecatalogappregistry") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicecatalogappregistry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servicediscovery: {
                        let field_value = match fields_map.get("servicediscovery") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicediscovery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#servicequotas: {
                        let field_value = match fields_map.get("servicequotas") {
                            Some(value) => value,
                            None => bail!("Missing field 'servicequotas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ses: {
                        let field_value = match fields_map.get("ses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sesv_2: {
                        let field_value = match fields_map.get("sesv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'sesv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sfn: {
                        let field_value = match fields_map.get("sfn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sfn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shield: {
                        let field_value = match fields_map.get("shield") {
                            Some(value) => value,
                            None => bail!("Missing field 'shield' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signer: {
                        let field_value = match fields_map.get("signer") {
                            Some(value) => value,
                            None => bail!("Missing field 'signer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#simpledb: {
                        let field_value = match fields_map.get("simpledb") {
                            Some(value) => value,
                            None => bail!("Missing field 'simpledb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sns: {
                        let field_value = match fields_map.get("sns") {
                            Some(value) => value,
                            None => bail!("Missing field 'sns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqs: {
                        let field_value = match fields_map.get("sqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssm: {
                        let field_value = match fields_map.get("ssm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssmcontacts: {
                        let field_value = match fields_map.get("ssmcontacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssmcontacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssmincidents: {
                        let field_value = match fields_map.get("ssmincidents") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssmincidents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssmquicksetup: {
                        let field_value = match fields_map.get("ssmquicksetup") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssmquicksetup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssmsap: {
                        let field_value = match fields_map.get("ssmsap") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssmsap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso: {
                        let field_value = match fields_map.get("sso") {
                            Some(value) => value,
                            None => bail!("Missing field 'sso' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssoadmin: {
                        let field_value = match fields_map.get("ssoadmin") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssoadmin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stepfunctions: {
                        let field_value = match fields_map.get("stepfunctions") {
                            Some(value) => value,
                            None => bail!("Missing field 'stepfunctions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storagegateway: {
                        let field_value = match fields_map.get("storagegateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'storagegateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sts: {
                        let field_value = match fields_map.get("sts") {
                            Some(value) => value,
                            None => bail!("Missing field 'sts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#swf: {
                        let field_value = match fields_map.get("swf") {
                            Some(value) => value,
                            None => bail!("Missing field 'swf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#synthetics: {
                        let field_value = match fields_map.get("synthetics") {
                            Some(value) => value,
                            None => bail!("Missing field 'synthetics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#taxsettings: {
                        let field_value = match fields_map.get("taxsettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'taxsettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestreaminfluxdb: {
                        let field_value = match fields_map.get("timestreaminfluxdb") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestreaminfluxdb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestreamquery: {
                        let field_value = match fields_map.get("timestreamquery") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestreamquery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestreamwrite: {
                        let field_value = match fields_map.get("timestreamwrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestreamwrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transcribe: {
                        let field_value = match fields_map.get("transcribe") {
                            Some(value) => value,
                            None => bail!("Missing field 'transcribe' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transcribeservice: {
                        let field_value = match fields_map.get("transcribeservice") {
                            Some(value) => value,
                            None => bail!("Missing field 'transcribeservice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transfer: {
                        let field_value = match fields_map.get("transfer") {
                            Some(value) => value,
                            None => bail!("Missing field 'transfer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verifiedpermissions: {
                        let field_value = match fields_map.get("verifiedpermissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'verifiedpermissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpclattice: {
                        let field_value = match fields_map.get("vpclattice") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpclattice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#waf: {
                        let field_value = match fields_map.get("waf") {
                            Some(value) => value,
                            None => bail!("Missing field 'waf' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wafregional: {
                        let field_value = match fields_map.get("wafregional") {
                            Some(value) => value,
                            None => bail!("Missing field 'wafregional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wafv_2: {
                        let field_value = match fields_map.get("wafv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'wafv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wellarchitected: {
                        let field_value = match fields_map.get("wellarchitected") {
                            Some(value) => value,
                            None => bail!("Missing field 'wellarchitected' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worklink: {
                        let field_value = match fields_map.get("worklink") {
                            Some(value) => value,
                            None => bail!("Missing field 'worklink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspaces: {
                        let field_value = match fields_map.get("workspaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspacesweb: {
                        let field_value = match fields_map.get("workspacesweb") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspacesweb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xray: {
                        let field_value = match fields_map.get("xray") {
                            Some(value) => value,
                            None => bail!("Missing field 'xray' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
