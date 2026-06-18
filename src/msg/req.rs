
// con cargo add serde -F derive poi a quel punto non ci servirà più Codec
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Req {
    GetPeers,
}


// dobbiamo fare un Codec per dire come seriealizzare e deserializzare le nostree struct diventando byte