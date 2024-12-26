use dotenv::dotenv;
use groq_api_rust::{
    ChatCompletionMessage, ChatCompletionRequest, ChatCompletionRoles, GroqClient,
};
use rustler::{Env, Term, Atom};
use std::sync::Arc;
use std::error::Error;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        api_error,
        initialization_error,
        code_rejection
    }
}

#[derive(Clone)]
pub struct GroqClientResource {
    client: Arc<GroqClient>,
}

impl GroqClientResource {
    fn new(client: Arc<GroqClient>) -> Self {
        GroqClientResource { client }
    }
}

unsafe impl Send for GroqClientResource {}
unsafe impl Sync for GroqClientResource {}

rustler::init!(
    "Elixir.ShaderApi.Groq",
    [get_vertex_code, get_fragment_code],
    load = on_load
);

fn on_load(env: Env, _: Term) -> bool {
    rustler::resource!(GroqClientResource, env);
    true
}

fn init_client() -> Result<GroqClient, Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").map_err(|e| format!("API_KEY not found: {}", e))?;
    Ok(GroqClient::new(api_key, None))
}

#[rustler::nif]
pub fn get_vertex_code(input: String) -> Result<(Atom, String), rustler::Error> {
     // Safely initialize the client
    let client = match init_client() {
        Ok(client) => Arc::new(client),
        Err(e) => return Ok((atoms::initialization_error(), e.to_string()))
    };
    
    let client_ref = GroqClientResource::new(client);
    
    match get_vertex_code_internal(&client_ref, input) {
        Ok(code) => Ok((atoms::ok(), code)),
        Err(e) => Ok((atoms::api_error(), e.to_string()))
    }
}

#[rustler::nif]
pub fn get_fragment_code(vertex_code: String, input: String) -> Result<(Atom, String), rustler::Error> {
     // Safely initialize the client
    let client = match init_client() {
        Ok(client) => Arc::new(client),
        Err(e) => return Ok((atoms::initialization_error(), e.to_string()))
    };
    
    let client_ref = GroqClientResource::new(client);
    
    match get_fragment_code_internal(&client_ref, vertex_code, input) {
        Ok(code) => Ok((atoms::ok(), code)),
        Err(e) => Ok((atoms::api_error(), e.to_string()))
    }
}

fn get_vertex_code_internal(
    client_ref: &GroqClientResource,
    input: String
) -> Result<String, Box<dyn Error>> {
     let input = format!(
        "Generate a simple WebGL vertex shader code, written in GLSL ES. \
        The shader must have an input attribute named `position` of type `vec2`. The shader should also have a `main` function which sets the `gl_Position` correctly, this can be set directly as `gl_Position = vec4(position, 0.0, 1.0);`. Use `attribute` storage qualifier for the `position` input attribute. Do not use any variable names starting with `gl_`. Do not use any for loops or while loops in the generated code. Do not give extra comments or any ``` delimiters, just the raw shader code. \
        Generate a vertex shader that does - {}. Here is an example shader: \
        ```glsl\nattribute vec2 position;\n void main() {{\n gl_Position = vec4(position, 0.0, 1.0);\n}}\n```\
        Ensure no extra punctuations, and no extra messages other than the shader code.
        no 'here is your code' type of statements , just and just the code",
        input
    );

    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionRoles::User,
        content: input,
        name: None,
    }];

    let request = ChatCompletionRequest::new("llama3-70b-8192", messages);

    match client_ref.client.chat_completion(request) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
               let code = choice.message.content.clone();
                if code.contains("while") {
                    return Err("Generated code contains 'while' loop which is not allowed".into());
                }
                Ok(code)
            } else {
                Err("Empty response from API".into())
            }
        }
        Err(e) => Err(format!("API Error: {}", e).into())
    }
}

fn get_fragment_code_internal(
    client_ref: &GroqClientResource,
    vertex_code: String,
    input: String
) -> Result<String, Box<dyn Error>> {
     let input = format!(
        "Generate a simple WebGL fragment shader code in GLSL ES, that complements the following vertex shader: \n\
         ```glsl\n{}\n```\n. \
         The fragment shader must use a uniform `u_time` of type `float` and `u_resolution` of type `vec2`. The fragment shader should also have a main function that sets `gl_FragColor` correctly, use `gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);` as a basic example. Do not use any variable names starting with `gl_` use `gl_FragColor` as the fragment color output. Use the storage qualifier `uniform` for `u_time` and `u_resolution`, and also add the `highp` precision qualifier to `float` types and `vec2` types. Do not use any for loops or while loops in the generated code. Do not give extra comments or any ``` delimiters, just the raw shader code. \
         Create a fragment shader that does - {}. Here is an example shader: \
        ```glsl\n uniform highp float u_time;\n uniform highp vec2 u_resolution; void main() {{\n  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);\n}}\n```\
         Ensure no extra punctuations, and no extra messages other than the shader code.
         no 'here is your code' type of statements , just and just the code",
        vertex_code, input
    );

    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionRoles::User,
        content: input,
        name: None,
    }];

    let request = ChatCompletionRequest::new("llama3-70b-8192", messages);

    match client_ref.client.chat_completion(request) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                 let code = choice.message.content.clone();
                if code.contains("while") {
                    return Err("Generated code contains 'while' loop which is not allowed".into());
                }
                 Ok(code)
            } else {
                Err("Empty response from API".into())
            }
        }
       Err(e) => Err(format!("API Error: {}", e).into())
    }
}