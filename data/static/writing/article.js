function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;
  let point = "/writing/";

  path.split("/").forEach((item) => {
    point += `${item}/`;
    out += ` <i class='fa fa-angle-right'></i> <a href="${point}">${item}</a>`;
  });

  return out;
}
