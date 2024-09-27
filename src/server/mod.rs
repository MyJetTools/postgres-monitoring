use app_ctx::AppContext;

mod app_ctx;
mod postgres;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppContext = {
       AppContext::new()
    };
}
