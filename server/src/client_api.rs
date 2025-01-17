use crate::error::{EdgeError, FeatureError};
use crate::http::feature_refresher::FeatureRefresher;
use crate::metrics::client_metrics::MetricsCache;
use crate::tokens::cache_key;
use crate::types::{EdgeJsonResult, EdgeResult, EdgeToken, FeatureFilters, ProjectFilter};
use actix_web::web::{self, Data, Json, Query};
use actix_web::{get, post, HttpRequest, HttpResponse};
use dashmap::DashMap;
use unleash_types::client_features::{ClientFeature, ClientFeatures};
use unleash_types::client_metrics::{ClientApplication, ClientMetrics, ConnectVia};

#[utoipa::path(
    context_path = "/api",
    params(FeatureFilters),
    responses(
        (status = 200, description = "Return feature toggles for this token", body = ClientFeatures),
        (status = 403, description = "Was not allowed to access features"),
        (status = 400, description = "Invalid parameters used")
    ),
    security(
        ("Authorization" = [])
    )
)]
#[get("/client/features")]
pub async fn get_features(
    edge_token: EdgeToken,
    features_cache: Data<DashMap<String, ClientFeatures>>,
    token_cache: Data<DashMap<String, EdgeToken>>,
    filter_query: Query<FeatureFilters>,
    req: HttpRequest,
) -> EdgeJsonResult<ClientFeatures> {
    resolve_features(edge_token, features_cache, token_cache, filter_query, req).await
}
#[utoipa::path(
    context_path = "/api",
    params(FeatureFilters),
    responses(
        (status = 200, description = "Return feature toggles for this token", body = ClientFeatures),
        (status = 403, description = "Was not allowed to access features"),
        (status = 400, description = "Invalid parameters used")
    ),
    security(
        ("Authorization" = [])
    )
)]
#[post("/client/features")]
pub async fn post_features(
    edge_token: EdgeToken,
    features_cache: Data<DashMap<String, ClientFeatures>>,
    token_cache: Data<DashMap<String, EdgeToken>>,
    filter_query: Query<FeatureFilters>,
    req: HttpRequest,
) -> EdgeJsonResult<ClientFeatures> {
    resolve_features(edge_token, features_cache, token_cache, filter_query, req).await
}

