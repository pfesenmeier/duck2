import { Duration, Stack, StackProps, CfnOutput } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';
import { LambdaToSqsToLambda } from "@aws-solutions-constructs/aws-lambda-sqs-lambda";
import { FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda';

export class BackendStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const producerLambda =   new RustFunction(this, 'producer-lambda', {
      directory: '/homo/pfese/backend/lambdas/producer-lambda/',
    });

    const fnUrl = producerLambda.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });
    
    // maybe someday... 
    //
    // set authentication scheme on the endpoint
    //
    // agree on some secret in secrets manager
    // https://bobbyhadz.com/blog/get-secrets-manager-values-aws-cdk
    //
    // Set up an authorizer that decodes jwts
    // And grants requests roles on decode
    // 
    // Then grant that role ability to invoke endpoint
    // fnUrl.grantInvokeUrl(role)

    const consumerLambda =   new RustFunction(this, 'consumer-lambda', {
      directory: '/homo/pfese/backend/lambdas/consumer-lambda/'
    });

    new LambdaToSqsToLambda(this, 'LambdaToSqsToLambdaPattern', {
      existingProducerLambdaObj: producerLambda,
      existingConsumerLambdaObj: consumerLambda,
      queueProps: {
        visibilityTimeout: Duration.days(1),
      },
    });

    new CfnOutput(this, 'ProducerLambdaUrl', {
      // The .url attributes will return the unique Function URL
      value: fnUrl.url,
    })
  }
}
