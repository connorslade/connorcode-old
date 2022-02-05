const TIME_UNITS = [
  ["s", 60],
  ["m", 60],
  ["h", 24],
];

let isLiked = false;

function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;
  let point = "/writing";

  path.split("/").forEach((item) => {
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

function setLikes(likes, liked) {
  isLiked = liked;
  let likeButton = document.querySelector("[like-button]");
  let likeContent = document.querySelector(".like-content");

  updateLikes(likes, isLiked);
  likeButton.addEventListener("click", () => {
    isLiked = !isLiked;
    updateLikes(likes, isLiked);
  });
}

function updateLikes(likes, isLiked) {
  let likeContent = document.querySelector(".like-content");
  isLiked = !isLiked;

  if (isLiked)
    likeContent.innerHTML = `<i class="icon fa fa-heart-o" style="font-weight: bold;"></i> <p>${likes}</p>`;
  else
    likeContent.innerHTML = `<i class="icon fa fa-heart" style="font-weight: bold;"></i> <p>${
      likes + 1
    }</p>`;
}
