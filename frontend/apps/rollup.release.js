import rust from "@wasm-tool/rollup-plugin-rust";
import nodeResolve from "@rollup/plugin-node-resolve";
import injectProcessEnv from 'rollup-plugin-inject-process-env';
import {getEnv} from "./rollup.common.js";

const {URL_FRONTEND_RELEASE} = require("../config/constants");

let {APP_NAME} = process.env;

if(!APP_NAME) {
    console.error("INVALID APP_NAME!");
    process.exit(1);
}

export default {
    input: {
        index: `./crates/entry/${APP_NAME}/Cargo.toml`,
    },
    output: {
        dir: `./dist/${APP_NAME}/js/`,
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: `${URL_FRONTEND_RELEASE}/${APP_NAME}/js/`,
			cargoArgs: ["--features", "release"],
            debug: false,
        }),
		nodeResolve(),
        injectProcessEnv(getEnv()),
    ],
};