async fn resolve_features(
    edge_token: EdgeToken,
    features_cache: Data<DashMap<String, ClientFeatures>>,
    token_cache: Data<DashMap<String, EdgeToken>>,
    filter_query: Query<FeatureFilters>,
    req: HttpRequest,
) -> EdgeJsonResult<ClientFeatures> {
    let validated_token = token_cache
        .get(&edge_token.token)
        .map(|e| e.value().clone())
        .ok_or(EdgeError::AuthorizationDenied)?;
    let client_features = match req.app_data::<Data<FeatureRefresher>>() {
        Some(refresher) => refresher.features_for_token(validated_token.clone()).await,
        None => features_cache
            .get(&cache_key(&edge_token))
            .map(|features| features.value().clone())
            .map(|client_features| ClientFeatures {
                features: client_features
                    .features
                    .filter_by_projects(&validated_token),
                ..client_features
            })
            .ok_or(EdgeError::ClientFeaturesFetchError(FeatureError::Retriable)),
    };
    let filters = filter_query.into_inner();
    let query = unleash_types::client_features::Query {
        tags: None,
        projects: Some(validated_token.projects),
        name_prefix: filters.name_prefix.clone(),
        environment: validated_token.environment,
        inline_segment_constraints: Some(false),
    };
    client_features
        .map(|f| ClientFeatures {
            query: Some(query),
            features: f
                .features
                .clone()
                .into_iter()
                .filter(|f| {
                    filters
                        .name_prefix
                        .clone()
                        .map(|prefix| f.name.starts_with(&prefix))
                        .unwrap_or(true)
                })
                .collect(),
            ..f
        })
        .map(Json)
}
#[utoipa::path(
    context_path = "/api",
    params(("feature_name" = String, Path,)),
    responses(
        (status = 200, description = "Return feature toggles for this token", body = ClientFeature),
        (status = 403, description = "Was not allowed to access feature"),
        (status = 400, description = "Invalid parameters used"),
        (status = 404, description = "Feature did not exist or token used was not allowed to access it")
    ),
    security(
        ("Authorization" = [])
    )
)]
#[get("/client/features/{feature_name}")]
pub async fn get_feature(
    edge_token: EdgeToken,
    features_cache: Data<DashMap<String, ClientFeatures>>,
    token_cache: Data<DashMap<String, EdgeToken>>,
    feature_name: web::Path<String>,
    req: HttpRequest,
) -> EdgeJsonResult<ClientFeature> {
    let validated_token = token_cache
        .get(&edge_token.token)
        .map(|e| e.value().clone())
        .ok_or(EdgeError::AuthorizationDenied)?;
    let client_features = match req.app_data::<Data<FeatureRefresher>>() {
        Some(refresher) => refresher.features_for_token(validated_token).await,
        None => features_cache
            .get(&cache_key(&edge_token))
            .map(|features| features.value().clone())
            .map(|client_features| ClientFeatures {
                features: client_features
                    .features
                    .filter_by_projects(&validated_token),
                ..client_features
            })
            .ok_or(EdgeError::ClientFeaturesFetchError(FeatureError::Retriable)),
    }?;
    client_features
        .features
        .into_iter()
        .find(|feature| feature.name == feature_name.clone())
        .map(Json)
        .ok_or(EdgeError::FeatureNotFound(feature_name.into_inner()))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 202, description = "Accepted client application registration"),
        (status = 403, description = "Was not allowed to register client application"),
    ),
    request_body = ClientApplication,
    security(
        ("Authorization" = [])
    )
)]
#[post("/client/register")]
pub async fn register(
    edge_token: EdgeToken,
    connect_via: Data<ConnectVia>,
    client_application: Json<ClientApplication>,
    metrics_cache: Data<MetricsCache>,
) -> EdgeResult<HttpResponse> {
    crate::metrics::client_metrics::register_client_application(
        edge_token,
        &connect_via,
        client_application.into_inner(),
        metrics_cache,
    );
    Ok(HttpResponse::Accepted().finish())
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 202, description = "Accepted client metrics"),
        (status = 403, description = "Was not allowed to post metrics"),
    ),
    request_body = ClientMetrics,
    security(
        ("Authorization" = [])
    )
)]
#[post("/client/metrics")]
pub async fn metrics(
    edge_token: EdgeToken,
    metrics: Json<ClientMetrics>,
    metrics_cache: Data<MetricsCache>,
) -> EdgeResult<HttpResponse> {
    crate::metrics::client_metrics::register_client_metrics(
        edge_token,
        metrics.into_inner(),
        metrics_cache,
    );
    Ok(HttpResponse::Accepted().finish())
}

pub fn configure_client_api(cfg: &mut web::ServiceConfig) {
    cfg.service(get_features)
        .service(get_feature)
        .service(register)
        .service(metrics);
}

