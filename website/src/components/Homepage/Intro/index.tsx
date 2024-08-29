import React from "react";
import styles from "./styles.module.css";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";

export default function Intro(): JSX.Element {
    const { siteConfig } = useDocusaurusContext();
    return (
        <section className={styles.intro}>
            <div className="container">
                <div className="row">
                    <div className="col col--4"></div>
                    <div className="col col--4 text--center">
                        <img
                            src={`${siteConfig.baseUrl}img/katana.png`}
                            alt="Nakago Logo"
                        />
                    </div>
                    <div className="col col--4"></div>
                </div>
                <div className="row">
                    <div className="col col--2"></div>
                    <div className="col col--8 text--center">
                        <h3>A Flexible Foundation</h3>
                        <p>
                            Nakago provides a flexible foundation for building
                            precise and performant services in Rust. It uses
                            dependency injectoin to complement existing tools in
                            the <a href="https://tokio.rs/">Tokio</a> ecosystem
                            and beyond, making it easy to bring powerful
                            libraries like{" "}
                            <a href="https://github.com/tokio-rs/axum">Axum</a>,{" "}
                            <a href="https://www.sea-ql.org/SeaORM/">SeaORM</a>,
                            and{" "}
                            <a href="https://async-graphql.github.io/async-graphql/en/index.html">
                                Async-GraphQL
                            </a>{" "}
                            together in an easily testable and configurable way.
                        </p>
                    </div>
                    <div className="col col--2"></div>
                </div>
                <div className="row">
                    <div className="col col--2"></div>
                    <div className="col col--8 text--center">
                        <p>
                            <a
                                href="https://crates.io/crates/nakago"
                                rel="nofollow"
                            >
                                <img
                                    src="https://img.shields.io/crates/v/nakago.svg"
                                    alt="Crates.io"
                                />
                            </a>{" "}
                            <a href="https://docs.rs/nakago" rel="nofollow">
                                <img
                                    src="https://docs.rs/nakago/badge.svg"
                                    alt="Docs.rs"
                                />
                            </a>{" "}
                            <a
                                href="https://github.com/bkonkle/nakago/actions"
                                rel="nofollow"
                            >
                                <img
                                    src="https://github.com/bkonkle/nakago/workflows/CI/badge.svg"
                                    alt="CI"
                                />
                            </a>{" "}
                            <a
                                href="https://codecov.io/gh/bkonkle/nakago"
                                rel="nofollow"
                            >
                                <img
                                    src="https://codecov.io/gh/bkonkle/nakago/branch/main/graph/badge.svg?token=BXEZAMHVLP"
                                    alt="Coverage Status"
                                />
                            </a>{" "}
                            <a href="https://www.rust-lang.org" rel="nofollow">
                                <img
                                    src="https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat"
                                    alt="Rust"
                                />
                            </a>{" "}
                            <a href="https://tokio.rs" rel="nofollow">
                                <img
                                    src="https://img.shields.io/badge/tokio-463103?logo=rust&style=flat"
                                    alt="Tokio"
                                />
                            </a>{" "}
                            <a
                                href="https://crates.io/crates/axum"
                                rel="nofollow"
                            >
                                <img
                                    src="https://img.shields.io/badge/axum-7b5312?logo=rust&style=flat"
                                    alt="Axum"
                                />
                            </a>
                        </p>
                    </div>
                    <div className="col col--2"></div>
                </div>
            </div>
        </section>
    );
}
