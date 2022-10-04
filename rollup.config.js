import rust from "@wasm-tool/rollup-plugin-rust";

export default [
  {
    input: "src/plain.js",
    output: {
      dir: "dist/js",
      sourcemap: true,
    },
  },
  {
    input: {
      wasm: "client/Cargo.toml",
    },
    output: {
      dir: "dist/js",
      format: "iife",
      sourcemap: true,
    },
    plugins: [
      rust({
        serverPath: "/js/",
      }),
    ],
  },
];
