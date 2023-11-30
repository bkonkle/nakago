"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[554],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>m});var a=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function r(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,i=function(e,t){if(null==e)return{};var n,a,i={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var s=a.createContext({}),u=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):r(r({},t),e)),n},p=function(e){var t=u(e.components);return a.createElement(s.Provider,{value:t},e.children)},d="mdxType",c={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},h=a.forwardRef((function(e,t){var n=e.components,i=e.mdxType,o=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),d=u(n),h=i,m=d["".concat(s,".").concat(h)]||d[h]||c[h]||o;return n?a.createElement(m,r(r({ref:t},p),{},{components:n})):a.createElement(m,r({ref:t},p))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=n.length,r=new Array(o);r[0]=h;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[d]="string"==typeof e?e:i,r[1]=l;for(var u=2;u<o;u++)r[u]=n[u];return a.createElement.apply(null,r)}return a.createElement.apply(null,n)}h.displayName="MDXCreateElement"},6351:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>r,default:()=>c,frontMatter:()=>o,metadata:()=>l,toc:()=>u});var a=n(7462),i=(n(7294),n(3905));const o={sidebar_position:2},r="Tutorial",l={unversionedId:"tutorial",id:"tutorial",title:"Tutorial",description:"This tutorial will walk you through the basics of using Nakago to build a simple HTTP service. It will use Axum to provide HTTP routes and will decode the user's JWT token and verify their identity via a separate OAuth2 provider, such as Auth0 or Okta or your own self-hosted service.",source:"@site/docs/tutorial.md",sourceDirName:".",slug:"/tutorial",permalink:"/docs/tutorial",draft:!1,editUrl:"https://github.com/bkonkle/nakago/tree/main/website/docs/tutorial.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"documentationSidebar",previous:{title:"Welcome to Nakago",permalink:"/docs/intro"},next:{title:"Features",permalink:"/docs/category/features"}},s={},u=[{value:"Cargo-Generate Template",id:"cargo-generate-template",level:2},{value:"Setup",id:"setup",level:2},{value:"Authentication",id:"authentication",level:2},{value:"Auth Config",id:"auth-config",level:3},{value:"Initialization",id:"initialization",level:3},{value:"Axum Route",id:"axum-route",level:3},{value:"Running the App",id:"running-the-app",level:3},{value:"Integration Testing",id:"integration-testing",level:2},{value:"Test Utils",id:"test-utils",level:3},{value:"HTTP Calls",id:"http-calls",level:3},{value:"Running the Tests",id:"running-the-tests",level:3},{value:"Finished Result",id:"finished-result",level:2}],p={toc:u},d="wrapper";function c(e){let{components:t,...n}=e;return(0,i.kt)(d,(0,a.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"tutorial"},"Tutorial"),(0,i.kt)("p",null,"This tutorial will walk you through the basics of using Nakago to build a simple HTTP service. It will use Axum to provide HTTP routes and will decode the user's JWT token and verify their identity via a separate OAuth2 provider, such as Auth0 or Okta or your own self-hosted service."),(0,i.kt)("h2",{id:"cargo-generate-template"},"Cargo-Generate Template"),(0,i.kt)("p",null,"First install ",(0,i.kt)("inlineCode",{parentName:"p"},"cargo-generate"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"cargo install cargo-generate\n")),(0,i.kt)("p",null,"Then generate a new project with this template:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"cargo generate bkonkle/nakago-simple-template\n")),(0,i.kt)("p",null,"You'll see a folder structure like this:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-text"},"simple/\n\u251c\u2500 .cargo/ -- Clippy config\n\u251c\u2500 .github/ -- Github Actions\n\u251c\u2500 config/ -- Config files for different environments\n\u251c\u2500 src/\n\u2502  \u251c\u2500 http/ -- Axum HTTP routes\n\u2502  \u2502  \u251c\u2500 health.rs -- The HTTP health check handler\n\u2502  \u2502  \u251c\u2500 init.rs -- Initialization hook for all handlers\n\u2502  \u2502  \u2514\u2500 mod.rs\n\u2502  \u251c\u2500 config.rs -- Your app's custom Config struct\n\u2502  \u251c\u2500 init.rs -- App initialization\n\u2502  \u251c\u2500 lib.rs\n\u2502  \u2514\u2500 main.rs -- Main entry point\n\u251c\u2500 Cargo.toml\n\u251c\u2500 Makefile.toml\n\u251c\u2500 README.md\n\u2514\u2500 // ...\n")),(0,i.kt)("p",null,"This includes a simple app-specific ",(0,i.kt)("inlineCode",{parentName:"p"},"Config")," struct with an embedded ",(0,i.kt)("inlineCode",{parentName:"p"},"http::Config")," provided by the ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago-axum")," library. You can add your own configuration fields to this struct and they'll be populated by the ",(0,i.kt)("a",{parentName:"p",href:"https://docs.rs/figment/latest/figment/"},"figment")," crate."),(0,i.kt)("p",null,"It includes a barebones ",(0,i.kt)("inlineCode",{parentName:"p"},"init::app()")," function that will load your configuration and initialize your dependencies. You can add your own dependencies to this function and they'll be available when you build your Axum routes."),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"main.rs")," uses the ",(0,i.kt)("a",{parentName:"p",href:"https://docs.rs/pico-args/0.5.0/pico_args/"},"pico-args")," to parse a simple command-line argument to specify an alternate config path, which is useful for many deployment scenarios that dynamically map a config file to a certain mount point within a container filesystem."),(0,i.kt)("p",null,"In the ",(0,i.kt)("inlineCode",{parentName:"p"},"http/")," folder, you'll find an Axum handler and a router initialization hook. The router maps a simple ",(0,i.kt)("inlineCode",{parentName:"p"},"GET /health")," route to a handler that returns a JSON response with a success message."),(0,i.kt)("p",null,"You now have a simple foundation to build on. Let's add some more functionality!"),(0,i.kt)("h2",{id:"setup"},"Setup"),(0,i.kt)("p",null,"Follow the Installation instructions in the ",(0,i.kt)("inlineCode",{parentName:"p"},"README.md")," to prepare your new local environment."),(0,i.kt)("h2",{id:"authentication"},"Authentication"),(0,i.kt)("p",null,"One of the first things you'll probably want to add to your application is authentication, which establishes the user's identity. This is separate and distinct from authorization, which determines what the user is allowed to do."),(0,i.kt)("p",null,"The only currently supported method of authentication is through JWT with JWKS keys, though other methods will be added in the future. The ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago-axum")," library provides a request extractor for for Axum that uses ",(0,i.kt)("a",{parentName:"p",href:"https://docs.rs/biscuit/0.6.0/biscuit/"},"biscuit")," with your Nakago application Config to decode a JWT from the ",(0,i.kt)("inlineCode",{parentName:"p"},"Authorization")," header, validate it with a JWKS key from the ",(0,i.kt)("inlineCode",{parentName:"p"},"/.well-known/jwks.json")," path on the auth url, and then return the value of the ",(0,i.kt)("inlineCode",{parentName:"p"},"sub")," claim from the payload."),(0,i.kt)("p",null,(0,i.kt)("em",{parentName:"p"},"Configurable claims and other authentication methods will be added in the future.")),(0,i.kt)("h3",{id:"auth-config"},"Auth Config"),(0,i.kt)("p",null,"In your ",(0,i.kt)("inlineCode",{parentName:"p"},"config.rs")," file, add a new property to the app's ",(0,i.kt)("inlineCode",{parentName:"p"},"Config")," struct:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"/// Server Config\n#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]\npub struct Config {\n    /// HTTP config\n    pub http: nakago_axum::Config,\n\n    /// HTTP Auth Config\n    pub auth: nakago_axum::auth::Config,\n}\n")),(0,i.kt)("p",null,"This auth ",(0,i.kt)("inlineCode",{parentName:"p"},"Config")," is automatically loaded as part of the default config loaders in the ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago-axum")," crate, which you'll see below."),(0,i.kt)("p",null,"Next, add the following values to your ",(0,i.kt)("inlineCode",{parentName:"p"},"config/local.toml.example")," file as a hint, so that new developers know they need to reach out to you for real values when they create their own ",(0,i.kt)("inlineCode",{parentName:"p"},"config/local.toml")," file:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-toml"},'[auth]\nurl = "https://simple-dev.oauth-service.com"\naudience = "localhost"\n\n[auth.client]\nid = "client_id"\nsecret = "client_secret"\n')),(0,i.kt)("p",null,"Add the real details to your own ",(0,i.kt)("inlineCode",{parentName:"p"},"config.toml")," file, which should be excluded from git via the ",(0,i.kt)("inlineCode",{parentName:"p"},".gitignore")," file. If you don't have real values yet, leave them as the dummy values above. You can still run integration tests without having a real OAuth2 provider running, if you want."),(0,i.kt)("h3",{id:"initialization"},"Initialization"),(0,i.kt)("p",null,"You're now ready to head over to your initialization routine. This is where you will provide all of the dependencies and lifecycle hooks your app needs in order to start up."),(0,i.kt)("p",null,"This line already in the top-level ",(0,i.kt)("inlineCode",{parentName:"p"},"init.rs")," ensures that your config is populated from environment variables or the currently chosen config file, along with the auth property you added above:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"// This line should already be in your `init.rs` file\napp.on(&EventType::Load, config::AddLoaders::default());\n")),(0,i.kt)("p",null,"First, add the default JWKS Validator from ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago_axum"),"'s ",(0,i.kt)("inlineCode",{parentName:"p"},"auth")," module using the ",(0,i.kt)("inlineCode",{parentName:"p"},"provide_type")," method, which uses the type as the key for the Inject container:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"app.provide_type::<Validator>(validator::Provide::default())\n    .await?;\n")),(0,i.kt)("p",null,"This will be overridden in your tests to use the unverified variant, but we'll get to that later."),(0,i.kt)("p",null,"Next should use ",(0,i.kt)("inlineCode",{parentName:"p"},"jwks::Provide")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"auth::subject::Provide")," providers to inject the pieces that Nakago-Axum's ",(0,i.kt)("inlineCode",{parentName:"p"},"Subject")," extractor will use to retrieve the authentication data for a request. For the JWKS config we'll use a tag, so use the ",(0,i.kt)("inlineCode",{parentName:"p"},"provide")," method rather than ",(0,i.kt)("inlineCode",{parentName:"p"},"provide_type"),". This uses the tag as the key for the Inject container."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use nakago_axum::auth::{self, jwks, Validator, JWKS};\n\n// ...\n\napp.provide(&JWKS, jwks::Provide::default().with_config_tag(&CONFIG))\n    .await?;\n\napp.provide_type::<Validator>(auth::subject::Provide::default())\n    .await?;\n\n// ...\n")),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},".with_config_tag(&CONFIG)")," provides the custom Tag for your app's custom ",(0,i.kt)("inlineCode",{parentName:"p"},"Config"),"."),(0,i.kt)("h3",{id:"axum-route"},"Axum Route"),(0,i.kt)("p",null,"You can now add a quick handler to ",(0,i.kt)("inlineCode",{parentName:"p"},"http/")," that allows a user to view their own username when logged in. Create a new file called ",(0,i.kt)("inlineCode",{parentName:"p"},"http/user.rs")),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'use nakago_axum::auth::Subject;\n\n/// A Username Response\n#[derive(Debug, Clone, Serialize, Deserialize)]\npub struct UsernameResponse {\n    /// The Status code\n    code: usize,\n\n    /// The username, or "(anonymous)"\n    username: String,\n}\n\n/// Handle Get Username requests\npub async fn get_username(sub: Subject) -> Json<UsernameResponse> {\n    let username = if let Subject(Some(username)) = sub {\n        username.clone()\n    } else {\n        "(anonymous)".to_string()\n    };\n\n    Json(UsernameResponse {\n        code: 200,\n        username,\n    })\n}\n')),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"Subject")," extension uses Nakago's bulit-in Axum State to find the Inject container, which it uses to grab the JWT config and the Validator instance. It uses them to decode the JWT and return the ",(0,i.kt)("inlineCode",{parentName:"p"},"sub")," claim from the payload. If the user is not logged in, the ",(0,i.kt)("inlineCode",{parentName:"p"},"Subject")," will contain a ",(0,i.kt)("inlineCode",{parentName:"p"},"None"),"."),(0,i.kt)("p",null,"Now add a route that uses the handler to the Init hook at ",(0,i.kt)("inlineCode",{parentName:"p"},"http/init.rs"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'i.handle(routes::Init::new(\n    Method::GET,\n    "/username",\n    user::get_username,\n))\n.await?;\n')),(0,i.kt)("h3",{id:"running-the-app"},"Running the App"),(0,i.kt)("p",null,"At this point, you can run your app and see the ",(0,i.kt)("inlineCode",{parentName:"p"},"(anonymous)")," response at the ",(0,i.kt)("inlineCode",{parentName:"p"},"GET /username")," endpoint:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"cargo make run\n")),(0,i.kt)("p",null,"The uses cargo-make, a tool to provide enhanced Makefile-like functionality for Rust projects. You can see the configuration in the ",(0,i.kt)("inlineCode",{parentName:"p"},"Makefile.toml")," file."),(0,i.kt)("p",null,"At first, you'll see a big ugly traceback with the following error message at the top because you don't have a valid autd provider configured:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"thread '<unnamed>' panicked at 'Unable to retrieve JWKS: invalid format'\n")),(0,i.kt)("p",null,'This is okay - you don\'t have to have a properly configured auth provider to run the integration tests for your app. You can use the "unverified" ',(0,i.kt)("inlineCode",{parentName:"p"},"AuthState")," variant during integration testing, and skip the rest of this section."),(0,i.kt)("p",null,"If you ",(0,i.kt)("em",{parentName:"p"},"do")," have a valid OAuth2 provider, then you'll want to create a ",(0,i.kt)("inlineCode",{parentName:"p"},"config/local.toml")," file and set the following property in it:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-toml"},'[auth]\nurl = "https://simple-dev.oauth-service.com"\n')),(0,i.kt)("p",null,"You can also use the ",(0,i.kt)("inlineCode",{parentName:"p"},"AUTH_URL")," environment variable to set this value. Consider using a tool like ",(0,i.kt)("a",{parentName:"p",href:"https://direnv.net/"},"direnv")," to manage variables like this in your local development environment with ",(0,i.kt)("inlineCode",{parentName:"p"},".envrc")," files."),(0,i.kt)("p",null,"Your provider should have a ",(0,i.kt)("inlineCode",{parentName:"p"},"/.well-known/jwks.json")," file available at the given auth url, which will avoid the error message above. You should now see output that looks like the following:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"2023-09-08T02:14:03.388670Z  INFO simple: Started on port: 8000\n")),(0,i.kt)("p",null,"When you call ",(0,i.kt)("inlineCode",{parentName:"p"},"http://localhost:8000/username")," in your browser, you should see the following response:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-json"},'{\n  "code": 200,\n  "username": "(anonymous)"\n}\n')),(0,i.kt)("h2",{id:"integration-testing"},"Integration Testing"),(0,i.kt)("p",null,"Now that you have a simple route that requires authentication, you'll want to add some integration tests to ensure that it works as expected. You don't actually need to have an OAuth2 provider running to test this, because the ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago-axum")," library provides a mock unverified ",(0,i.kt)("inlineCode",{parentName:"p"},"AuthState")," that you can use to simulate a logged-in user."),(0,i.kt)("h3",{id:"test-utils"},"Test Utils"),(0,i.kt)("p",null,"Nakago Axum's HTTP ",(0,i.kt)("inlineCode",{parentName:"p"},"Utils")," class is based on the idea of extending the base test ",(0,i.kt)("inlineCode",{parentName:"p"},"Utils")," class you'll find in ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago_axum::test::Utils")," with additional functionality, like adding a ",(0,i.kt)("inlineCode",{parentName:"p"},"graphql")," property if you're using ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago-async-graphql")," or adding convenience methods around your app-specific data."),(0,i.kt)("p",null,"To start out with, create a ",(0,i.kt)("inlineCode",{parentName:"p"},"tests")," folder alongside your ",(0,i.kt)("inlineCode",{parentName:"p"},"src"),". This will be used by Cargo as an ",(0,i.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests"},'"integration test"')," module, and will be excluded from your final binary. It allows you to import the module in your ",(0,i.kt)("inlineCode",{parentName:"p"},"src")," as if it were an external package, with access only to the public exports. You don't need to add a ",(0,i.kt)("inlineCode",{parentName:"p"},"lib.rs"),", ",(0,i.kt)("inlineCode",{parentName:"p"},"mod.rs"),", or ",(0,i.kt)("inlineCode",{parentName:"p"},"main.rs")," - each file in the ",(0,i.kt)("inlineCode",{parentName:"p"},"tests")," folder will be auto-discovered and treated as a separate entry point with its own module."),(0,i.kt)("p",null,"For the purposes of your own application, you'll want to create a ",(0,i.kt)("inlineCode",{parentName:"p"},"tests/utils.rs")," file that wraps the ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago_axum::test::Utils")," so that you can override any dependencies that you need or add convenience methods to build test data easily for your tests. Start out with a newtype like this:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use simple::Config;\n\npub struct Utils(nakago_axum::test::Utils<Config>);\n")),(0,i.kt)("p",null,"Replace ",(0,i.kt)("inlineCode",{parentName:"p"},"simple")," with your actual project name."),(0,i.kt)("p",null,"To make it easy to access the fields on the inner ",(0,i.kt)("inlineCode",{parentName:"p"},"Utils"),", you can implement the ",(0,i.kt)("inlineCode",{parentName:"p"},"Deref")," trait for your newtype. This isn't generally a good practice for newtypes in Production because it can result in some easy-to-miss implicit conversion behind the scenes, but in testing it's a nice convenience:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use std::ops::Deref;\n\nimpl Deref for Utils {\n    type Target = nakago_axum::test::Utils<Config>;\n\n    fn deref(&self) -> &Self::Target {\n        &self.0\n    }\n}\n")),(0,i.kt)("p",null,"Now, you can implement an ",(0,i.kt)("inlineCode",{parentName:"p"},"init()")," method for your app-specific ",(0,i.kt)("inlineCode",{parentName:"p"},"Utils")," wrapper:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'use anyhow::Result;\nuse nakago_axum::auth;\n\nuse simple::init;\n\nimpl Utils {\n    pub async fn init() -> Result<Self> {\n        let app = init::app().await?;\n\n        app.replace_type_with::<Validator>(auth::subject::ProvideUnverified::default())\n            .await?;\n\n        let config_path =\n            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/test.toml".to_string());\n\n        let utils = nakago_axum::Utils::init(app, &config_path, "/").await?;\n\n        Ok(Self(utils))\n    }\n}\n')),(0,i.kt)("p",null,"Again, replace ",(0,i.kt)("inlineCode",{parentName:"p"},"simple")," with your actual project name. The ",(0,i.kt)("inlineCode",{parentName:"p"},"CONFIG_PATH")," variable is used so that you can replace that with ",(0,i.kt)("inlineCode",{parentName:"p"},"config/ci.toml")," or whatever you need for testing in different environments."),(0,i.kt)("p",null,"Now, create a ",(0,i.kt)("inlineCode",{parentName:"p"},"test_users_int.rs")," to represent your User integration tests, which will currently just test the ",(0,i.kt)("inlineCode",{parentName:"p"},"/username")," endpoint."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg(feature = "integration")]\n\nuse test_utils::Utils;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = Utils::init().await?;\n\n    todo!("unimplemented")\n}\n')),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},'#![cfg(feature = "integration")]')," at the top of this file means that it will only be included in the build if the ",(0,i.kt)("inlineCode",{parentName:"p"},"integration")," feature flag is enabled. This is a good practice to follow for all your integration tests, because it allows you to run your unit tests while skipping integration tests so that you don't need supporting services in a local Docker Compose formation or other external dependencies."),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"todo!()")," macro allows you to leave this test unfinished for now, but it will throw an error if you try to execute the tests."),(0,i.kt)("h3",{id:"http-calls"},"HTTP Calls"),(0,i.kt)("p",null,"Next, we can add an HTTP call with a JWT token. First, create the dummy token, which will only work with the ",(0,i.kt)("inlineCode",{parentName:"p"},"auth::subject::ProvideUnverified")," Validator provider above for use in testing."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'use ulid::Ulid;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = Utils::init().await?; // <-- this line should already be there\n\n    let username = Ulid::new().to_string();\n    let token = utils.create_jwt(&username).await?;\n\n    todo!("unimplemented")\n}\n')),(0,i.kt)("p",null,"Now we can make the HTTP call:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'let req = utils\n    .http\n    .call(Method::GET, "/username", Value::Null, Some(&token))?;\n\nlet resp = utils.http_client.request(req).await?;\n')),(0,i.kt)("p",null,"Pull the response apart into a status and a body:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let status = resp.status();\nlet body = to_bytes(resp.into_body()).await?;\n")),(0,i.kt)("p",null,"Now you can make assertions based on the response:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'let json: Value = serde_json::from_slice(&body)?;\n\nassert_eq!(status, 200);\nassert_eq!(json["username"], username);\n')),(0,i.kt)("p",null,"Add an ",(0,i.kt)("inlineCode",{parentName:"p"},"Ok(())")," at the end to signal a successful test run, and your final test should look like this:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'use anyhow::Result;\n\n#[cfg(test)]\nmod test_utils;\n\nuse hyper::body::to_bytes;\nuse serde_json::Value;\nuse test_utils::Utils;\nuse ulid::Ulid;\n\n#[tokio::test]\nasync fn test_get_username_success() -> Result<()> {\n    let utils = Utils::init().await?;\n\n    let username = Ulid::new().to_string();\n    let token = utils.create_jwt(&username).await?;\n\n    let req = utils.http.call("/username", Value::Null, Some(&token))?;\n    let resp = utils.http_client.request(req).await?;\n\n    let status = resp.status();\n    let body = to_bytes(resp.into_body()).await?;\n\n    let json: Value = serde_json::from_slice(&body)?;\n\n    assert_eq!(status, 200);\n    assert_eq!(json["username"], username);\n\n    Ok(())\n}\n')),(0,i.kt)("h3",{id:"running-the-tests"},"Running the Tests"),(0,i.kt)("p",null,"To run integration tests locally, add the following command to your ",(0,i.kt)("inlineCode",{parentName:"p"},"Makefile.toml"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-toml"},'[tasks.integration]\nenv = { "RUN_MODE" = "test", "RUST_LOG" = "info", "RUST_BACKTRACE" = 1 }\ncommand = "cargo"\nargs = ["nextest", "run", "--features=integration", "--workspace", "${@}"]\n')),(0,i.kt)("p",null,"This won't work until you add the ",(0,i.kt)("inlineCode",{parentName:"p"},"integration")," feature to your ",(0,i.kt)("inlineCode",{parentName:"p"},"Cargo.toml"),", however:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-toml"},"[features]\nintegration = []\n")),(0,i.kt)("p",null,"Now you can run ",(0,i.kt)("inlineCode",{parentName:"p"},"cargo make integration"),", and it will use ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/nextest-rs/nextest"},"nextest")," to run all available integration tests. It also allows you to pass options to ",(0,i.kt)("inlineCode",{parentName:"p"},"nextest"),", including filtering down to a specific test or group of tests."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"cargo make integration\n")),(0,i.kt)("p",null,"You should see a message that looks like the following:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"    Starting 1 test across 4 binaries\n        PASS [   0.230s] simple::test_users_int test_get_username_success\n------------\n     Summary [   0.230s] 1 test run: 1 passed, 0 skipped\n")),(0,i.kt)("p",null,"If you want to see it fail, you can adjust the expectations at the end of the test in ",(0,i.kt)("inlineCode",{parentName:"p"},"test_users_int.rs"),":"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'assert_eq!(json["username"], "bob");\n')),(0,i.kt)("p",null,"Instead of the output above, you'll see a gnarly stacktrace with the following at the top:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-sh"},"        FAIL [   0.378s] simple::test_users_int test_get_username_success\n\n--- STDOUT:              simple::test_users_int test_get_username_success ---\n\nrunning 1 test\nthread '<unnamed>' panicked at 'assertion failed: `(left == right)`\n  left: `String(\"01HA5SF2AB3FV269P5ZEZ46033\")`,\n right: `\"bob\"`', tests/test_users_int.rs:32:5\n")),(0,i.kt)("h2",{id:"finished-result"},"Finished Result"),(0,i.kt)("p",null,"Congratulations! You now have a simple API server with JWT+JWKS authentication in Rust, and you've added integration tests to ensure that it works as expected!"),(0,i.kt)("p",null,"You can see everything together in the ",(0,i.kt)("a",{parentName:"p",href:"https://github.com/bkonkle/nakago/tree/main/examples/simple"},"examples/simple")," folder of the ",(0,i.kt)("inlineCode",{parentName:"p"},"nakago")," repository."))}c.isMDXComponent=!0}}]);