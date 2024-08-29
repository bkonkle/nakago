"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[208],{4331:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>r,default:()=>h,frontMatter:()=>s,metadata:()=>o,toc:()=>u});var a=n(4848),i=n(8453);const s={sidebar_position:3},r="Using Nakago with Axum",o={id:"features/axum-http",title:"Using Nakago with Axum",description:"The nakago-axum crate defines provides a way to easily use the Nakago Inject container via Axum's State mechanism.",source:"@site/docs/features/axum-http.md",sourceDirName:"features",slug:"/features/axum-http",permalink:"/docs/features/axum-http",draft:!1,unlisted:!1,editUrl:"https://github.com/bkonkle/nakago/tree/main/website/docs/features/axum-http.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"documentationSidebar",previous:{title:"Dependency Injection",permalink:"/docs/features/dependency-injection"},next:{title:"SeaORM",permalink:"/docs/features/sea-orm"}},c={},u=[{value:"Axum State",id:"axum-state",level:2},{value:"Integration Testing",id:"integration-testing",level:2},{value:"CI Integration Testing",id:"ci-integration-testing",level:3}];function l(e){const t={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",p:"p",pre:"pre",...(0,i.R)(),...e.components};return(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)(t.header,{children:(0,a.jsx)(t.h1,{id:"using-nakago-with-axum",children:"Using Nakago with Axum"})}),"\n",(0,a.jsxs)(t.p,{children:["The ",(0,a.jsx)(t.code,{children:"nakago-axum"})," crate defines provides a way to easily use the Nakago ",(0,a.jsx)(t.code,{children:"Inject"})," container via Axum's ",(0,a.jsx)(t.code,{children:"State"})," mechanism."]}),"\n",(0,a.jsx)(t.h2,{id:"axum-state",children:"Axum State"}),"\n",(0,a.jsx)(t.p,{children:"Axum provides the State extractor that allows you to inject dependencies that stay the same across many requests. For DI-driven applications, however, your dependencies are provided through the injection container. Nakago's Axum helpers use State to automatically carry the injection container, but in a way that you don't have to think about while building typical applications."}),"\n",(0,a.jsxs)(t.p,{children:["Nakago provides an extractor called ",(0,a.jsx)(t.code,{children:"Inject"}),' that allows you to request dependencies from Nakago as smoothly as using any other Axum extractor. In this example, the "resolve" request handler uses ',(0,a.jsx)(t.code,{children:"Inject"})," to request the ",(0,a.jsx)(t.code,{children:"graphql::Schema"})," and a ",(0,a.jsx)(t.code,{children:"users::Service"})," trait implementation from the injection container so that it can be used to handle the request. The ",(0,a.jsx)(t.code,{children:"Subject"})," extractor is also used, to provide the JWT payload claims needed to find a User in the database:"]}),"\n",(0,a.jsx)(t.pre,{children:(0,a.jsx)(t.code,{className:"language-rust",children:"use nakago_axum::{auth::Subject, Inject};\n\nuse crate::domains::{graphql, users};\n\npub async fn resolve(\n    Inject(schema): Inject<graphql::Schema>,\n    Inject(users): Inject<Box<dyn users::Service>>,\n    sub: Subject,\n    req: GraphQLRequest,\n) -> GraphQLResponse {\n    // Retrieve the request User, if username is present\n    let user = if let Subject(Some(ref username)) = sub {\n        users.get_by_username(username, &true).await.unwrap_or(None)\n    } else {\n        None\n    };\n\n    // Add the Subject and optional User to the context\n    let request = req.into_inner().data(sub).data(user);\n\n    schema.execute(request).await.into()\n}\n"})}),"\n",(0,a.jsx)(t.p,{children:"Then you can initialize your top level Axum router in an initializer:"}),"\n",(0,a.jsx)(t.pre,{children:(0,a.jsx)(t.code,{className:"language-rust",children:'pub fn init(i: &Inject) -> Router {\n    Router::new()\n        .layer(trace_layer())\n        .route("/health", get(health::health_check))\n        .route("/graphql", get(graphql::graphiql).post(graphql::resolve))\n        .route("/events", get(events::handle))\n        .with_state(State::new(i.clone()))\n}\n'})}),"\n",(0,a.jsx)(t.h2,{id:"integration-testing",children:"Integration Testing"}),"\n",(0,a.jsx)(t.p,{children:"Integration testing is handled by initializing your application server in a way similar to Production, using test utils to make requests to your server running in the background."}),"\n",(0,a.jsx)(t.pre,{children:(0,a.jsx)(t.code,{className:"language-rust",children:'let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.test.toml".to_string());\n\nlet i = init::app(Some(config_path.clone().into())).await?;\n\ni.replace_with::<Validator>(validator::ProvideUnverified::default())\n    .await?;\n\nlet router = router::init(&i);\n\nlet utils = nakago_axum::test::Utils::init(i, "/", router).await?;\n\nlet username = Ulid::new().to_string();\nlet token = utils.create_jwt(&username).await?;\n\nlet resp = utils\n    .http\n    .request_json(Method::POST, "/username", Value::Null, Some(&token))\n    .send()\n    .await?;\n'})}),"\n",(0,a.jsxs)(t.p,{children:["See the ",(0,a.jsx)(t.a,{href:"https://github.com/bkonkle/nakago/tree/main/examples/async-graphql/tests",children:"Async-GraphQL Example's integration tests"})," for examples of how to use this pattern. This will evolve as more pieces are moved into the framework itself over time."]}),"\n",(0,a.jsx)(t.h3,{id:"ci-integration-testing",children:"CI Integration Testing"}),"\n",(0,a.jsx)(t.p,{children:"This strategy can be used for integration testing in the CI service of your choice based on a Docker Compose formation of shallow dependencies. This allows you to set up things like LocalStack or Postgers within your CI Docker environment and run integration tests against them without needing to use deployed resources. Branch-specific PR's are easy to run tests for in isolation."}),"\n",(0,a.jsx)(t.p,{children:"Stay tuned for more details on how to set up this approach in your CI environment."})]})}function h(e={}){const{wrapper:t}={...(0,i.R)(),...e.components};return t?(0,a.jsx)(t,{...e,children:(0,a.jsx)(l,{...e})}):l(e)}},8453:(e,t,n)=>{n.d(t,{R:()=>r,x:()=>o});var a=n(6540);const i={},s=a.createContext(i);function r(e){const t=a.useContext(s);return a.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function o(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:r(e.components),a.createElement(s.Provider,{value:t},e.children)}}}]);