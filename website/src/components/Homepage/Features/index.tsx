import React from "react";
import clsx from "clsx";
import styles from "./styles.module.css";

type FeatureItem = {
    title: string;
    Svg: React.ComponentType<React.ComponentProps<"svg">>;
    description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
    {
        title: "Dependency Injection",
        Svg: require("@site/static/img/undraw_logistics_x-4-dc.svg").default,
        description: (
            <>
                Nakago is a fully async dependency injection system based on{" "}
                <a href="https://doc.rust-lang.org/std/any/index.html">Any</a>{" "}
                from the Rust standard library, and{" "}
                <a href="https://docs.rs/futures/latest/futures/future/struct.Shared.html">
                    Shared Futures
                </a>{" "}
                that allow multiple threads to await the same async dependency
                Provider.
            </>
        ),
    },
    {
        title: "Framework Agnostic",
        Svg: require("@site/static/img/undraw_my_answer_re_k4dv.svg").default,
        description: (
            <>
                Nakago already plays nicely with Axum and Warp, and can be
                smoothly integrated with other libraries or in-house frameworks.
                It focuses on making dependency injection useful and flexible,
                so that you can focus on scaling your application.
            </>
        ),
    },
    {
        title: "Comprehensive Testing",
        Svg: require("@site/static/img/undraw_software_engineer_re_tnjc.svg")
            .default,
        description: (
            <>
                Easily swap in mock dependencies for unit testing to isolate and
                exercise individual components. Put everything together and
                validate your solution with realistic integration tests that can
                be automatically executed in CI for each change.
            </>
        ),
    },
];

function Feature({ title, Svg, description }: FeatureItem) {
    return (
        <div className={clsx("col col--4")}>
            <div className="text--center">
                <Svg className={styles.featureSvg} role="img" />
            </div>
            <div className="text--center padding-horiz--md">
                <h3>{title}</h3>
                <p>{description}</p>
            </div>
        </div>
    );
}

export default function Features(): JSX.Element {
    return (
        <section className={styles.features}>
            <div className="container">
                <div className="row">
                    {FeatureList.map((props, idx) => (
                        <Feature key={idx} {...props} />
                    ))}
                </div>
            </div>
        </section>
    );
}
