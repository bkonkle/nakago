import React from "react";
import styles from "./styles.module.css";

export default function Etymoology(): JSX.Element {
    return (
        <section className={styles.etymology}>
            <div className="container">
                <div className="row">
                    <div className="col col--2"></div>
                    <div className="col col--8 text--center">
                        <h3>Etymology</h3>
                        <p>
                            Nakago (中子) is a Japanese word meaning "core", or
                            less commonly the "middle of a nest of boxes". It
                            often refers to the{" "}
                            <a href="https://en.wikipedia.org/wiki/Tang_(tools)">
                                tang
                            </a>{" "}
                            of a Japanese katana - the foundation of the hilt
                            and the mechanism through which a sword is wielded.
                            The nakago must be sound and resilient, allowing the
                            holder to guide the blade with confidence.
                        </p>
                    </div>
                    <div className="col col--2"></div>
                </div>
            </div>
        </section>
    );
}
