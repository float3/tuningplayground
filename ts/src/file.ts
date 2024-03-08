let textFile: string | null = null;

const makeTextFile = (text: string): string => {
  const data: Blob = new Blob([text], { type: "text/plain" });

  if (textFile !== null) {
    window.URL.revokeObjectURL(textFile);
  }

  textFile = window.URL.createObjectURL(data);

  return textFile;
};

export const downloadFile = (name: string, contents: string): void => {
  const a: HTMLAnchorElement = document.createElement("a");
  a.style.display = "none";
  a.href = makeTextFile(contents);
  a.download = name;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a); // Optionally, remove the element after click
};
