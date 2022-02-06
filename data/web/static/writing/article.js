const TIME_UNITS = [
  ["s", 60],
  ["m", 60],
  ["h", 24],
];

let isLiked = false;
let doc = "";

function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;
  let point = "/writing";

  let pathParts = path.split("/");
  pathParts.forEach((item) => {
    point += `/${item}`;
    out += ` <i class='fa fa-angle-right'></i> <a href="${point}">${item}</a>`;
  });

  return out;
}

function loadTime(time) {
  for (let i = 0; i < TIME_UNITS.length; i++) {
    let unit = TIME_UNITS[i];
    if (time < unit[1]) return `${Math.round(time)}${unit[0]}`;

    time /= unit[1];
  }
  return `${Math.round(time)}h`;
}

function setLikes(likes, ogLiked) {
  isLiked = ogLiked;
  let likeButton = document.querySelector(".like");
  if (likes == null) return;

  updateLikesUI(likes, ogLiked);
  likeButton.addEventListener("click", () => {
    isLiked = !isLiked;
    updateLikesUI(likes, ogLiked);
    updateLikesApi(isLiked);
  });
}

function updateLikesUI(likes, ogLiked) {
  let likeContent = document.querySelector(".like-content");

  if (isLiked && !ogLiked) likes = likes + 1;
  if (!isLiked && ogLiked) likes = likes - 1;

  if (isLiked)
    likeContent.innerHTML = `<i class="icon fa fa-heart"></i> <p>${likes}</p>`;
  else
    likeContent.innerHTML = `<i class="icon fa fa-heart-o"></i> <p>${likes}</p>`;
}

function updateLikesApi(isLiked) {
  let doc = window.location.pathname.split("/writing/");
  doc.shift();
  doc = doc.join("/writing/");

  fetch("/api/writing/like", {
    method: "POST",
    body: JSON.stringify({ doc, like: isLiked }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}
