defmodule ShaderApiWeb.ShaderController do
  use ShaderApiWeb, :controller

  def create_vertex(conn, %{"prompt" => prompt}) do
    case ShaderApi.Groq.get_vertex_code(prompt) do
      {:ok, code} ->
        json(conn, %{vertex_code: code})
      {:error, error} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{error: error})
    end
  end

  def create_fragment(conn, %{"vertex_code" => vertex_code, "prompt" => prompt}) do
    case ShaderApi.Groq.get_fragment_code(vertex_code, prompt) do
      {:ok, code} ->
        json(conn, %{fragment_code: code})
      {:error, error} ->
        conn
        |> put_status(:unprocessable_entity)
        |> json(%{error: error})
    end
  end
end
