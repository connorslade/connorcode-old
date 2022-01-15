const TIME_UNITS = [
  ["s", 60],
  ["m", 60],
  ["h", 24],
];

function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;
  let point = "/writing/";

  path.split("/").forEach((item) => {
    point += `${item}/`;
    out += ` <i class='fa fa-angle-right'></i> <a href="${point}">${item}</a>`;
  });

  return out;
}

function loadTime(time) {
  for (let i = 0; i < TIME_UNITS.length; i++) {
    let unit = TIME_UNITS[i];
    if (unit[1] == 0 || time < unit[1]) return `${Math.round(time)} ${unit[0]}`;
    
    time /= unit[1];
  }
  return `${Math.round(time)} year`;
}
