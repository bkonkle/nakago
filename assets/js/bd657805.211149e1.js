"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[495],{3905:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>m});var a=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function r(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,i=function(e,t){if(null==e)return{};var n,a,i={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var p=a.createContext({}),l=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):r(r({},t),e)),n},d=function(e){var t=l(e.components);return a.createElement(p.Provider,{value:t},e.children)},c="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},y=a.forwardRef((function(e,t){var n=e.components,i=e.mdxType,o=e.originalType,p=e.parentName,d=s(e,["components","mdxType","originalType","parentName"]),c=l(n),y=i,m=c["".concat(p,".").concat(y)]||c[y]||u[y]||o;return n?a.createElement(m,r(r({ref:t},d),{},{components:n})):a.createElement(m,r({ref:t},d))}));function m(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=n.length,r=new Array(o);r[0]=y;var s={};for(var p in t)hasOwnProperty.call(t,p)&&(s[p]=t[p]);s.originalType=e,s[c]="string"==typeof e?e:i,r[1]=s;for(var l=2;l<o;l++)r[l]=n[l];return a.createElement.apply(null,r)}return a.createElement.apply(null,n)}y.displayName="MDXCreateElement"},1365:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>r,default:()=>u,frontMatter:()=>o,metadata:()=>s,toc:()=>l});var a=n(7462),i=(n(7294),n(3905));const o={sidebar_position:1},r="Dependency Injection",s={unversionedId:"features/dependency-injection",id:"features/dependency-injection",title:"Dependency Injection",description:"Dependency injection is a way to decouple your structures from their dependencies. It allows you to replace the components that your system needs with alternative implementations for different situations.",source:"@site/docs/features/dependency-injection.md",sourceDirName:"features",slug:"/features/dependency-injection",permalink:"/docs/features/dependency-injection",draft:!1,editUrl:"https://github.com/bkonkle/nakago/tree/main/website/docs/features/dependency-injection.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1},sidebar:"documentationSidebar",previous:{title:"Features",permalink:"/docs/category/features"},next:{title:"Application Lifecycle",permalink:"/docs/features/application"}},p={},l=[{value:"Async",id:"async",level:2},{value:"Usage",id:"usage",level:2},{value:"Dependency Tags",id:"dependency-tags",level:2},{value:"Providing Dependencies",id:"providing-dependencies",level:2},{value:"The Provider Macro",id:"the-provider-macro",level:2},{value:"The Inject Container",id:"the-inject-container",level:2},{value:"Invoking Dependencies",id:"invoking-dependencies",level:2},{value:"Consuming Dependencies",id:"consuming-dependencies",level:2},{value:"Ejection",id:"ejection",level:2}],d={toc:l},c="wrapper";function u(e){let{components:t,...n}=e;return(0,i.kt)(c,(0,a.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"dependency-injection"},"Dependency Injection"),(0,i.kt)("p",null,"Dependency injection is a way to ",(0,i.kt)("a",{parentName:"p",href:"https://en.wikipedia.org/wiki/Loose_coupling"},"decouple")," your structures from their dependencies. It allows you to replace the components that your system needs with alternative implementations for different situations."),(0,i.kt)("p",null,"For example, a Controller may want to interact with a Repository to access information in persistent storage. In different situations you may want a Postgres Repository, a DynamoDB Repository, or an In-Memory Repository. Using dependency injection for loose coupling allows your Controller to depend on a common Repository trait that they all implement, without caring which underlying implementation actually fulfills the requirement."),(0,i.kt)("p",null,"This is accomplished using ",(0,i.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/std/any/index.html"},"Any")," from Rust's standard library. By using dynamic typing, you can easily swap between different implementations for different entry points or contexts, and easily add more as needed for any situation."),(0,i.kt)("p",null,"One quirk of Any is that values need to have the ",(0,i.kt)("inlineCode",{parentName:"p"},"'static")," lifetime, meaning they are valid until program execution ends if they are not dropped. Keep this in mind if you're frequently injecting and removing items from the container during your program's lifecycle."),(0,i.kt)("h2",{id:"async"},"Async"),(0,i.kt)("p",null,"Nakago's ",(0,i.kt)("inlineCode",{parentName:"p"},"Inject")," framework is built on ",(0,i.kt)("a",{parentName:"p",href:"https://tokio.rs/"},"Tokio")," with ",(0,i.kt)("a",{parentName:"p",href:"https://docs.rs/futures/latest/futures/future/struct.Shared.html"},"Shared Futures"),", allowing multiple threads to request and await the same dependency and use an Arc to hold on to it across await points without worrying about lifetimes."),(0,i.kt)("p",null,"It uses Providers that implement the async ",(0,i.kt)("inlineCode",{parentName:"p"},"Provider")," trait and use the provider's instance for configuration or context, and the Inject container to request other dependencies you require. Providers are lazily invoked - they are stored internally but not invoked until they are requested. They are then converted into a pending Shared Future that can be polled by multiple threads at the same time. This allows multiple resources to wait for the same Provider invocation without duplicaiton."),(0,i.kt)("p",null,"Providers don't have to be injected in any order - they will wait inside the container until they have been requested, so you have some flexibility in your application's initialization process. They are guarded by ",(0,i.kt)("inlineCode",{parentName:"p"},"RwLock")," wrappers for thread-safe access, and the locks are released once the ",(0,i.kt)("inlineCode",{parentName:"p"},"Arc<T>")," is yielded."),(0,i.kt)("h2",{id:"usage"},"Usage"),(0,i.kt)("p",null,"First, let's establish a hypothetical Entity and a trait that defines a method to retrieve it from persistent storage:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use async_trait::async_trait;\n\nstruct Entity {\n    id: String,\n}\n\n#[async_trait]\ntrait Repository: Sync + Send {\n    async fn get(&self, id: &str) -> Option<Entity>;\n}\n")),(0,i.kt)("p",null,"Then a hypothetical Postgres implementation:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use sqlx::{Pool, Postgres};\n\nstruct PostgresRepository {\n    pool: Pool<Postgres>\n}\n\n#[async_trait]\nimpl Repository for PostgresRepository {\n    async fn get(&self, id: &str) -> Option<Entity> {\n        // ...\n    }\n}\n")),(0,i.kt)("p",null,"And an alternate DynamoDB implementation:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use aws_sdk_dynamodb::Client;\n\nstruct DynamoRepository {\n    client: Client,\n}\n\n#[async_trait]\nimpl Repository for DynamoRepository {\n    async fn get(&self, id: &str) -> Option<Entity> {\n        // ...\n    }\n}\n")),(0,i.kt)("h2",{id:"dependency-tags"},"Dependency Tags"),(0,i.kt)("p",null,"The injection framework can work directly with the ",(0,i.kt)("inlineCode",{parentName:"p"},"TypeId")," identifiers that are automatically generated by the ",(0,i.kt)("a",{parentName:"p",href:"https://doc.rust-lang.org/std/any/index.html"},"any")," package, but they often require you to pass in type parameters and repeat yourself more often, and they result in debug output that can be rather verbose:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"nakago::inject::container::test::entity::DynamoRepository was not found\n\nAvailable:\n - std::boxed::Box<dyn nakago::inject::container::test::entity::Repository>\n\n - nakago::inject::container::test::entity::PostgresRepository\n")),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Tags")," carry the underlying type around with them, meaning it can be inferred by the compiler in most cases. They also allow you to inject multiple instances of the same type, with different keys. If you have multiple Database Configs, for example, you can inject them into the container with separate tags even though they contain the same type."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'pub const POSTGRES_REPO: Tag<PostgresRepository> = Tag::new("entity::PostgresRepository");\npub const DYNAMO_REPO: Tag<DynamoRepository> = Tag::new("entity::DynamoRepository");\npub const REPO: Tag<Box<dyn Repository>> = Tag::new("entity::Repository");\n')),(0,i.kt)("p",null,"Instead of requesting the type explicitly like this:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let result = i.get_type::<PostgresRepository>()?;\n")),(0,i.kt)("p",null,"Tags are passed in and the type is inferred:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let result = i.get(&POSTGRES_REPO)?;\n")),(0,i.kt)("p",null,"Tags have a special String value that can be used instead of the full type name. This makes it easier to understand debug output, and this is what allows multiple versions of the same type to have different keys."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"Tag(DynamoEntityRepository) was not found\n\nAvailable:\n - Tag(EntityRepository)\n\n - Tag(PostgresEntityRepository)\n")),(0,i.kt)("h2",{id:"providing-dependencies"},"Providing Dependencies"),(0,i.kt)("p",null,"To provide a dependency, create a Provider that implements the ",(0,i.kt)("inlineCode",{parentName:"p"},"inject::Provider")," trait:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"use async_trait::async_trait;\nuse nakago::inject::{inject, Provider, Inject};\nuse sqlx::{Pool, Postgres};\n\n#[derive(Default)]\npub struct PostgresRepositoryProvider {}\n\n#[Provider]\n#[async_trait]\nimpl Provider<Box<dyn Repository>> for PostgresRepositoryProvider {\n    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Box<dyn Repository>>> {\n        let pool = i.get_type::<Pool<Postgres>>().await?;\n\n        Ok(Arc::new(Box::new(PostgresRepository::new(pool.clone()))))\n    }\n}\n")),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"PostgresRepositoryProvider")," struct is empty, and just exists so that we can implement the ",(0,i.kt)("inlineCode",{parentName:"p"},"Provider<T>")," trait. It uses ",(0,i.kt)("inlineCode",{parentName:"p"},"#[derive(Default)]")," because it doesn't need to initialize any config properties or context. It doesn't ",(0,i.kt)("em",{parentName:"p"},"have")," to be empty, though, and can carry information for the provider that is passed in on initialization and held until the Provider is invoked."),(0,i.kt)("p",null,"The result is wrapped in an ",(0,i.kt)("inlineCode",{parentName:"p"},"inject::Result")," so that an ",(0,i.kt)("inlineCode",{parentName:"p"},"Err")," can be returned to handle things like a failed ",(0,i.kt)("inlineCode",{parentName:"p"},"i.get()")," call or a failed database connection initialization."),(0,i.kt)("p",null,"In this particular case since ",(0,i.kt)("inlineCode",{parentName:"p"},"Pool<Postgres>")," is a known Sized type, it's safe to provide it without Boxing it to handle Unsized dynamic trait implementations. In many cases, however, you'll be working with ",(0,i.kt)("inlineCode",{parentName:"p"},"dyn Trait")," implementations so that you can swap between implementations easily. You'll want to make sure to box it up like ",(0,i.kt)("inlineCode",{parentName:"p"},"Box<dyn Trait>")," so that it can later be wrapped in the Shared Future and held across await points."),(0,i.kt)("p",null,"You don't need to worry about using a Tag with a Provider yet - that comes in the next step."),(0,i.kt)("h2",{id:"the-provider-macro"},"The Provider Macro"),(0,i.kt)("p",null,"You may have noticed the ",(0,i.kt)("inlineCode",{parentName:"p"},"#[Provider]")," macro above. This macro provides a companion implementation for the ",(0,i.kt)("inlineCode",{parentName:"p"},"impl Provider<Box<dyn Repository>>")," above that provides ",(0,i.kt)("inlineCode",{parentName:"p"},"impl Provider<Dependency>")," instead. This is so that your Provider that carries a specific type ",(0,i.kt)("inlineCode",{parentName:"p"},"T")," can also provide the generic version of that Dependency that is ",(0,i.kt)("inlineCode",{parentName:"p"},"dyn Any + Send + Sync")," that the container needs to keep it in the same ",(0,i.kt)("inlineCode",{parentName:"p"},"HashMap")," as all the other dependencies."),(0,i.kt)("p",null,"If you want to provide it manually instead, you can:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"#[async_trait]\nimpl Provider<Dependency> for PostgresRepositoryProvider {\n    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Dependency>> {\n        let provider = self as Arc<dyn Provider<Box<dyn Repository>>>;\n\n        Ok(provider.provide(i).await?)\n    }\n}\n")),(0,i.kt)("h2",{id:"the-inject-container"},"The Inject Container"),(0,i.kt)("p",null,"To make use of these Providers, create a dependency injection container instance:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let i = Inject::default();\n")),(0,i.kt)("p",null,"This is typically done at an entry point to your application, such as a ",(0,i.kt)("inlineCode",{parentName:"p"},"main.go")," file or within a unit or integration test setup routine."),(0,i.kt)("p",null,"Now, use ",(0,i.kt)("inlineCode",{parentName:"p"},"i.provide(...).await?")," to inject the Provider and associate it with a Tag:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"i.provide(&REPO, PostgresRepositoryProvider::default()).await?;\n")),(0,i.kt)("p",null,"I'll pause to point out here that if you had tried to use the ",(0,i.kt)("inlineCode",{parentName:"p"},"&POSTGRES_REPO")," Tag here, the compiler would report an error because the ",(0,i.kt)("inlineCode",{parentName:"p"},"PostgresRepositoryProvider")," above provides ",(0,i.kt)("inlineCode",{parentName:"p"},"Box<dyn Repository>"),", not ",(0,i.kt)("inlineCode",{parentName:"p"},"PostgresRepository"),"."),(0,i.kt)("h2",{id:"invoking-dependencies"},"Invoking Dependencies"),(0,i.kt)("p",null,"To pull dependencies out of the container, use ",(0,i.kt)("inlineCode",{parentName:"p"},"i.get(&TAG).await?")," or ",(0,i.kt)("inlineCode",{parentName:"p"},"i.get_type::<T>().await?"),". If a dependency isn't available, the container will return an ",(0,i.kt)("inlineCode",{parentName:"p"},"InjectError::NotFound")," result. This is often performed within a ",(0,i.kt)("inlineCode",{parentName:"p"},"provide")," function from the ",(0,i.kt)("inlineCode",{parentName:"p"},"Provider")," trait, but it is also used often at entry points to bootstrap an application."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let repo = i.get(&TAG).await?;\n")),(0,i.kt)("p",null,"You can use ",(0,i.kt)("inlineCode",{parentName:"p"},"i.get_opt(&TAG).await?")," to receive an Option rather than a Result."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let maybe_repo = i.get_opt(&TAG).await?;\n")),(0,i.kt)("h2",{id:"consuming-dependencies"},"Consuming Dependencies"),(0,i.kt)("p",null,"In some cases, such as with Config Loaders, a dependency is intended to be used up and made unavailable afterwards. This is often done within Lifecycle Hooks."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let loaders = i.consume(&CONFIG_LOADERS).await?;\n\nlet loader = Loader::<C>::new(loaders);\n")),(0,i.kt)("p",null,"In this example if any providers have been injected for the ",(0,i.kt)("inlineCode",{parentName:"p"},"&CONFIG_LOADERS")," tag they are requested, awaited, and then pulled out of the container and the tag is removed. If you try to ",(0,i.kt)("inlineCode",{parentName:"p"},"i.get()")," or ",(0,i.kt)("inlineCode",{parentName:"p"},"i.consume()")," the ",(0,i.kt)("inlineCode",{parentName:"p"},"&CONFIG_LOADERS")," tag again, you will receive the ",(0,i.kt)("inlineCode",{parentName:"p"},"InjectError::NotFound")," Error."),(0,i.kt)("h2",{id:"ejection"},"Ejection"),(0,i.kt)("p",null,"In certain contexts, such as testing, it's useful to drop the entire container except for a particular dependency - like a ",(0,i.kt)("inlineCode",{parentName:"p"},"MockDatabaseConnection")," used for validating expectations in unit testing."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},"let db = i.eject(&DATABASE_CONNECTION).await?;\n")),(0,i.kt)("p",null,"You can then perform the mutable operations you need for validating assertions in testing:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-rust"},'// Check the transaction log\nassert_eq!(\n    db.into_transaction_log(),\n    vec![Transaction::from_sql_and_values(\n        DatabaseBackend::Postgres,\n        r#"SELECT "episodes"."id", "episodes"."created_at", "episodes"."updated_at", "episodes"."title", "episodes"."summary", "episodes"."picture", "episodes"."show_id" FROM "episodes" WHERE "episodes"."id" = $1 LIMIT $2"#,\n        vec![episode.id.into(), 1u64.into()]\n    )]\n);\n')))}u.isMDXComponent=!0}}]);