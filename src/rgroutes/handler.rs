use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use rocket::http::{Status, ContentType};
use rocket::serde::json::{Json};
use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams},
    Client, CustomResource,
};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CreateRgrouteRequest {
    pub namespace: String,
    pub name: String,
    pub domain: String,
    pub service_host:  String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdateRgrouteRequest {
    pub namespace: String,
    pub name: String,
    pub domain: String,
    pub service_host:  String,
}

// crd: rginx.hedui.com/v1 Kind: RGRoute
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(group = "rginx.hedui.com", version = "v1", kind = "RGRoute", namespaced)]
pub struct RGRouteSpec {
    domain: String,
    serviceHost: String,
}

#[get("/rgroutes/namespace/<namespace>")]
pub async fn list_rgroute(namespace: &str) -> (Status,(ContentType,String)) {
    let client = Client::try_default().await.unwrap();
    let rgroute_api:Api<RGRoute>=Api::namespaced(client, namespace);
    let rgroute_list = rgroute_api.list(&ListParams::default()).await.unwrap();
    (Status::Ok,(ContentType::JSON,serde_json::to_string(&rgroute_list).unwrap()))
}

#[get("/rgroutes/namespace/<namespace>/name/<name>")]
pub async fn get_rgroute(namespace: &str, name: &str) -> (Status,(ContentType,String)) {
    let client = Client::try_default().await.unwrap();
    let rgroute_api:Api<RGRoute>=Api::namespaced(client, namespace);
    let rgroute = rgroute_api.get(name).await.unwrap();
    (Status::Ok,(ContentType::JSON,serde_json::to_string(&rgroute).unwrap()))
}

#[post("/rgroutes", format = "json", data="<req>")]
pub async fn create_rgroute(req: Json<CreateRgrouteRequest>) -> (Status,(ContentType,String)) {
    let client = Client::try_default().await.unwrap();
    let rgroute_api:Api<RGRoute>=Api::namespaced(client, req.namespace.as_str());

    let p: RGRoute = serde_json::from_value(json!({
        "apiVersion": "rginx.hedui.com/v1",
        "kind": "RGRoute",
        "metadata": { "name": req.name },
        "spec": {
            "domain": req.domain,
            "serviceHost": req.service_host,
        }
    })).unwrap();

    let ret = rgroute_api.create(&PostParams::default(),&p).await.unwrap();

    (Status::Ok,(ContentType::JSON,serde_json::to_string(&ret).unwrap()))

}

#[delete("/rgroutes/namespace/<namespace>/name/<name>")]
pub async fn delete_rgroute(namespace: &str, name: &str) -> (Status,(ContentType,String)) {
    let client = Client::try_default().await.unwrap();
    let rgroute_api:Api<RGRoute>=Api::namespaced(client, namespace);
    let dp = DeleteParams::default();
    let ret = rgroute_api.delete(name, &dp).await.unwrap();
    (Status::Ok,(ContentType::JSON,serde_json::to_string(&String::from("success")).unwrap()))
}

#[put("/rgroutes", format = "json", data="<req>")]
pub async fn update_rgroute(req: Json<UpdateRgrouteRequest>) -> (Status,(ContentType,String)) {
    let client = Client::try_default().await.unwrap();
    let rgroute_api:Api<RGRoute>=Api::namespaced(client, req.namespace.as_str());

    let patch = json!({
        "spec": {
            "domain": req.domain,
            "serviceHost": req.service_host
        }
    });
    let patchparams = PatchParams::default();
    let p_patched = rgroute_api.patch(&req.name, &patchparams, &Patch::Merge(&patch)).await.unwrap();
    (Status::Ok,(ContentType::JSON,serde_json::to_string(&p_patched).unwrap()))
}
