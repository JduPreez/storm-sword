/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "storm-sword",
      removal: input?.stage === "production" ? "retain" : "remove",
      protect: ["production"].includes(input?.stage),
      home: "aws",
    };
  },
  async run() {
    // Remember: We only have to define fields we want to index on.
    const dev = false;
    const eventsTable = new sst.aws.Dynamo("Events", {
      fields: {
        Ns: "string", // event_type~~country_code
        Id: "string", // CUID
        StartDate: "number",
        EndDate: "number",
        DistanceMin: "number",
        DistanceMax: "number",
        Date: "number",
        AddressAdministrativeAreaIdx: "string", // event_type~~country_code~~administrative_area
      },
      primaryIndex: { hashKey: "Ns", rangeKey: "Id" },
      globalIndexes: {
        StartDateIndex: { hashKey: "Ns", rangeKey: "StartDate" },
        DateIndex: { hashKey: "Ns", rangeKey: "Date" },
        EndDateIndex: { hashKey: "Ns", rangeKey: "EndDate" },
        DistanceMinIndex: { hashKey: "Ns", rangeKey: "DistanceMin" },
        DistanceMaxIndex: { hashKey: "Ns", rangeKey: "DistanceMax" },
        AddressAdministrativeAreaIndex: { hashKey: "AddressAdministrativeAreaIdx", rangeKey: "Date" },
      }
    });

    // TODO (maybe): https://sst.dev/docs/examples/#aws-lamda-rust-multiple-binaries

    // Private Events Service Lambda - no HTTP endpoint
    const eventsService = new sst.aws.Function("EventsService", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/events/target/lambda/events",
      link: [eventsTable],
      dev,
      environment: {
        EVENTS_TABLE_NAME: eventsTable.name,
        RUST_LOG: "info",
      }
    });

    // Shared-secret token for mutating public-api routes. Set the value with:
    //   npx sst secret set EventsApiToken <token> --stage <stage>
    const eventsApiToken = new sst.Secret("EventsApiToken");

    // Public API Gateway
    const api = new sst.aws.ApiGatewayV2("PublicApi");

    api.route("GET /health", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      environment: { RUST_LOG: "info" },
    });

    /* Path segments must match [\w-]+ (word chars + hyphens — the regex http_router uses for {param}).
    So eventType (event_type) can't carry a space or %20 in the path. Send TrailRun, not Trail Run.
    This is fine because the events service runs the value through normalize (whitespace-strip + lowercase)
    before building Ns — but note a hyphen is not stripped by normalize, so trail-run and trailrun are different
    namespaces. The client should send the whitespace-free form of whatever was used at save time.
    */
    api.route("GET /events/{eventType}/{countryCode}", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      permissions: [{ actions: ["lambda:InvokeFunction"], resources: [eventsService.arn] }],
      environment: {
        EVENTS_LAMBDA_ARN: eventsService.arn,
        RUST_LOG: "info",
      },
    });

    api.route("POST /events", {
      runtime: "provided.al2023",
      handler: "bootstrap",
      bundle: "services/apps/public-api/target/lambda/public-api",
      dev,
      permissions: [{ actions: ["lambda:InvokeFunction"], resources: [eventsService.arn] }],
      environment: {
        EVENTS_LAMBDA_ARN: eventsService.arn,
        API_TOKEN: eventsApiToken.value,
        RUST_LOG: "info",
      },
    });

    return {
      api: api.url,
      eventsService: eventsService.name,
    };
  },
});
