const commit = await fetch(
  "https://api.github.com/repos/swc-project/swc/commits/main",
)
  .then((r) => r.json())
  .then((j) => j.sha);
const lib_rs = await fetch(
  `https://raw.githubusercontent.com/swc-project/swc/${commit}/crates/swc_ecma_visit/src/lib.rs`,
).then((r) => r.text());

const defines = lib_rs.split("define!({")[1];

const re =
  /pub (struct|enum) ([A-Za-z]+) \{((?:[\n\s]*(?:pub )?\w+[^\n]+?,)+)/g;
const items: { type: "struct" | "enum"; name: string; fieldNames: string[] }[] =
  [...defines.matchAll(re)].map((a) => ({
    type: a[1] as "struct" | "enum",
    name: a[2],
    fieldNames: a[3]
      .replaceAll(/pub |,$/g, "")
      .split(",")
      .map((m) =>
        m
          .trim()
          .split(/:/)[0]
          .replace(/\(.+?\)/, "(_)"),
      ),
  }));

function toSnakeCase(s: string) {
  return s
    .replaceAll(/[A-Z]/g, (m, idx, str) =>
      /[a-z]/.test(str[idx - 1] ?? "")
        ? `_${m}`
        : /[A-Z][a-z]/.test(str[idx - 1] + str[idx + 1])
          ? `_${m}`
          : m,
    )
    .toLowerCase();
}

let tag = 0;

console.log(
  `/* auto-generated with \`bun scripts/generate.ts > src/detector/visit.rs\`, swc @ ${commit} */\n`,
);
console.log(`impl swc_ecma_visit::Visit for JsHasher {`);
for (const item of items) {
  // header
  console.log(
    `    fn visit_${toSnakeCase(item.name)}(&mut self, n: &swc_ecma_ast::${item.name}) {`,
  );
  console.log(`        self.hasher.write(b"${item.name}");`);

  if (item.type === "enum") {
    console.log(`        match n {`);
    console.log(
      "           ",
      item.fieldNames
        .map(
          (n) =>
            `swc_ecma_ast::${item.name}::${n} => self.hasher.write(b"${n}"),`,
        )
        .join("\n            "),
    );
    console.log(`        }`);
  }

  // close
  console.log(
    "        swc_ecma_visit::VisitWith::visit_children_with(n, self);\n    }\n",
  );
}
console.log("}");
