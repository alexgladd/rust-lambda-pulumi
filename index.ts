import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
// import * as awsx from "@pulumi/awsx";

// The following resources appear to be the bare minimum required to set up an HTTP APIGWv2 and get
// it integrated with a Lambda function

const role = new aws.iam.Role("LambdaRole", {
  assumeRolePolicy: {
    Version: "2012-10-17",
    Statement: [{
        Action: "sts:AssumeRole",
        Principal: {
            Service: "lambda.amazonaws.com",
        },
        Effect: "Allow",
    }],
  },
});

const policy = new aws.iam.RolePolicyAttachment("LambdaExecutePolicy", {
  role: role,
  policyArn: aws.iam.ManagedPolicies.AWSLambdaBasicExecutionRole,
});

const func = new aws.lambda.Function("RustLambda", {
  architectures: ["x86_64"],
  runtime: "provided.al2023",
  role: role.arn,
  handler: "bootstrap",
  code: new pulumi.asset.FileArchive("./target/lambda/rust-lambda/bootstrap.zip"),
});

const api = new aws.apigatewayv2.Api("RustLambdaApi", {
  protocolType: "HTTP",
});

const funcExec = new aws.lambda.Permission("RustLambdaExec", {
  function: func.name,
  action: "lambda:InvokeFunction",
  principal: "apigateway.amazonaws.com",
  sourceArn: pulumi.interpolate `${api.executionArn}/*`
});

const stage = new aws.apigatewayv2.Stage("RustLambdaApiStage", {
  apiId: api.id,
  name: "$default",
  autoDeploy: true,
});

const handler = new aws.apigatewayv2.Integration("RustLambdaApiIntegration", {
  apiId: api.id,
  integrationType: "AWS_PROXY",
  integrationMethod: "POST",
  integrationUri: func.invokeArn,
  payloadFormatVersion: "2.0",
});

const route = new aws.apigatewayv2.Route("RustLambdaProxyRoute", {
  apiId: api.id,
  routeKey: "ANY /{proxy+}",
  target: pulumi.interpolate `integrations/${handler.id}`,
});

// exports
export const endpoint = api.apiEndpoint;
