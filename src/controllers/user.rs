use interface::CurrentResponse;
use loco_rs::prelude::*;

use crate::models::_entities::users;

async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Json<CurrentResponse>> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let resp = CurrentResponse {
        pid: user.pid.to_string(),
        name: user.name.clone(),
        email: user.email.clone(),
    };
    format::json(resp)
}

pub fn routes() -> Routes {
    Routes::new().prefix("user").add("/current", get(current))
}
