import Config

# Do not print debug messages in production
config :logger, level: :info

# Runtime production configuration, including reading
# of environment variables, is done on config/runtime.exs.
config :shader_api, ShaderApiWeb.Endpoint,
  url: [host: "shadergen-api.onrender.com", scheme: "https", port: 443],
  http: [ip: {0, 0, 0, 0}, port: String.to_integer(System.get_env("PORT") || "4000")],
  server: true,
  secret_key_base: System.get_env("SECRET_KEY_BASE")
