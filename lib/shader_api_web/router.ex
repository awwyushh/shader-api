defmodule ShaderApiWeb.Router do
  use ShaderApiWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", ShaderApiWeb do
    pipe_through :api

    post "/shaders/vertex", ShaderController, :create_vertex
    post "/shaders/fragment", ShaderController, :create_fragment
  end
end
