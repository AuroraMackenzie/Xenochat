use std::path::PathBuf;

use xenochat_api::{
    ApiService, Route,
    server::{build_router, serve},
};
use xenochat_common::{config::XenochatConfig, version::VERSION};
use xenochat_core::{
    Collaborator, CompletionRequest, KeywordTrigger, ModelProvider, Tool, ToolCall, ToolOutcome,
};
use xenochat_gpu::GpuProbe;

use xenochat_discord::DiscordAdapter;
use xenochat_googlechat::GoogleChatAdapter;
use xenochat_imessage::IMessageAdapter;
use xenochat_instagram::InstagramAdapter;
use xenochat_kakaotalk::KakaoTalkAdapter;
use xenochat_line::LineAdapter;
use xenochat_messenger::MessengerAdapter;
use xenochat_qq::QqAdapter;
use xenochat_signal::SignalAdapter;
use xenochat_skype::SkypeAdapter;
use xenochat_slack::SlackAdapter;
use xenochat_teams::TeamsAdapter;
use xenochat_telegram::TelegramAdapter;
use xenochat_viber::ViberAdapter;
use xenochat_wechat::WeChatAdapter;
use xenochat_whatsapp::WhatsAppAdapter;
use xenochat_zoom::ZoomAdapter;

struct LocalStubModel;

impl ModelProvider for LocalStubModel {
    fn name(&self) -> &'static str {
        "local-stub"
    }

    fn complete(&self, request: &CompletionRequest) -> String {
        format!("processed:{}", request.prompt)
    }
}

struct HealthTool;

impl Tool for HealthTool {
    fn name(&self) -> &'static str {
        "health-check"
    }

    fn call(&self, input: &str) -> ToolOutcome {
        ToolOutcome {
            ok: true,
            output: format!("ok:{input}"),
        }
    }
}

#[tokio::main]
async fn main() {
    let config = load_runtime_config();
    if let Err(error) = config.validate() {
        eprintln!("configuration validation failed: {error:?}");
        std::process::exit(1);
    }

    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "preview".to_owned());

    match command.as_str() {
        "serve" => {
            let _router = build_router(config.clone());
            println!(
                "starting xenochat api at {}:{}",
                config.api.host, config.api.port
            );
            if let Err(error) = serve(config).await {
                eprintln!("api server failed: {error}");
                std::process::exit(1);
            }
        }
        "preview" => {
            run_preview(config);
        }
        other => {
            eprintln!("unknown command: {other}");
            eprintln!("usage: xenochat [preview|serve]");
            std::process::exit(2);
        }
    }
}

fn load_runtime_config() -> XenochatConfig {
    if let Ok(path) = std::env::var("XENOCHAT_CONFIG") {
        let parsed = XenochatConfig::from_toml_file(&PathBuf::from(path));
        if let Ok(config) = parsed {
            return config;
        }
    }

    let default_path = PathBuf::from("configs/xenochat.toml");
    if default_path.exists() {
        let parsed = XenochatConfig::from_toml_file(&default_path);
        if let Ok(config) = parsed {
            return config;
        }
    }

    XenochatConfig::default()
}

fn run_preview(config: XenochatConfig) {
    let mut api = ApiService::new(config.clone());
    api.register_adapter(TelegramAdapter::default());
    api.register_adapter(DiscordAdapter::default());
    api.register_adapter(SlackAdapter::default());
    api.register_adapter(TeamsAdapter::default());
    api.register_adapter(GoogleChatAdapter::default());
    api.register_adapter(IMessageAdapter::default());
    api.register_adapter(InstagramAdapter::default());
    api.register_adapter(KakaoTalkAdapter::default());
    api.register_adapter(LineAdapter::default());
    api.register_adapter(MessengerAdapter::default());
    api.register_adapter(QqAdapter::default());
    api.register_adapter(SignalAdapter::default());
    api.register_adapter(SkypeAdapter::default());
    api.register_adapter(ViberAdapter::default());
    api.register_adapter(WeChatAdapter::default());
    api.register_adapter(WhatsAppAdapter::default());
    api.register_adapter(ZoomAdapter::default());
    api.handle_route(Route::Health);

    let mut collaborator = Collaborator::default();
    collaborator.register(LocalStubModel);
    collaborator.tools_mut().register(HealthTool);
    let preview = collaborator.respond(&CompletionRequest {
        prompt: "startup check".to_owned(),
    });
    let tool_output = collaborator.call_tool(&ToolCall {
        name: "health-check".to_owned(),
        input: "boot".to_owned(),
    });

    let mut trigger = KeywordTrigger::default();
    trigger.register("deploy", "Release command detected");
    let trigger_status = trigger
        .check("deploy build now")
        .map(|rule| rule.response_template.clone())
        .unwrap_or_else(|| "no keyword match".to_owned());

    let probe = GpuProbe::detect();

    println!("xenochat version: {VERSION}");
    println!(
        "api host: {}:{}",
        api.config().api.host,
        api.config().api.port
    );
    println!("startup model output: {preview}");
    println!("tool output: {tool_output}");
    println!("trigger status: {trigger_status}");
    println!("gpu backend: {:?}", probe.backend);
    println!("gpu details: {}", probe.details);
}
