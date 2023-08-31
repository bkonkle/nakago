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
        Svg: require("@site/static/img/undraw_my_answer_re_k4dv.svg").default,
        description: (
            <>
                Nakago is built around a fully async dependency injection system
                based on{" "}
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
        title: "Application Lifecycle",
        Svg: require("@site/static/img/undraw_software_engineer_re_tnjc.svg")
            .default,
        description: (
            <>
                Nakago provides a simple and flexible application lifecycle that
                defines events that you can react to with Hooks. Load
                dependencies and config, initialize services, and start your
                application with the context you need for each entry point.
            </>
        ),
    },
    {
        title: "Comprehensive Testing",
        Svg: require("@site/static/img/undraw_result_re_uj08.svg").default,
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
