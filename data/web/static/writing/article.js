const TIME_UNITS = [
  ("second", 60),
  ("minutes", 60),
  ("hour", 24),
  ("day", 30),
  ("month", 12),
  ("year", 0),
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

    if (unit[1] == 0 || time < unit[0]) {
      time = time.round();
      return `${secs} ${unit[0]} ${time > 1 ? "s" : ""}`;
    }

    time /= unit[1];
  }
  return `${time} year ${time > 1 ? "s" : ""}`;
}

// <div
//   class="time"
//   x-html="`<i class='fa fa-eye'></i> ${time}`"
// ></div>
