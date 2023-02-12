let last;

particlesJS.load("body", "/christmas/particles.json");
update();
setInterval(update, 10);

function update() {
  let now = new Date();
  let cmas = new Date(now.getFullYear(), 11, 25);

  if (now.getMonth() === 11 && now.getDate() > 25)
    cmas.setFullYear(cmas.getFullYear() + 1);

  let out = secondsToDhms(Math.ceil(cmas.getTime() - now.getTime()) / 1000);

  if (now.getMonth() === 11 && now.getDate() === 25)
    out = "❄ TODAY ❄";

  if (out === last) return;

  document.querySelector("#countdown").innerHTML = out;
  last = out;
}

function secondsToDhms(seconds) {
  seconds = Number(seconds);
  let d = Math.floor(seconds / (3600 * 24))
    .toString()
    .padStart(2, "0");
  let h = Math.floor((seconds % (3600 * 24)) / 3600)
    .toString()
    .padStart(2, "0");
  let m = Math.floor((seconds % 3600) / 60)
    .toString()
    .padStart(2, "0");
  let s = Math.floor(seconds % 60)
    .toString()
    .padStart(2, "0");

  return `${d} Days<br>${h} Hors<br>${m} Mins<br>${s} Secs`;
}
