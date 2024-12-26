defmodule ShaderApi.Groq do
  use Rustler,
    otp_app: :shader_api,
    crate: :groq_api_rust,
    mode: :release

  # Function declarations
  def get_vertex_code(_input), do: :erlang.nif_error(:nif_not_loaded)
  def get_fragment_code(_vertex_code, _input), do: :erlang.nif_error(:nif_not_loaded)
end
