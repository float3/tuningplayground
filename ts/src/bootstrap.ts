console.log("pre import");
import("./index.js").catch((e) =>
  console.error("Error importing `index.js`:", e),
);
console.log("post import");