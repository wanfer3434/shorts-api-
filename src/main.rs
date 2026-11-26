use axum::{routing::{get, post}, Json, Router, extract::State, http::Uri};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc, env};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Idea {
    id: String,
    personaje: String,
    lugar: String,
    accion: String,
    estado: String,
    prompt_final: String,
}

#[derive(Debug, Deserialize)]
struct NuevaIdea { personaje: String, lugar: String, accion: String }

type Cola = Arc<Mutex<VecDeque<Idea>>>;

#[tokio::main]
async fn main() {
    let cola: Cola = Arc::new(Mutex::new(VecDeque::new()));
    let app = Router::new()
        .route("/", get(|| async { "Shorts API OK" }))
        .route("/cola", post(crear_idea).get(listar))
        .route("/cola/pendiente", get(dar_trabajo))
        .fallback(|uri: Uri| async move {
            format!("Ruta no encontrada: {}", uri.path())
        })
        .with_state(cola);

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Shorts API en {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn crear_idea(State(cola): State<Cola>, Json(datos): Json<NuevaIdea>) -> Json<Idea> {
    let prompt = format!("Video vertical 9:16, estilo TikTok realista, camara en mano. {} {} en {}, gente sorprendida, iluminacion natural, 8 segundos.", datos.personaje, datos.accion, datos.lugar);
    let idea = Idea { id: Uuid::new_v4().to_string(), personaje: datos.personaje, lugar: datos.lugar, accion: datos.accion, estado: "pendiente".to_string(), prompt_final: prompt };
    cola.lock().await.push_back(idea.clone());
    Json(idea)
}

async fn dar_trabajo(State(cola): State<Cola>) -> Json<Option<Idea>> {
    let mut q = cola.lock().await;
    Json(q.pop_front())
}

async fn listar(State(cola): State<Cola>) -> Json<Vec<Idea>> {
    Json(cola.lock().await.iter().cloned().collect())
}