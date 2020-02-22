#[derive(Debug, Deserialize, Serialize)]
pub struct InternalSettings {
    workflow: Workflow,
}

// On the first transition transition id is retrieved from the http API
// After that we store ids in the internal config file to avoid unecessary http calls
#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub wip: i32,
    pub done: i32,
    pub todo: i32,
}
