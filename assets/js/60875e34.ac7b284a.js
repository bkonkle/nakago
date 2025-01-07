"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[661],{7315:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>o,default:()=>h,frontMatter:()=>a,metadata:()=>r,toc:()=>d});var s=t(4848),i=t(8453);const a={sidebar_position:2},o="Tutorial",r={id:"tutorial",title:"Tutorial",description:"This tutorial will walk you through the basics of using Nakago to build a simple HTTP service. It will use Axum to provide HTTP routes and will decode the user's JWT token and verify their identity via a separate OAuth2 provider, such as Auth0 or Okta or your own self-hosted service.",source:"@site/docs/tutorial.md",sourceDirName:".",slug:"/tutorial",permalink:"/docs/tutorial",draft:!1,unlisted:!1,editUrl:"https://github.com/bkonkle/nakago/tree/main/website/docs/tutorial.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"documentationSidebar",previous:{title:"Welcome to Nakago",permalink:"/docs/intro"},next:{title:"Features",permalink:"/docs/category/features"}},l={},d=[{value:"Cargo-Generate Template",id:"cargo-generate-template",level:2},{value:"Setup",id:"setup",level:2},{value:"Authentication",id:"authentication",level:2},{value:"Auth Config",id:"auth-config",level:3},{value:"Initialization",id:"initialization",level:3},{value:"Axum Route",id:"axum-route",level:3},{value:"Running the App",id:"running-the-app",level:3},{value:"Integration Testing",id:"integration-testing",level:2},{value:"Test Utils",id:"test-utils",level:3},{value:"HTTP Calls",id:"http-calls",level:3},{value:"Running the Tests",id:"running-the-tests",level:3},{value:"Finished Result",id:"finished-result",level:2}];function c(e){const n={a:"a",code:"code",em:"em",h1:"h1",h2:"h2",h3:"h3",header:"header",p:"p",pre:"pre",...(0,i.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.header,{children:(0,s.jsx)(n.h1,{id:"tutorial",children:"Tutorial"})}),"\n",(0,s.jsx)(n.p,{children:"This tutorial will walk you through the basics of using Nakago to build a simple HTTP service. It will use Axum to provide HTTP routes and will decode the user's JWT token and verify their identity via a separate OAuth2 provider, such as Auth0 or Okta or your own self-hosted service."}),"\n",(0,s.jsx)(n.h2,{id:"cargo-generate-template",children:"Cargo-Generate Template"}),"\n",(0,s.jsxs)(n.p,{children:["First install ",(0,s.jsx)(n.code,{children:"cargo-generate"}),":"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"cargo install cargo-generate\n"})}),"\n",(0,s.jsx)(n.p,{children:"Then generate a new project with this template:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"cargo generate bkonkle/nakago-simple-template\n"})}),"\n",(0,s.jsx)(n.p,{children:"You'll see a folder structure like this:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-text",children:"simple/\n\u251c\u2500 .cargo/ -- Clippy config\n\u251c\u2500 .github/ -- Github Actions\n\u251c\u2500 config/ -- Config files for different environments\n\u251c\u2500 src/\n\u2502  \u251c\u2500 http/ -- Axum HTTP routes\n\u2502  \u2502  \u251c\u2500 mod.rs\n\u2502  \u2502  \u251c\u2500 health.rs -- The HTTP health check handler\n\u2502  \u2502  \u2514\u2500 router.rs -- Axum router initialization\n\u2502  \u251c\u2500 lib.rs\n\u2502  \u251c\u2500 config.rs -- Your app's custom Config struct\n\u2502  \u251c\u2500 init.rs -- App initialization\n\u2502  \u2514\u2500 main.rs -- Main entry point\n\u251c\u2500 Cargo.toml\n\u251c\u2500 Makefile.toml\n\u251c\u2500 README.md\n\u2514\u2500 // ...\n"})}),"\n",(0,s.jsxs)(n.p,{children:["This includes a simple app-specific ",(0,s.jsx)(n.code,{children:"Config"})," struct with an embedded ",(0,s.jsx)(n.code,{children:"http"})," config provided by the ",(0,s.jsx)(n.code,{children:"nakago-axum"})," library. You can add your own configuration fields to this struct and they'll be populated by the ",(0,s.jsx)(n.a,{href:"https://docs.rs/figment/latest/figment/",children:"figment"})," crate using the ",(0,s.jsx)(n.a,{href:"https://github.com/bkonkle/nakago/tree/main/nakago_figment",children:"nakago-figment"})," library."]}),"\n",(0,s.jsxs)(n.p,{children:["It includes a barebones ",(0,s.jsx)(n.code,{children:"init::app()"})," function that will load your configuration and initialize your dependencies. You can add your own dependencies to this function and they'll be available when you build your Axum routes through a convenient ",(0,s.jsx)(n.code,{children:"State"})," struct that contains the injection container."]}),"\n",(0,s.jsxs)(n.p,{children:["The ",(0,s.jsx)(n.code,{children:"main.rs"})," file uses ",(0,s.jsx)(n.a,{href:"https://docs.rs/pico-args/0.5.0/pico_args/",children:"pico-args"})," to parse a simple command-line argument to specify an alternate config path, which is useful for many deployment scenarios that dynamically map a config file to a certain mount point within a container filesystem."]}),"\n",(0,s.jsxs)(n.p,{children:["In the ",(0,s.jsx)(n.code,{children:"http/"})," folder, you'll find an Axum handler and a router initialization function. The router maps a simple ",(0,s.jsx)(n.code,{children:"GET /health"})," route to a handler that returns a JSON response with a success message."]}),"\n",(0,s.jsx)(n.p,{children:"You now have a simple foundation to build on. Let's add some more functionality!"}),"\n",(0,s.jsx)(n.h2,{id:"setup",children:"Setup"}),"\n",(0,s.jsxs)(n.p,{children:["Follow the Installation instructions in the ",(0,s.jsx)(n.code,{children:"README.md"})," to prepare your new local environment."]}),"\n",(0,s.jsx)(n.h2,{id:"authentication",children:"Authentication"}),"\n",(0,s.jsx)(n.p,{children:"One of the first things you'll probably want to add to your application is authentication, which establishes the user's identity. This is separate and distinct from authorization, which determines what the user is allowed to do."}),"\n",(0,s.jsxs)(n.p,{children:["The only currently supported method of authentication is through JWT with JWKS keys, though other methods will be added in the future. The ",(0,s.jsx)(n.code,{children:"nakago-axum"})," library provides a request extractor for for Axum that uses ",(0,s.jsx)(n.a,{href:"https://docs.rs/biscuit/0.6.0/biscuit/",children:"biscuit"})," with your Figment Config to decode a JWT from the ",(0,s.jsx)(n.code,{children:"Authorization"})," header, validate it with a JWKS key from the ",(0,s.jsx)(n.code,{children:"/.well-known/jwks.json"})," path on the auth url, and then return the value of the ",(0,s.jsx)(n.code,{children:"sub"})," claim from the payload."]}),"\n",(0,s.jsx)(n.p,{children:(0,s.jsx)(n.em,{children:"Configurable claims and other authentication methods will be added in the future."})}),"\n",(0,s.jsx)(n.h3,{id:"auth-config",children:"Auth Config"}),"\n",(0,s.jsxs)(n.p,{children:["In your ",(0,s.jsx)(n.code,{children:"config.rs"})," file, add a new property to the app's ",(0,s.jsx)(n.code,{children:"Config"})," struct:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use nakago_axum::auth;\n// ...\n\n/// Server Config\n#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]\npub struct Config {\n    /// HTTP config\n    pub http: nakago_axum::Config,\n\n    /// HTTP Auth Config\n    pub auth: auth::Config,\n}\n"})}),"\n",(0,s.jsxs)(n.p,{children:["This auth ",(0,s.jsx)(n.code,{children:"Config"})," is automatically loaded as part of the default config loaders in the ",(0,s.jsx)(n.code,{children:"nakago-axum"})," crate, which you'll see below."]}),"\n",(0,s.jsxs)(n.p,{children:["Next, add the following values to your ",(0,s.jsx)(n.code,{children:"config.local.toml.example"})," file as a hint, so that new developers know they need to reach out to you for real values when they create their own ",(0,s.jsx)(n.code,{children:"config.local.toml"})," file:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-toml",children:'[auth]\nurl = "https://simple-dev.oauth-service.com"\naudience = "localhost"\n\n[auth.client]\nid = "client_id"\nsecret = "client_secret"\n'})}),"\n",(0,s.jsxs)(n.p,{children:["Add the real details to your own ",(0,s.jsx)(n.code,{children:"config.toml"})," file, which should be excluded from git via the ",(0,s.jsx)(n.code,{children:".gitignore"})," file. If you don't have real values yet, leave them as the dummy values above. You can still run integration tests without having a real OAuth2 provider running, if you want."]}),"\n",(0,s.jsx)(n.h3,{id:"initialization",children:"Initialization"}),"\n",(0,s.jsx)(n.p,{children:"You're now ready to head over to your initialization routine. This is where you will provide all of the dependencies and setup your app needs in order to run."}),"\n",(0,s.jsxs)(n.p,{children:["This block that is already in the top-level ",(0,s.jsx)(n.code,{children:"init.rs"})," ensures your config is populated from environment variables or the currently chosen config file, along with the auth property you added above:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"// These lines should already be in your `init.rs` file - no change needed\nnakago_figment::Init::<Config>::default()\n    .maybe_with_path(config_path)\n    .init(&i)\n    .await?;\n"})}),"\n",(0,s.jsxs)(n.p,{children:["First, add the default JWKS Validator from ",(0,s.jsx)(n.code,{children:"nakago_axum"}),"'s ",(0,s.jsx)(n.code,{children:"auth"})," module using the ",(0,s.jsx)(n.code,{children:"provide"})," method, which uses the type as the key for the Inject container. Add this to your ",(0,s.jsx)(n.code,{children:"init.rs"})," file, within the ",(0,s.jsx)(n.code,{children:"app()"})," function:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use nakago_axum::auth::{validator, Validator};\n\n// ...\n\ni.provide::<Box<dyn Validator>>(validator::Provide::default())\n        .await?;\n"})}),"\n",(0,s.jsxs)(n.p,{children:["This will be overridden in your tests to use the unverified variant, but we'll get to that later. Next you should use ",(0,s.jsx)(n.code,{children:"jwks::Provide"})," to inject the JWKS config. Add thios to your ",(0,s.jsx)(n.code,{children:"init.rs"})," file as well:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use nakago_axum::auth::{jwks, JWKSet, Empty};\n\n// ...\n\ni.provide::<JWKSet<Empty>>(jwks::Provide::<Config>::default())\n        .await?;\n"})}),"\n",(0,s.jsxs)(n.p,{children:["Your ",(0,s.jsx)(n.code,{children:"init.rs"})," should now look like this:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use std::path::PathBuf;\n\nuse nakago::{Inject, Result};\nuse nakago_axum::{\n    auth::{jwks, validator, Empty, JWKSet, Validator},\n    config,\n};\n\nuse crate::config::Config;\n\n/// Create a dependency injection container for the top-level application\npub async fn app(config_path: Option<PathBuf>) -> Result<Inject> {\n    let i = Inject::default();\n\n    i.provide::<Box<dyn Validator>>(validator::Provide::default())\n        .await?;\n\n    i.provide::<JWKSet<Empty>>(jwks::Provide::<Config>::default())\n        .await?;\n\n    // Add config loaders before the Config is initialized\n    config::add_default_loaders(&i).await?;\n\n    // Initialize the Config\n    nakago_figment::Init::<Config>::default()\n        .maybe_with_path(config_path)\n        .init(&i)\n        .await?;\n\n    Ok(i)\n}\n"})}),"\n",(0,s.jsx)(n.h3,{id:"axum-route",children:"Axum Route"}),"\n",(0,s.jsxs)(n.p,{children:["You can now add a quick handler to ",(0,s.jsx)(n.code,{children:"http/"})," that allows a user to view their own username when logged in. Create a new file called ",(0,s.jsx)(n.code,{children:"http/user.rs"}),":"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'use axum::Json;\nuse nakago_axum::auth::Subject;\nuse serde_derive::{Deserialize, Serialize};\n\n/// A Username Response\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct UsernameResponse {\n    /// The Status code\n    code: usize,\n\n    /// The username, or "(anonymous)"\n    username: String,\n}\n\n/// Handle Get Username requests\npub async fn get_username(sub: Subject) -> Json<UsernameResponse> {\n    let username = if let Subject(Some(username)) = sub {\n        username.clone()\n    } else {\n        "(anonymous)".to_string()\n    };\n\n    Json(UsernameResponse {\n        code: 200,\n        username,\n    })\n}\n'})}),"\n",(0,s.jsxs)(n.p,{children:["Make sure to add the ",(0,s.jsx)(n.code,{children:"user.rs"})," file to your ",(0,s.jsx)(n.code,{children:"http/mod.rs"})," file:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"/// User handlers\npub mod user;\n"})}),"\n",(0,s.jsxs)(n.p,{children:["The ",(0,s.jsx)(n.code,{children:"Subject"})," extension uses Nakago Axum's State proivider to find the Inject container, which it then uses to grab the JWT config and the Validator instance. It decodes the JWT and returns the ",(0,s.jsx)(n.code,{children:"sub"})," claim from the payload. If the user is not logged in, the ",(0,s.jsx)(n.code,{children:"Subject"})," will contain a ",(0,s.jsx)(n.code,{children:"None"}),"."]}),"\n",(0,s.jsxs)(n.p,{children:["Now add a route that uses the handler to the Init hook at ",(0,s.jsx)(n.code,{children:"http/router.rs"}),":"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'use super::{health, user};\n\n// ...\n\nRouter::new()\n    // ...\n    .route("/username", get(user::get_username))\n'})}),"\n",(0,s.jsxs)(n.p,{children:["Your ",(0,s.jsx)(n.code,{children:"http/router.rs"})," file should now look like this:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'/// This method should already exist in your `http/router.rs` file\npub fn init(i: &Inject) -> Router {\n    Router::new()\n        .layer(trace_layer())\n        .route("/health", get(health::health_check))\n        .route("/username", get(user::get_username)) // <-- add this line\n        .with_state(State::new(i.clone()))\n}\n'})}),"\n",(0,s.jsx)(n.h3,{id:"running-the-app",children:"Running the App"}),"\n",(0,s.jsxs)(n.p,{children:["At this point, you can run your app and see the ",(0,s.jsx)(n.code,{children:"(anonymous)"})," response at the ",(0,s.jsx)(n.code,{children:"GET /username"})," endpoint:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"cargo make run\n"})}),"\n",(0,s.jsxs)(n.p,{children:["The uses cargo-make, a tool to provide enhanced Makefile-like functionality for Rust projects. You can see the configuration in the ",(0,s.jsx)(n.code,{children:"Makefile.toml"})," file."]}),"\n",(0,s.jsx)(n.p,{children:"At first, you'll see a big ugly traceback with the following error message at the top because you don't have a valid autd provider configured:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"thread '<unnamed>' panicked at 'Unable to retrieve JWKS: invalid format'\n"})}),"\n",(0,s.jsxs)(n.p,{children:['This is okay - you don\'t have to have a properly configured auth provider to run the integration tests for your app. You can use the "unverified" ',(0,s.jsx)(n.code,{children:"AuthState"})," variant during integration testing, and skip the rest of this section."]}),"\n",(0,s.jsxs)(n.p,{children:["If you ",(0,s.jsx)(n.em,{children:"do"})," have a valid OAuth2 provider, then you'll want to create a ",(0,s.jsx)(n.code,{children:"config.local.toml"})," file and set the following property in it:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-toml",children:'[auth]\nurl = "https://simple-dev.oauth-service.com"\n'})}),"\n",(0,s.jsxs)(n.p,{children:["You can also use the ",(0,s.jsx)(n.code,{children:"AUTH_URL"})," environment variable to set this value. Consider using a tool like ",(0,s.jsx)(n.a,{href:"https://direnv.net/",children:"direnv"})," to manage variables like this in your local development environment with ",(0,s.jsx)(n.code,{children:".envrc"})," files."]}),"\n",(0,s.jsxs)(n.p,{children:["Your provider should have a ",(0,s.jsx)(n.code,{children:"/.well-known/jwks.json"})," file available at the given auth url, which will avoid the error message above. You should now see output that looks like the following:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"2023-09-08T02:14:03.388670Z  INFO simple: Started on port: 8000\n"})}),"\n",(0,s.jsxs)(n.p,{children:["When you call ",(0,s.jsx)(n.code,{children:"http://localhost:8000/username"})," in your browser, you should see the following response:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-json",children:'{\n  "code": 200,\n  "username": "(anonymous)"\n}\n'})}),"\n",(0,s.jsx)(n.h2,{id:"integration-testing",children:"Integration Testing"}),"\n",(0,s.jsxs)(n.p,{children:["Now that you have a simple route that requires authentication, you'll want to add some integration tests to ensure that it works as expected. You don't actually need to have an OAuth2 provider running to test this, because the ",(0,s.jsx)(n.code,{children:"nakago-axum"})," library provides a mock unverified ",(0,s.jsx)(n.code,{children:"AuthState"})," that you can use to simulate a logged-in user."]}),"\n",(0,s.jsx)(n.h3,{id:"test-utils",children:"Test Utils"}),"\n",(0,s.jsxs)(n.p,{children:["Nakago Axum's HTTP ",(0,s.jsx)(n.code,{children:"Utils"})," class is based on the idea of extending the base test ",(0,s.jsx)(n.code,{children:"Utils"})," class you'll find in ",(0,s.jsx)(n.code,{children:"nakago_axum::test::Utils"})," with additional functionality, like adding a ",(0,s.jsx)(n.code,{children:"graphql"})," property if you're using ",(0,s.jsx)(n.code,{children:"nakago-async-graphql"})," or adding convenience methods around your app-specific data."]}),"\n",(0,s.jsxs)(n.p,{children:["To start out with, create a ",(0,s.jsx)(n.code,{children:"tests"})," folder alongside your ",(0,s.jsx)(n.code,{children:"src"}),". This will be used by Cargo as an ",(0,s.jsx)(n.a,{href:"https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests",children:'"integration test"'})," module, and will be excluded from your final binary. It allows you to import the module in your ",(0,s.jsx)(n.code,{children:"src"})," as if it were an external package, with access only to the public exports. You don't need to add a ",(0,s.jsx)(n.code,{children:"lib.rs"}),", ",(0,s.jsx)(n.code,{children:"mod.rs"}),", or ",(0,s.jsx)(n.code,{children:"main.rs"})," - each file in the ",(0,s.jsx)(n.code,{children:"tests"})," folder will be auto-discovered and treated as a separate entry point with its own module."]}),"\n",(0,s.jsxs)(n.p,{children:["For the purposes of your own application, you'll want to create a ",(0,s.jsx)(n.code,{children:"tests/utils.rs"})," file that wraps the ",(0,s.jsx)(n.code,{children:"nakago_axum::test::Utils"})," so that you can override any dependencies that you need or add convenience methods to build test data easily for your tests. Start out with a newtype like this:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use simple::Config;\n\npub struct TestUtils(nakago_axum::test::Utils<Config>);\n"})}),"\n",(0,s.jsxs)(n.p,{children:["Replace ",(0,s.jsx)(n.code,{children:"simple"})," with your actual project name."]}),"\n",(0,s.jsxs)(n.p,{children:["To make it easy to access the fields on the inner ",(0,s.jsx)(n.code,{children:"TestUtils"}),", you can implement the ",(0,s.jsx)(n.code,{children:"Deref"})," trait for your newtype. This isn't generally a good practice for newtypes in Production because it can result in some easy-to-miss implicit conversion behind the scenes, but in testing it's a nice convenience:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"use std::ops::Deref;\n\n// ...\n\nimpl Deref for TestUtils {\n    type Target = nakago_axum::test::Utils<Config>;\n\n    fn deref(&self) -> &Self::Target {\n        &self.0\n    }\n}\n"})}),"\n",(0,s.jsxs)(n.p,{children:["Now, you can implement an ",(0,s.jsx)(n.code,{children:"init()"})," method for your app-specific ",(0,s.jsx)(n.code,{children:"Utils"})," wrapper:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'use anyhow::Result;\nuse nakago_axum::auth::{validator, Validator};\nuse simple::{http::router, init, Config};\n\n// ...\n\nimpl TestUtils {\n    pub async fn init() -> Result<Self> {\n        let config_path = std::env::var("CONFIG_PATH_SIMPLE")\n            .unwrap_or_else(|_| "examples/simple/config.test.toml".to_string());\n\n        let i = init::app(Some(config_path.clone().into())).await?;\n\n        i.replace_with::<Validator>(validator::ProvideUnverified::default())\n            .await?;\n\n        let router = router::init(&i);\n\n        let utils = nakago_axum::test::Utils::init(i, "/", router).await?;\n\n        Ok(Self(utils))\n    }\n}\n'})}),"\n",(0,s.jsxs)(n.p,{children:["Again, replace ",(0,s.jsx)(n.code,{children:"simple"})," with your actual project name. The ",(0,s.jsx)(n.code,{children:"CONFIG_PATH"})," variable is used so that you can replace that with ",(0,s.jsx)(n.code,{children:"config.ci.toml"})," or whatever you need for testing in different environments."]}),"\n",(0,s.jsxs)(n.p,{children:["Now, create a ",(0,s.jsx)(n.code,{children:"test_users_int.rs"})," to represent your User integration tests, which will currently just test the ",(0,s.jsx)(n.code,{children:"/username"})," endpoint."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'#![cfg(feature = "integration")]\n\nuse utils::TestUtils;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = TestUtils::init().await?;\n\n    todo!("unimplemented")\n}\n'})}),"\n",(0,s.jsxs)(n.p,{children:["The ",(0,s.jsx)(n.code,{children:'#![cfg(feature = "integration")]'})," at the top of this file means that it will only be included in the build if the ",(0,s.jsx)(n.code,{children:"integration"})," feature flag is enabled. This is a good practice to follow for all your integration tests, because it allows you to run your unit tests while skipping integration tests so that you don't need supporting services in a local Docker Compose formation or other external dependencies."]}),"\n",(0,s.jsxs)(n.p,{children:["The ",(0,s.jsx)(n.code,{children:"todo!()"})," macro allows you to leave this test unfinished for now, but it will throw an error if you try to execute the tests."]}),"\n",(0,s.jsx)(n.h3,{id:"http-calls",children:"HTTP Calls"}),"\n",(0,s.jsxs)(n.p,{children:["Next, we can add an HTTP call with a JWT token. First, create the dummy token, which will only work with the ",(0,s.jsx)(n.code,{children:"auth::subject::ProvideUnverified"})," Validator provider above for use in testing."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'use ulid::Ulid;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = TestUtils::init().await?; // <-- this line should already be there\n\n    let username = Ulid::new().to_string();\n    let token = utils.create_jwt(&username).await?;\n\n    todo!("unimplemented")\n}\n'})}),"\n",(0,s.jsx)(n.p,{children:"Now we can make the HTTP call:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'let resp = utils\n    .http\n    .get_json("/username", Some(&token))\n    .send()\n    .await?;\n'})}),"\n",(0,s.jsx)(n.p,{children:"Pull the response apart into a status and a JSON body:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:"let status = resp.status();\nlet json = resp.json::<Value>().await?;\n"})}),"\n",(0,s.jsx)(n.p,{children:"Now you can make assertions based on the response:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'assert_eq!(status, 200);\nassert_eq!(json["username"], username);\n'})}),"\n",(0,s.jsxs)(n.p,{children:["Add an ",(0,s.jsx)(n.code,{children:"Ok(())"})," at the end to signal a successful test run, and your final test should look like this:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'#![cfg(feature = "integration")]\n\nuse anyhow::Result;\n\n#[cfg(test)]\nmod utils;\n\nuse serde_json::Value;\nuse ulid::Ulid;\nuse utils::TestUtils;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = TestUtils::init().await?;\n\n    let username = Ulid::new().to_string();\n    let token = utils.create_jwt(&username).await?;\n\n    let resp = utils\n        .http\n        .get_json("/username", Some(&token))\n        .send()\n        .await?;\n\n    let status = resp.status();\n\n    let json = resp.json::<Value>().await?;\n\n    assert_eq!(status, 200);\n    assert_eq!(json["username"], username);\n\n    Ok(())\n}\n'})}),"\n",(0,s.jsx)(n.h3,{id:"running-the-tests",children:"Running the Tests"}),"\n",(0,s.jsxs)(n.p,{children:["To run integration tests locally, add the following command to your ",(0,s.jsx)(n.code,{children:"Makefile.toml"}),":"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-toml",children:'[tasks.integration]\nenv = { "RUN_MODE" = "test", "RUST_LOG" = "info", "RUST_BACKTRACE" = 1 }\ncommand = "cargo"\nargs = ["nextest", "run", "--features=integration", "--workspace", "${@}"]\n'})}),"\n",(0,s.jsxs)(n.p,{children:["This won't work until you add the ",(0,s.jsx)(n.code,{children:"integration"})," feature to your ",(0,s.jsx)(n.code,{children:"Cargo.toml"}),", however:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-toml",children:"[features]\nintegration = []\n"})}),"\n",(0,s.jsxs)(n.p,{children:["Now you can run ",(0,s.jsx)(n.code,{children:"cargo make integration"}),", and it will use ",(0,s.jsx)(n.a,{href:"https://github.com/nextest-rs/nextest",children:"nextest"})," to run all available integration tests. It also allows you to pass options to ",(0,s.jsx)(n.code,{children:"nextest"}),", including filtering down to a specific test or group of tests."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"cargo make integration\n"})}),"\n",(0,s.jsx)(n.p,{children:"You should see a message that looks like the following:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"    Starting 1 test across 4 binaries\n        PASS [   0.230s] simple::test_users_int test_get_username_success\n------------\n     Summary [   0.230s] 1 test run: 1 passed, 0 skipped\n"})}),"\n",(0,s.jsxs)(n.p,{children:["If you want to see it fail, you can adjust the expectations at the end of the test in ",(0,s.jsx)(n.code,{children:"test_users_int.rs"}),":"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-rust",children:'assert_eq!(json["username"], "bob");\n'})}),"\n",(0,s.jsx)(n.p,{children:"Instead of the output above, you'll see a gnarly stacktrace with the following at the top:"}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{className:"language-sh",children:"        FAIL [   0.378s] simple::test_users_int test_get_username_success\n\n--- STDOUT:              simple::test_users_int test_get_username_success ---\n\nrunning 1 test\nthread '<unnamed>' panicked at 'assertion failed: `(left == right)`\n  left: `String(\"01HA5SF2AB3FV269P5ZEZ46033\")`,\n right: `\"bob\"`', tests/test_users_int.rs:32:5\n"})}),"\n",(0,s.jsx)(n.h2,{id:"finished-result",children:"Finished Result"}),"\n",(0,s.jsx)(n.p,{children:"Congratulations! You now have a simple API server with JWT+JWKS authentication in Rust, and you've added integration tests to ensure that it works as expected!"}),"\n",(0,s.jsxs)(n.p,{children:["You can see everything together in the ",(0,s.jsx)(n.a,{href:"https://github.com/bkonkle/nakago/tree/main/examples/simple",children:"examples/simple"})," folder of the ",(0,s.jsx)(n.code,{children:"nakago"})," repository."]})]})}function h(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(c,{...e})}):c(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>o,x:()=>r});var s=t(6540);const i={},a=s.createContext(i);function o(e){const n=s.useContext(a);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function r(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),s.createElement(a.Provider,{value:n},e.children)}}}]);