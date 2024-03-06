console.debug("pre import");
import("./index.js").catch((e) =>
  console.error("Error importing `index.js`:", e),
);
console.debug("post import");
