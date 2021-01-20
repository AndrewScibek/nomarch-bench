use goose::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[cfg(feature = "json")]
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    pub pipeline: String,
    pub service: String,
    pub events: Vec<String>,
}

fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        // In this example, we only create a single taskset, named "WebsiteUser".
        .register_taskset(
            taskset!("push_events")
                .register_task(task!(nomarch_event_pipeline_1_all_steps).set_name("Pipeline 1 Complete"))
                .register_task(task!(nomarch_event_pipeline_1_missing_step_1).set_name("Pipeline 1 Missing"))
                .register_task(task!(nomarch_event_pipeline_2_all_steps).set_name("Pipeline 2 Complete"))
                .register_task(task!(nomarch_event_pipeline_2_missing_step_1).set_name("Pipeline 2 Missing"))
        )
        .set_default(GooseDefault::Host, "http://localhost:8080")?
        .set_default(GooseDefault::Users, 2000)?
        .set_default(GooseDefault::HatchRate, "100")?
        .set_default(GooseDefault::StatusCodes, true)?
        .execute()?
        .print();

    Ok(())
}


async fn nomarch_event_pipeline_1_all_steps(user: &GooseUser) -> GooseTaskResult {
    let tmp = Uuid::new_v4();
    let uuid = tmp.to_string();
    let mut request_builder = user.goose_post("/events").await?;
    let first: Pipeline = Pipeline{
        pipeline: "pipeline1".into(),
        service: "step1".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&first) {
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }

    let second: Pipeline = Pipeline{
        pipeline: "pipeline1".into(),
        service: "step2".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&second) {
        request_builder = user.goose_post("/events").await?;
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }
    Ok(())
}

async fn nomarch_event_pipeline_1_missing_step_1(user: &GooseUser) -> GooseTaskResult {
    let uuid = Uuid::new_v4();
    let request_builder = user.goose_post("/events").await?;
    let req: Pipeline = Pipeline{
        pipeline: "pipeline1".into(),
        service: "step2".into(),
        events: vec!(uuid.to_string())
    };
    let _goose = user.goose_send(request_builder.json(&req), None).await?;
    Ok(())
}

async fn nomarch_event_pipeline_2_all_steps(user: &GooseUser) -> GooseTaskResult {
    let tmp = Uuid::new_v4();
    let uuid = tmp.to_string();
    let mut request_builder = user.goose_post("/events").await?;
    let second: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step2".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&second) {
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }

    let third: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step3".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&third) {
        request_builder = user.goose_post("/events").await?;
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }
    
    let foura: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step4a".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&foura) {
        request_builder = user.goose_post("/events").await?;
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }
    
    let fourb: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step4b".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&fourb) {
        request_builder = user.goose_post("/events").await?;
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }
    
    let fifth: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step5".into(),
        events: vec!(uuid.clone())
    };
    if let Ok(body) = serde_json::to_value(&fifth) {
        request_builder = user.goose_post("/events").await?;
        let _goose = user.goose_send(request_builder.json(&body), None).await?;
    }
    Ok(())
}

async fn nomarch_event_pipeline_2_missing_step_1(user: &GooseUser) -> GooseTaskResult {
    let uuid = Uuid::new_v4();
    let request_builder = user.goose_post("/events").await?;
    let req: Pipeline = Pipeline{
        pipeline: "pipeline2".into(),
        service: "step2".into(),
        events: vec!(uuid.to_string())
    };
    let _goose = user.goose_send(request_builder.json(&req), None).await?;
    Ok(())
}