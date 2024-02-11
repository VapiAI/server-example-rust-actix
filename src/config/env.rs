use std::env;

pub struct EnvConfig {
    pub weather: WeatherConfig,
    pub openai: OpenaiConfig,
    pub vapi: VapiConfig,
}

pub struct WeatherConfig {
    pub base_url: String,
    pub api_key: String,
}

pub struct OpenaiConfig {
    pub api_key: String,
}

pub struct VapiConfig {
    pub base_url: String,
    pub api_key: String,
}

pub fn load_env_config() -> EnvConfig {
    EnvConfig {
        weather: WeatherConfig {
            base_url: env::var("WEATHER_BASE_URL")
                .unwrap_or_else(|_| "https://api.openweathermap.org/data/2.5".to_string()),
            api_key: env::var("WEATHER_API_KEY").unwrap_or_else(|_| "".to_string()),
        },
        openai: OpenaiConfig {
            api_key: env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        },
        vapi: VapiConfig {
            base_url: env::var("VAPI_BASE_URL")
                .unwrap_or_else(|_| "https://api.vapi.ai".to_string()),
            api_key: env::var("VAPI_API_KEY").unwrap_or_else(|_| "".to_string()),
        },
    }
}
