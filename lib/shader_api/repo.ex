defmodule ShaderApi.Repo do
  use Ecto.Repo,
    otp_app: :shader_api,
    adapter: Ecto.Adapters.Postgres
end