pub fn configure_experimental_post_features(
    cfg: &mut web::ServiceConfig,
    post_features_enabled: bool,
) {
    if post_features_enabled {
        cfg.service(post_features);
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use std::str::FromStr;
    use std::{collections::HashMap, sync::Arc};

    use crate::metrics::client_metrics::{ApplicationKey, MetricsKey};
    use crate::types::{TokenType, TokenValidationStatus};

    use super::*;

    use crate::auth::token_validator::TokenValidator;
    use crate::cli::OfflineArgs;
    use crate::http::unleash_client::UnleashClient;
    use crate::middleware;
    use crate::tests::{features_from_disk, upstream_server};
    use actix_http::Request;
    use actix_web::{
        http::header::ContentType,
        test,
        web::{self, Data},
        App,
    };
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use maplit::hashmap;
    use reqwest::StatusCode;
    use ulid::Ulid;
    use unleash_types::client_features::{ClientFeature, Constraint, Operator, Strategy};
    use unleash_types::client_metrics::{ClientMetricsEnv, MetricBucket, ToggleStats};
    use unleash_yggdrasil::EngineState;

    async fn make_metrics_post_request() -> Request {
        test::TestRequest::post()
            .uri("/api/client/metrics")
            .insert_header(ContentType::json())
            .insert_header((
                "Authorization",
                "*:development.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7",
            ))
            .set_json(Json(ClientMetrics {
                app_name: "some-app".into(),
                instance_id: Some("some-instance".into()),
                bucket: MetricBucket {
                    start: Utc.with_ymd_and_hms(1867, 11, 7, 12, 0, 0).unwrap(),
                    stop: Utc.with_ymd_and_hms(1934, 11, 7, 12, 0, 0).unwrap(),
                    toggles: hashmap! {
                        "some-feature".to_string() => ToggleStats {
                            yes: 1,
                            no: 0,
                            variants: hashmap! {}
                        }
                    },
                },
                environment: Some("development".into()),
            }))
            .to_request()
    }

    async fn make_register_post_request(application: ClientApplication) -> Request {
        test::TestRequest::post()
            .uri("/api/client/register")
            .insert_header(ContentType::json())
            .insert_header((
                "Authorization",
                "*:development.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7",
            ))
            .set_json(Json(application))
            .to_request()
    }

    async fn make_features_request_with_token(token: EdgeToken) -> Request {
        test::TestRequest::get()
            .uri("/api/client/features")
            .insert_header(("Authorization", token.token))
            .to_request()
    }

    #[actix_web::test]
    async fn metrics_endpoint_correctly_aggregates_data() {
        let metrics_cache = Arc::new(MetricsCache::default());

        let app = test::init_service(
            App::new()
                .app_data(Data::new(ConnectVia {
                    app_name: "test".into(),
                    instance_id: Ulid::new().to_string(),
                }))
                .app_data(Data::from(metrics_cache.clone()))
                .service(web::scope("/api").service(metrics)),
        )
        .await;

        let req = make_metrics_post_request().await;
        let _result = test::call_and_read_body(&app, req).await;

        let cache = metrics_cache.clone();

        let found_metric = cache
            .metrics
            .get(&MetricsKey {
                app_name: "some-app".into(),
                feature_name: "some-feature".into(),
                timestamp: DateTime::parse_from_rfc3339("1867-11-07T12:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                environment: "development".into(),
            })
            .unwrap();

        let expected = ClientMetricsEnv {
            app_name: "some-app".into(),
            feature_name: "some-feature".into(),
            environment: "development".into(),
            timestamp: DateTime::parse_from_rfc3339("1867-11-07T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            yes: 1,
            no: 0,
            variants: HashMap::new(),
        };

        assert_eq!(found_metric.yes, expected.yes);
        assert_eq!(found_metric.yes, 1);
        assert_eq!(found_metric.no, 0);
        assert_eq!(found_metric.no, expected.no);
    }

    fn cached_client_features() -> ClientFeatures {
        ClientFeatures {
            version: 2,
            features: vec![
                ClientFeature {
                    name: "feature_one".into(),
                    feature_type: Some("release".into()),
                    description: Some("test feature".into()),
                    created_at: Some(Utc::now()),
                    last_seen_at: None,
                    enabled: true,
                    stale: Some(false),
                    impression_data: Some(false),
                    project: Some("default".into()),
                    strategies: Some(vec![
                        Strategy {
                            name: "standard".into(),
                            sort_order: Some(500),
                            segments: None,
                            constraints: None,
                            parameters: None,
                        },
                        Strategy {
                            name: "gradualRollout".into(),
                            sort_order: Some(100),
                            segments: None,
                            constraints: None,
                            parameters: None,
                        },
                    ]),
                    variants: None,
                },
                ClientFeature {
                    name: "feature_two_no_strats".into(),
                    feature_type: None,
                    description: None,
                    created_at: Some(Utc.with_ymd_and_hms(2022, 12, 5, 12, 31, 0).unwrap()),
                    last_seen_at: None,
                    enabled: true,
                    stale: None,
                    impression_data: None,
                    project: Some("default".into()),
                    strategies: None,
                    variants: None,
                },
                ClientFeature {
                    name: "feature_three".into(),
                    feature_type: Some("release".into()),
                    description: None,
                    created_at: None,
                    last_seen_at: None,
                    enabled: true,
                    stale: None,
                    impression_data: None,
                    project: Some("default".into()),
                    strategies: Some(vec![
                        Strategy {
                            name: "gradualRollout".to_string(),
                            sort_order: None,
                            segments: None,
                            constraints: Some(vec![Constraint {
                                context_name: "version".to_string(),
                                operator: Operator::SemverGt,
                                case_insensitive: false,
                                inverted: false,
                                values: None,
                                value: Some("1.5.0".into()),
                            }]),
                            parameters: None,
                        },
                        Strategy {
                            name: "".to_string(),
                            sort_order: None,
                            segments: None,
                            constraints: None,
                            parameters: None,
                        },
                    ]),
                    variants: None,
                },
            ],
            segments: None,
            query: None,
        }
    }

    #[tokio::test]
    async fn register_endpoint_correctly_aggregates_applications() {
        let metrics_cache = Arc::new(MetricsCache::default());
        let our_app = ConnectVia {
            app_name: "test".into(),
            instance_id: Ulid::new().to_string(),
        };
        let app = test::init_service(
            App::new()
                .app_data(Data::new(our_app.clone()))
                .app_data(Data::from(metrics_cache.clone()))
                .service(web::scope("/api").service(register)),
        )
        .await;
        let mut client_app = ClientApplication::new("test_application", 15);
        client_app.instance_id = Some("test_instance".into());
        let req = make_register_post_request(client_app.clone()).await;
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::ACCEPTED);
        assert_eq!(metrics_cache.applications.len(), 1);
        let application_key = ApplicationKey {
            app_name: client_app.app_name.clone(),
            instance_id: client_app.instance_id.unwrap(),
        };
        let saved_app = metrics_cache
            .applications
            .get(&application_key)
            .unwrap()
            .value()
            .clone();
        assert_eq!(saved_app.app_name, client_app.app_name);
        assert_eq!(saved_app.connect_via, Some(vec![our_app]));
    }

    #[tokio::test]
    async fn client_features_endpoint_correctly_returns_cached_features() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .service(web::scope("/api").service(get_features)),
        )
        .await;
        let client_features = cached_client_features();
        let example_features = features_from_disk("../examples/features.json");
        features_cache.insert("development".into(), client_features.clone());
        features_cache.insert("production".into(), example_features.clone());
        let mut token = EdgeToken::try_from(
            "*:development.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7".to_string(),
        )
        .unwrap();
        token.token_type = Some(TokenType::Client);
        token.status = TokenValidationStatus::Validated;
        token_cache.insert(token.token.clone(), token.clone());
        let req = make_features_request_with_token(token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features, client_features.features);
        let mut production_token = EdgeToken::try_from(
            "*:production.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7".to_string(),
        )
        .unwrap();
        production_token.token_type = Some(TokenType::Client);
        production_token.status = TokenValidationStatus::Validated;
        token_cache.insert(production_token.token.clone(), production_token.clone());
        let req = make_features_request_with_token(production_token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features.len(), example_features.features.len());
    }

    #[tokio::test]
    async fn post_request_to_client_features_does_the_same_as_get_when_mounted() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .service(
                    web::scope("/api")
                        .service(get_features)
                        .service(post_features),
                ),
        )
        .await;
        let client_features = cached_client_features();
        let example_features = features_from_disk("../examples/features.json");
        features_cache.insert("development".into(), client_features.clone());
        features_cache.insert("production".into(), example_features.clone());
        let mut token = EdgeToken::try_from(
            "*:development.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7".to_string(),
        )
        .unwrap();
        token.token_type = Some(TokenType::Client);
        token.status = TokenValidationStatus::Validated;
        token_cache.insert(token.token.clone(), token.clone());
        let req = make_features_request_with_token(token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features, client_features.features);
        let mut production_token = EdgeToken::try_from(
            "*:production.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7".to_string(),
        )
        .unwrap();
        production_token.token_type = Some(TokenType::Client);
        production_token.status = TokenValidationStatus::Validated;
        token_cache.insert(production_token.token.clone(), production_token.clone());

        let post_req = test::TestRequest::post()
            .uri("/api/client/features")
            .insert_header(("Authorization", production_token.clone().token))
            .insert_header(ContentType::json())
            .to_request();

        let get_req = make_features_request_with_token(production_token.clone()).await;
        let get_res: ClientFeatures = test::call_and_read_body_json(&app, get_req).await;
        let post_res: ClientFeatures = test::call_and_read_body_json(&app, post_req).await;

        assert_eq!(get_res.features, post_res.features)
    }

    #[tokio::test]
    async fn client_features_endpoint_filters_on_project_access_in_token() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .service(web::scope("/api").service(get_features)),
        )
        .await;
        let mut edge_token = EdgeToken::try_from(
            "demo-app:production.03fa5f506428fe80ed5640c351c7232e38940814d2923b08f5c05fa7"
                .to_string(),
        )
        .unwrap();
        edge_token.token_type = Some(TokenType::Client);
        token_cache.insert(edge_token.token.clone(), edge_token.clone());
        let example_features = features_from_disk("../examples/features.json");
        features_cache.insert("production".into(), example_features.clone());
        let req = make_features_request_with_token(edge_token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features.len(), 5);
        assert!(res
            .features
            .iter()
            .all(|t| t.project == Some("demo-app".into())));
    }

    #[tokio::test]
    async fn client_features_endpoint_filters_when_multiple_projects_in_token() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .service(web::scope("/api").service(get_features)),
        )
        .await;
        let mut token =
            EdgeToken::try_from("[]:production.puff_the_magic_dragon".to_string()).unwrap();
        token.projects = vec!["dx".into(), "eg".into(), "unleash-cloud".into()];
        token.status = TokenValidationStatus::Validated;
        token.token_type = Some(TokenType::Client);
        token_cache.insert(token.token.clone(), token.clone());
        let example_features = features_from_disk("../examples/hostedexample.json");
        features_cache.insert("production".into(), example_features.clone());
        let req = make_features_request_with_token(token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features.len(), 24);
        assert!(res
            .features
            .iter()
            .all(|f| token.projects.contains(&f.project.clone().unwrap())));
    }

    #[tokio::test]
    async fn when_running_in_offline_mode_with_proxy_key_should_not_filter_features() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .app_data(Data::new(crate::cli::EdgeMode::Offline(OfflineArgs {
                    bootstrap_file: Some(PathBuf::from("../examples/features.json")),
                    tokens: vec!["secret_123".into()],
                })))
                .service(web::scope("/api").service(get_features)),
        )
        .await;
        let token = EdgeToken::offline_token("secret-123");
        token_cache.insert(token.token.clone(), token.clone());
        let example_features = features_from_disk("../examples/features.json");
        features_cache.insert(token.token.clone(), example_features.clone());
        let req = make_features_request_with_token(token.clone()).await;
        let res: ClientFeatures = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.features.len(), example_features.features.len());
    }

    #[tokio::test]
    async fn calling_client_features_endpoint_with_new_token_hydrates_from_upstream() {
        let upstream_features_cache: Arc<DashMap<String, ClientFeatures>> =
            Arc::new(DashMap::default());
        let upstream_token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let upstream_engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let server = upstream_server(
            upstream_token_cache.clone(),
            upstream_features_cache.clone(),
            upstream_engine_cache.clone(),
        )
        .await;
        let upstream_features = features_from_disk("../examples/hostedexample.json");
        let mut upstream_known_token = EdgeToken::from_str("dx:development.secret123").unwrap();
        upstream_known_token.status = TokenValidationStatus::Validated;
        upstream_known_token.token_type = Some(TokenType::Client);
        upstream_token_cache.insert(
            upstream_known_token.token.clone(),
            upstream_known_token.clone(),
        );
        upstream_features_cache.insert(cache_key(&upstream_known_token), upstream_features.clone());
        let unleash_client = Arc::new(UnleashClient::new(server.url("/").as_str(), None).unwrap());
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let feature_refresher = Arc::new(FeatureRefresher {
            unleash_client: unleash_client.clone(),
            tokens_to_refresh: Arc::new(Default::default()),
            features_cache: features_cache.clone(),
            engine_cache: engine_cache.clone(),
            refresh_interval: Duration::seconds(6000),
            persistence: None,
        });
        let token_validator = Arc::new(TokenValidator {
            unleash_client: unleash_client.clone(),
            token_cache: token_cache.clone(),
            persistence: None,
        });
        let local_app = test::init_service(
            App::new()
                .app_data(Data::from(token_validator.clone()))
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(engine_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .app_data(Data::from(feature_refresher.clone()))
                .wrap(middleware::as_async_middleware::as_async_middleware(
                    middleware::validate_token::validate_token,
                ))
                .service(web::scope("/api").configure(configure_client_api)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/client/features")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", upstream_known_token.token.clone()))
            .to_request();
        let res = test::call_service(&local_app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn gets_feature_by_name() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let features = features_from_disk("../examples/hostedexample.json");
        let mut dx_token = EdgeToken::from_str("dx:development.secret123").unwrap();
        dx_token.status = TokenValidationStatus::Validated;
        dx_token.token_type = Some(TokenType::Client);
        token_cache.insert(dx_token.token.clone(), dx_token.clone());
        features_cache.insert(cache_key(&dx_token), features.clone());
        let local_app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(engine_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .wrap(middleware::as_async_middleware::as_async_middleware(
                    middleware::validate_token::validate_token,
                ))
                .service(web::scope("/api").configure(configure_client_api)),
        )
        .await;
        let desired_toggle = "projectStatusApi";
        let request = test::TestRequest::get()
            .uri(format!("/api/client/features/{desired_toggle}").as_str())
            .insert_header(ContentType::json())
            .insert_header(("Authorization", dx_token.token.clone()))
            .to_request();
        let result: ClientFeature = test::call_and_read_body_json(&local_app, request).await;
        assert_eq!(result.name, desired_toggle);
    }

    #[tokio::test]
    pub async fn token_with_no_access_to_named_feature_yields_404() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let features = features_from_disk("../examples/hostedexample.json");
        let mut dx_token = EdgeToken::from_str("dx:development.secret123").unwrap();
        dx_token.status = TokenValidationStatus::Validated;
        dx_token.token_type = Some(TokenType::Client);
        token_cache.insert(dx_token.token.clone(), dx_token.clone());
        features_cache.insert(cache_key(&dx_token), features.clone());
        let local_app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(engine_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .wrap(middleware::as_async_middleware::as_async_middleware(
                    middleware::validate_token::validate_token,
                ))
                .service(web::scope("/api").configure(configure_client_api)),
        )
        .await;
        let desired_toggle = "serviceAccounts";
        let request = test::TestRequest::get()
            .uri(format!("/api/client/features/{desired_toggle}").as_str())
            .insert_header(ContentType::json())
            .insert_header(("Authorization", dx_token.token.clone()))
            .to_request();
        let result = test::call_service(&local_app, request).await;
        assert_eq!(result.status(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    pub async fn still_subsumes_tokens_after_moving_registration_to_initial_hydration() {
        let upstream_features_cache: Arc<DashMap<String, ClientFeatures>> =
            Arc::new(DashMap::default());
        let upstream_token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let upstream_engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let server = upstream_server(
            upstream_token_cache.clone(),
            upstream_features_cache.clone(),
            upstream_engine_cache.clone(),
        )
        .await;
        let upstream_features = features_from_disk("../examples/hostedexample.json");
        let mut upstream_dx_token = EdgeToken::from_str("dx:development.secret123").unwrap();
        upstream_dx_token.status = TokenValidationStatus::Validated;
        upstream_dx_token.token_type = Some(TokenType::Client);
        upstream_token_cache.insert(upstream_dx_token.token.clone(), upstream_dx_token.clone());
        let mut upstream_eg_token = EdgeToken::from_str("eg:development.secret321").unwrap();
        upstream_eg_token.status = TokenValidationStatus::Validated;
        upstream_eg_token.token_type = Some(TokenType::Client);
        upstream_token_cache.insert(upstream_eg_token.token.clone(), upstream_eg_token.clone());
        upstream_features_cache.insert(cache_key(&upstream_dx_token), upstream_features.clone());
        let unleash_client = Arc::new(UnleashClient::new(server.url("/").as_str(), None).unwrap());
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let feature_refresher = Arc::new(FeatureRefresher::new(
            unleash_client.clone(),
            features_cache.clone(),
            engine_cache.clone(),
            Duration::seconds(6000),
            None,
        ));
        let token_validator = Arc::new(TokenValidator {
            unleash_client: unleash_client.clone(),
            token_cache: token_cache.clone(),
            persistence: None,
        });
        let local_app = test::init_service(
            App::new()
                .app_data(Data::from(token_validator.clone()))
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(engine_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .app_data(Data::from(feature_refresher.clone()))
                .wrap(middleware::as_async_middleware::as_async_middleware(
                    middleware::validate_token::validate_token,
                ))
                .service(web::scope("/api").configure(configure_client_api)),
        )
        .await;
        let dx_req = test::TestRequest::get()
            .uri("/api/client/features")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", upstream_dx_token.token.clone()))
            .to_request();
        let res: ClientFeatures = test::call_and_read_body_json(&local_app, dx_req).await;
        assert!(!res.features.is_empty());
        let eg_req = test::TestRequest::get()
            .uri("/api/client/features")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", upstream_eg_token.token.clone()))
            .to_request();
        let eg_res: ClientFeatures = test::call_and_read_body_json(&local_app, eg_req).await;
        assert!(!eg_res.features.is_empty());
        assert_eq!(feature_refresher.tokens_to_refresh.len(), 2);
        assert_eq!(features_cache.len(), 1);
    }

    #[tokio::test]
    pub async fn can_filter_features_list_by_name_prefix() {
        let features_cache: Arc<DashMap<String, ClientFeatures>> = Arc::new(DashMap::default());
        let token_cache: Arc<DashMap<String, EdgeToken>> = Arc::new(DashMap::default());
        let engine_cache: Arc<DashMap<String, EngineState>> = Arc::new(DashMap::default());
        let features = features_from_disk("../examples/hostedexample.json");
        let mut dx_token = EdgeToken::from_str("dx:development.secret123").unwrap();
        dx_token.status = TokenValidationStatus::Validated;
        dx_token.token_type = Some(TokenType::Client);
        token_cache.insert(dx_token.token.clone(), dx_token.clone());
        features_cache.insert(cache_key(&dx_token), features.clone());
        let local_app = test::init_service(
            App::new()
                .app_data(Data::from(features_cache.clone()))
                .app_data(Data::from(engine_cache.clone()))
                .app_data(Data::from(token_cache.clone()))
                .wrap(middleware::as_async_middleware::as_async_middleware(
                    middleware::validate_token::validate_token,
                ))
                .service(web::scope("/api").configure(configure_client_api)),
        )
        .await;
        let request = test::TestRequest::get()
            .uri("/api/client/features?namePrefix=embed")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", dx_token.token.clone()))
            .to_request();
        let result: ClientFeatures = test::call_and_read_body_json(&local_app, request).await;
        assert_eq!(result.features.len(), 2);
        assert_eq!(result.query.unwrap().name_prefix.unwrap(), "embed");
    }
}
